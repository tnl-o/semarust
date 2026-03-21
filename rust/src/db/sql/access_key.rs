//! Access Key CRUD Operations
//!
//! Операции с ключами доступа в SQL

use crate::db::sql::types::SqlDb;
use crate::error::{Error, Result};
use crate::models::AccessKey;

impl SqlDb {
    /// Получает ключи доступа проекта
    pub async fn get_access_keys(&self, project_id: i32) -> Result<Vec<AccessKey>> {
        match unreachable!() {
            
        }
    }

    /// Получает ключ доступа по ID
    pub async fn get_access_key(&self, project_id: i32, key_id: i32) -> Result<AccessKey> {
        match unreachable!() {
            
        }
    }

    /// Создаёт ключ доступа
    pub async fn create_access_key(&self, mut key: AccessKey) -> Result<AccessKey> {
        match unreachable!() {
            
        }
    }

    /// Обновляет ключ доступа
    pub async fn update_access_key(&self, key: AccessKey) -> Result<()> {
        match unreachable!() {
            
        }
    }

    /// Удаляет ключ доступа
    pub async fn delete_access_key(&self, project_id: i32, key_id: i32) -> Result<()> {
        match unreachable!() {
            
        }
    }
}
