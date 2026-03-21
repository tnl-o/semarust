//! Integration Extract Value - операции с IntegrationExtractValue
//!
//! Аналог db/sql/integration.go из Go версии (часть 3: IntegrationExtractValue)

use crate::db::sql::types::SqlDb;
use crate::error::{Error, Result};
use crate::models::*;
use sqlx::Row;

impl SqlDb {
    /// Получает все extract values для интеграции
    pub async fn get_integration_extract_values(&self, project_id: i32, integration_id: i32) -> Result<Vec<IntegrationExtractValue>> {
        match unreachable!() {
            
        }
    }
    
    /// Создаёт IntegrationExtractValue
    pub async fn create_integration_extract_value(&self, mut value: IntegrationExtractValue) -> Result<IntegrationExtractValue> {
        match unreachable!() {
            
        }
    }
    
    /// Обновляет IntegrationExtractValue
    pub async fn update_integration_extract_value(&self, value: IntegrationExtractValue) -> Result<()> {
        match unreachable!() {
            
        }
    }
    
    /// Удаляет IntegrationExtractValue
    pub async fn delete_integration_extract_value(&self, project_id: i32, integration_id: i32, value_id: i32) -> Result<()> {
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

        // Создаём таблицу integration_extract_value
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS integration_extract_value (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                integration_id INTEGER NOT NULL,
                project_id INTEGER NOT NULL,
                name TEXT NOT NULL DEFAULT '',
                value_source TEXT NOT NULL,
                body_data_type TEXT NOT NULL DEFAULT 'json',
                key TEXT,
                variable TEXT,
                value_name TEXT NOT NULL,
                value_type TEXT NOT NULL
            )"
        )
        .execute(db.get_sqlite_pool().unwrap())
        .await
        .unwrap();

        TestDb { db, _temp: temp }
    }

    #[tokio::test]
    async fn test_create_and_get_integration_extract_value() {
        let TestDb { db, _temp } = create_test_db().await;
        
        let value = IntegrationExtractValue {
            id: 0,
            integration_id: 1,
            project_id: 1,
            name: "Test Value".to_string(),
            value_source: "body".to_string(),
            body_data_type: "json".to_string(),
            key: None,
            variable: None,
            value_name: "task_id".to_string(),
            value_type: "json".to_string(),
        };
        
        let created = db.create_integration_extract_value(value.clone()).await.unwrap();
        assert!(created.id > 0);
        
        let values = db.get_integration_extract_values(1, 1).await.unwrap();
        assert!(values.len() >= 1);
        assert_eq!(values[0].value_name, "task_id");
        
        // Cleanup
        let _ = db.close().await;
    }

    #[tokio::test]
    async fn test_update_integration_extract_value() {
        let TestDb { db, _temp } = create_test_db().await;
        
        let value = IntegrationExtractValue {
            id: 0,
            integration_id: 1,
            project_id: 1,
            name: "Test Value".to_string(),
            value_source: "body".to_string(),
            body_data_type: "json".to_string(),
            key: None,
            variable: None,
            value_name: "task_id".to_string(),
            value_type: "json".to_string(),
        };
        
        let created = db.create_integration_extract_value(value).await.unwrap();
        
        let mut updated = created.clone();
        updated.value_name = "job_id".to_string();
        
        db.update_integration_extract_value(updated).await.unwrap();
        
        let values = db.get_integration_extract_values(1, 1).await.unwrap();
        assert_eq!(values[0].value_name, "job_id");
        
        // Cleanup
        let _ = db.close().await;
    }

    #[tokio::test]
    async fn test_delete_integration_extract_value() {
        let TestDb { db, _temp } = create_test_db().await;
        
        let value = IntegrationExtractValue {
            id: 0,
            integration_id: 1,
            project_id: 1,
            name: "Test Value".to_string(),
            value_source: "body".to_string(),
            body_data_type: "json".to_string(),
            key: None,
            variable: None,
            value_name: "task_id".to_string(),
            value_type: "json".to_string(),
        };
        
        let created = db.create_integration_extract_value(value).await.unwrap();
        
        db.delete_integration_extract_value(1, 1, created.id).await.unwrap();
        
        let values = db.get_integration_extract_values(1, 1).await.unwrap();
        assert!(values.is_empty());
        
        // Cleanup
        let _ = db.close().await;
    }
}
