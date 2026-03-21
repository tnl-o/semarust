//! Integration CRUD - операции с интеграциями
//!
//! Аналог db/sql/integration.go из Go версии (часть 1: CRUD)

use crate::db::sql::types::SqlDb;
use crate::error::{Error, Result};
use crate::models::*;
use sqlx::Row;

impl SqlDb {
    /// Получает все интеграции проекта
    pub async fn get_integrations(&self, project_id: i32) -> Result<Vec<Integration>> {
        match unreachable!() {
            
        }
    }
    
    /// Получает интеграцию по ID
    pub async fn get_integration(&self, project_id: i32, integration_id: i32) -> Result<Integration> {
        match unreachable!() {
            
        }
    }
    
    /// Создаёт новую интеграцию
    pub async fn create_integration(&self, mut integration: Integration) -> Result<Integration> {
        match unreachable!() {
            
        }
    }

    /// Обновляет интеграцию
    pub async fn update_integration(&self, integration: Integration) -> Result<()> {
        match unreachable!() {
            
        }
    }
    
    /// Удаляет интеграцию
    pub async fn delete_integration(&self, project_id: i32, integration_id: i32) -> Result<()> {
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

        // Создаём таблицу integration
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS integration (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                template_id INTEGER,
                auth_method TEXT NOT NULL DEFAULT 'none',
                auth_header TEXT,
                auth_secret_id INTEGER
            )"
        )
        .execute(db.get_sqlite_pool().unwrap())
        .await
        .unwrap();

        TestDb { db, _temp: temp }
    }

    #[tokio::test]
    async fn test_create_and_get_integration() {
        let TestDb { db, _temp } = create_test_db().await;
        
        let integration = Integration {
            id: 0,
            project_id: 1,
            name: "Test Integration".to_string(),
            template_id: 1,
            auth_method: "none".to_string(),
            auth_header: None,
            auth_secret_id: None,
        };
        
        let created = db.create_integration(integration.clone()).await.unwrap();
        assert!(created.id > 0);
        
        let retrieved = db.get_integration(1, created.id).await.unwrap();
        assert_eq!(retrieved.name, "Test Integration");
        
        // Cleanup
        let _ = db.close().await;
    }

    #[tokio::test]
    async fn test_get_integrations() {
        let TestDb { db, _temp } = create_test_db().await;
        
        // Создаём несколько интеграций
        for i in 0..5 {
            let integration = Integration {
                id: 0,
                project_id: 1,
                name: format!("Integration {}", i),
                template_id: 1,
                auth_method: "none".to_string(),
                auth_header: None,
                auth_secret_id: None,
            };
            db.create_integration(integration).await.unwrap();
        }
        
        let integrations = db.get_integrations(1).await.unwrap();
        assert!(integrations.len() >= 5);
        
        // Cleanup
        let _ = db.close().await;
    }

    #[tokio::test]
    async fn test_update_integration() {
        let TestDb { db, _temp } = create_test_db().await;
        
        let integration = Integration {
            id: 0,
            project_id: 1,
            name: "Test Integration".to_string(),
            template_id: 1,
            auth_method: "none".to_string(),
            auth_header: None,
            auth_secret_id: None,
        };
        
        let created = db.create_integration(integration).await.unwrap();
        
        let mut updated = created.clone();
        updated.name = "Updated Integration".to_string();
        
        db.update_integration(updated).await.unwrap();
        
        let retrieved = db.get_integration(1, created.id).await.unwrap();
        assert_eq!(retrieved.name, "Updated Integration");
        
        // Cleanup
        let _ = db.close().await;
    }

    #[tokio::test]
    async fn test_delete_integration() {
        let TestDb { db, _temp } = create_test_db().await;
        
        let integration = Integration {
            id: 0,
            project_id: 1,
            name: "Test Integration".to_string(),
            template_id: 1,
            auth_method: "none".to_string(),
            auth_header: None,
            auth_secret_id: None,
        };
        
        let created = db.create_integration(integration).await.unwrap();
        
        db.delete_integration(1, created.id).await.unwrap();
        
        let result = db.get_integration(1, created.id).await;
        assert!(result.is_err());
        
        // Cleanup
        let _ = db.close().await;
    }
}
