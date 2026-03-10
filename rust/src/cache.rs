//! Redis Cache Module - Модуль кэширования на базе Redis
//!
//! Этот модуль предоставляет:
//! - Кэширование запросов к БД
//! - Кэширование сессий пользователей
//! - Инвалидацию кэша при изменениях
//! - Метрики hit/miss

use std::sync::Arc;
use std::time::Duration;
use redis::{Client, ConnectionLike, AsyncCommands, RedisResult, Value};
use redis::aio::ConnectionManager;
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};
use serde::{Serialize, Deserialize};
use crate::error::{Error, Result};

/// Конфигурация Redis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    /// URL подключения к Redis
    pub url: String,
    /// Префикс для всех ключей
    pub key_prefix: String,
    /// TTL по умолчанию (в секундах)
    pub default_ttl_secs: u64,
    /// Максимальное количество попыток подключения
    pub max_retries: u32,
    /// Таймаут подключения (в секундах)
    pub connection_timeout_secs: u64,
    /// Включить кэширование
    pub enabled: bool,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://localhost:6379".to_string(),
            key_prefix: "semaphore:".to_string(),
            default_ttl_secs: 300, // 5 минут
            max_retries: 3,
            connection_timeout_secs: 5,
            enabled: false,
        }
    }
}

/// Статистика кэша
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Количество попаданий в кэш
    pub hits: u64,
    /// Количество промахов кэша
    pub misses: u64,
    /// Количество ошибок
    pub errors: u64,
    /// Количество установленных соединений
    pub connections: u64,
}

impl CacheStats {
    /// Процент попаданий
    pub fn hit_ratio(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64 * 100.0
        }
    }

    /// Общее количество запросов
    pub fn total_requests(&self) -> u64 {
        self.hits + self.misses
    }
}

/// Redis клиент с управлением соединением
pub struct RedisCache {
    config: RedisConfig,
    client: Option<Client>,
    connection: Arc<RwLock<Option<ConnectionManager>>>,
    stats: Arc<RwLock<CacheStats>>,
}

impl RedisCache {
    /// Создаёт новый Redis кэш
    pub fn new(config: RedisConfig) -> Self {
        Self {
            config: config.clone(),
            client: None,
            connection: Arc::new(RwLock::new(None)),
            stats: Arc::new(RwLock::new(CacheStats::default())),
        }
    }

    /// Инициализирует соединение с Redis
    pub async fn initialize(&mut self) -> Result<()> {
        if !self.config.enabled {
            info!("Redis cache is disabled");
            return Ok(());
        }

        info!("Connecting to Redis at {}", self.config.url);

        let client = Client::open(self.config.url.as_str())
            .map_err(|e| Error::Other(format!("Failed to create Redis client: {}", e)))?;

        // Пробуем подключиться с повторными попытками
        let mut last_error = None;
        for attempt in 1..=self.config.max_retries {
            match client.get_connection_manager().await {
                Ok(conn) => {
                    self.client = Some(client);
                    *self.connection.write().await = Some(conn);
                    
                    let mut stats = self.stats.write().await;
                    stats.connections = 1;
                    
                    info!("Successfully connected to Redis");
                    return Ok(());
                }
                Err(e) => {
                    warn!("Redis connection attempt {} failed: {}", attempt, e);
                    last_error = Some(e);
                    
                    if attempt < self.config.max_retries {
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                }
            }
        }

        let err_msg = format!("Failed to connect to Redis after {} attempts", self.config.max_retries);
        if let Some(e) = last_error {
            error!("{}: {}", err_msg, e);
        }
        
        Err(Error::Other(err_msg))
    }

    /// Проверяет доступность Redis
    pub async fn ping(&self) -> bool {
        let mut conn_guard = self.connection.write().await;
        if let Some(conn) = conn_guard.as_mut() {
            let result: RedisResult<String> = redis::cmd("PING").query_async(conn).await;
            result.is_ok()
        } else {
            false
        }
    }

    /// Получает значение из кэша
    pub async fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        if !self.config.enabled {
            return Ok(None);
        }

        let full_key = self.make_key(key);
        let mut conn_guard = self.connection.write().await;
        
        let conn = match conn_guard.as_mut() {
            Some(c) => c,
            None => {
                let mut stats = self.stats.write().await;
                stats.errors += 1;
                return Ok(None);
            }
        };

        let result: RedisResult<String> = conn.get(&full_key).await;
        
        match result {
            Ok(value_str) => {
                let mut stats = self.stats.write().await;
                stats.hits += 1;
                
                match serde_json::from_str::<T>(&value_str) {
                    Ok(v) => Ok(Some(v)),
                    Err(e) => {
                        warn!("Failed to deserialize cached value for key {}: {}", key, e);
                        Ok(None)
                    }
                }
            }
            Err(e) => {
                let mut stats = self.stats.write().await;
                stats.misses += 1;
                
                if e.is_unrecoverable_error() {
                    error!("Redis get error for key {}: {}", key, e);
                }
                
                Ok(None)
            }
        }
    }

    /// Устанавливает значение в кэш с TTL по умолчанию
    pub async fn set<T: Serialize>(&self, key: &str, value: &T) -> Result<()> {
        self.set_with_ttl(key, value, self.config.default_ttl_secs).await
    }

    /// Устанавливает значение в кэш с указанным TTL
    pub async fn set_with_ttl<T: Serialize>(&self, key: &str, value: &T, ttl_secs: u64) -> Result<()> {
        if !self.config.enabled {
            return Ok(());
        }

        let full_key = self.make_key(key);
        let mut conn_guard = self.connection.write().await;
        
        let conn = match conn_guard.as_mut() {
            Some(c) => c,
            None => {
                let mut stats = self.stats.write().await;
                stats.errors += 1;
                return Ok(());
            }
        };

        let serialized = serde_json::to_string(value)
            .map_err(|e| Error::Other(format!("Failed to serialize value: {}", e)))?;

        let result: RedisResult<()> = conn.set_ex(&full_key, &serialized, ttl_secs).await;
        
        match result {
            Ok(_) => {
                debug!("Cached key {} with TTL {}s", key, ttl_secs);
                Ok(())
            }
            Err(e) => {
                let mut stats = self.stats.write().await;
                stats.errors += 1;
                error!("Redis set error for key {}: {}", key, e);
                Err(Error::Other(format!("Redis set error: {}", e)))
            }
        }
    }

    /// Удаляет значение из кэша
    pub async fn delete(&self, key: &str) -> Result<()> {
        if !self.config.enabled {
            return Ok(());
        }

        let full_key = self.make_key(key);
        let mut conn_guard = self.connection.write().await;
        
        let conn = match conn_guard.as_mut() {
            Some(c) => c,
            None => return Ok(()),
        };

        let result: RedisResult<()> = conn.del(&full_key).await;
        
        match result {
            Ok(_) => {
                debug!("Deleted key {}", key);
                Ok(())
            }
            Err(e) => {
                let mut stats = self.stats.write().await;
                stats.errors += 1;
                warn!("Redis delete error for key {}: {}", key, e);
                Ok(()) // Не считаем ошибкой удаление несуществующего ключа
            }
        }
    }

    /// Удаляет несколько ключей по паттерну
    pub async fn delete_pattern(&self, pattern: &str) -> Result<()> {
        if !self.config.enabled {
            return Ok(());
        }

        let full_pattern = format!("{}{}", self.config.key_prefix, pattern);
        let mut conn_guard = self.connection.write().await;
        
        let conn = match conn_guard.as_mut() {
            Some(c) => c,
            None => return Ok(()),
        };

        let keys: Vec<String> = conn.keys(&full_pattern).await.unwrap_or_default();
        
        for key in &keys {
            let _: () = conn.del(key).await.unwrap_or(());
        }
        
        debug!("Deleted {} keys by pattern {}", keys.len(), pattern);
        Ok(())
    }

    /// Проверяет существует ли ключ
    pub async fn exists(&self, key: &str) -> Result<bool> {
        if !self.config.enabled {
            return Ok(false);
        }

        let full_key = self.make_key(key);
        let mut conn_guard = self.connection.write().await;
        
        let conn = match conn_guard.as_mut() {
            Some(c) => c,
            None => return Ok(false),
        };

        let result: RedisResult<bool> = conn.exists(&full_key).await;
        result.map_err(|e| Error::Other(format!("Redis exists error: {}", e)))
    }

    /// Увеличивает значение счётчика
    pub async fn increment(&self, key: &str) -> Result<u64> {
        if !self.config.enabled {
            return Ok(0);
        }

        let full_key = self.make_key(key);
        let mut conn_guard = self.connection.write().await;
        
        let conn = match conn_guard.as_mut() {
            Some(c) => c,
            None => return Ok(0),
        };

        let result: RedisResult<u64> = conn.incr(&full_key, 1).await;
        result.map_err(|e| Error::Other(format!("Redis increment error: {}", e)))
    }

    /// Получает статистику кэша
    pub async fn get_stats(&self) -> CacheStats {
        self.stats.read().await.clone()
    }

    /// Сбрасывает статистику
    pub async fn reset_stats(&self) {
        let mut stats = self.stats.write().await;
        *stats = CacheStats::default();
    }

    /// Формирует полный ключ с префиксом
    fn make_key(&self, key: &str) -> String {
        format!("{}{}", self.config.key_prefix, key)
    }

    /// Проверяет включён ли кэш
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }
}

/// Helper для получения строкового представления ключа
pub fn cache_key(parts: &[&str]) -> String {
    parts.join(":")
}

/// Макрос для кэширования результатов функции
#[macro_export]
macro_rules! cache_result {
    ($cache:expr, $key:expr, $ttl:expr, $block:block) => {
        async {
            if let Some(cached) = $cache.get::<_>(&$key).await? {
                return Ok(cached);
            }
            
            let result = $block;
            
            if let Ok(ref value) = result {
                $cache.set_with_ttl(&$key, value, $ttl).await?;
            }
            
            result
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_key() {
        assert_eq!(cache_key(&["user", "123"]), "user:123");
        assert_eq!(cache_key(&["project", "456", "tasks"]), "project:456:tasks");
    }

    #[test]
    fn test_cache_stats() {
        let mut stats = CacheStats::default();
        stats.hits = 80;
        stats.misses = 20;
        
        assert_eq!(stats.hit_ratio(), 80.0);
        assert_eq!(stats.total_requests(), 100);
    }
}
