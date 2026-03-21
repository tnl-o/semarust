//! Integration Matcher - операции с IntegrationMatcher
//!
//! Аналог db/sql/integration.go из Go версии (часть 2: IntegrationMatcher)

use crate::db::sql::types::SqlDb;
use crate::error::{Error, Result};
use crate::models::*;
use sqlx::Row;

impl SqlDb {
    /// Получает все matcher'ы для интеграции
    pub async fn get_integration_matchers(&self, project_id: i32, integration_id: i32) -> Result<Vec<IntegrationMatcher>> {
        match unreachable!() {
            
        }
    }
    
    /// Создаёт IntegrationMatcher
    pub async fn create_integration_matcher(&self, mut matcher: IntegrationMatcher) -> Result<IntegrationMatcher> {
        match unreachable!() {
            
        }
    }
    
    /// Обновляет IntegrationMatcher
    pub async fn update_integration_matcher(&self, matcher: IntegrationMatcher) -> Result<()> {
        match unreachable!() {
            
        }
    }
    
    /// Удаляет IntegrationMatcher
    pub async fn delete_integration_matcher(&self, project_id: i32, integration_id: i32, matcher_id: i32) -> Result<()> {
        match unreachable!() {
            
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestDb {
        db: SqlDb,
        _temp: tempfile::NamedTempFile,
    }

    async fn create_test_db() -> TestDb {
        let (db_path, temp) = crate::db::sql::init::test_sqlite_url();

        let db = SqlDb::connect_sqlite(&db_path).await.unwrap();

        // Создаём таблицу integration_matcher
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS integration_matcher (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                integration_id INTEGER NOT NULL,
                project_id INTEGER NOT NULL,
                name TEXT NOT NULL DEFAULT '',
                body_data_type TEXT NOT NULL DEFAULT 'json',
                key TEXT,
                matcher_type TEXT NOT NULL,
                matcher_value TEXT NOT NULL,
                method TEXT NOT NULL DEFAULT 'GET'
            )"
        )
        .execute(db.get_sqlite_pool().unwrap())
        .await
        .unwrap();

        TestDb { db, _temp: temp }
    }

    #[tokio::test]
    async fn test_create_and_get_integration_matcher() {
        let TestDb { db, _temp } = create_test_db().await;
        
        let matcher = IntegrationMatcher {
            id: 0,
            integration_id: 1,
            project_id: 1,
            name: "Test Matcher".to_string(),
            body_data_type: "json".to_string(),
            key: None,
            matcher_type: "header".to_string(),
            matcher_value: "Content-Type".to_string(),
            method: "GET".to_string(),
        };
        
        let created = db.create_integration_matcher(matcher.clone()).await.unwrap();
        assert!(created.id > 0);
        
        let matchers = db.get_integration_matchers(1, 1).await.unwrap();
        assert!(matchers.len() >= 1);
        assert_eq!(matchers[0].matcher_type, "header");
        
        // Cleanup
        let _ = db.close().await;
    }

    #[tokio::test]
    async fn test_update_integration_matcher() {
        let TestDb { db, _temp } = create_test_db().await;
        
        let matcher = IntegrationMatcher {
            id: 0,
            integration_id: 1,
            project_id: 1,
            name: "Test Matcher".to_string(),
            body_data_type: "json".to_string(),
            key: None,
            matcher_type: "header".to_string(),
            matcher_value: "Content-Type".to_string(),
            method: "GET".to_string(),
        };
        
        let created = db.create_integration_matcher(matcher).await.unwrap();
        
        let mut updated = created.clone();
        updated.matcher_value = "Authorization".to_string();
        
        db.update_integration_matcher(updated).await.unwrap();
        
        let matchers = db.get_integration_matchers(1, 1).await.unwrap();
        assert_eq!(matchers[0].matcher_value, "Authorization");
        
        // Cleanup
        let _ = db.close().await;
    }

    #[tokio::test]
    async fn test_delete_integration_matcher() {
        let TestDb { db, _temp } = create_test_db().await;
        
        let matcher = IntegrationMatcher {
            id: 0,
            integration_id: 1,
            project_id: 1,
            name: "Test Matcher".to_string(),
            body_data_type: "json".to_string(),
            key: None,
            matcher_type: "header".to_string(),
            matcher_value: "Content-Type".to_string(),
            method: "GET".to_string(),
        };
        
        let created = db.create_integration_matcher(matcher).await.unwrap();
        
        db.delete_integration_matcher(1, 1, created.id).await.unwrap();
        
        let matchers = db.get_integration_matchers(1, 1).await.unwrap();
        assert!(matchers.is_empty());
        
        // Cleanup
        let _ = db.close().await;
    }
}
