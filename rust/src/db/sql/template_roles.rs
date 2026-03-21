//! Template Roles - операции с TemplateRole
//!
//! Аналог db/sql/template.go из Go версии (часть 3: TemplateRole)

use crate::db::sql::types::SqlDb;
use crate::error::{Error, Result};
use crate::models::*;
use sqlx::Row;

impl SqlDb {
    /// Получает все роли для шаблона
    pub async fn get_template_roles(&self, project_id: i32, template_id: i32) -> Result<Vec<TemplateRolePerm>> {
        match unreachable!() {
            
        }
    }
    
    /// Создаёт TemplateRole
    pub async fn create_template_role(&self, mut role: TemplateRolePerm) -> Result<TemplateRolePerm> {
        match unreachable!() {
            
        }
    }
    
    /// Обновляет TemplateRole
    pub async fn update_template_role(&self, role: TemplateRolePerm) -> Result<()> {
        match unreachable!() {
            
        }
    }
    
    /// Удаляет TemplateRole
    pub async fn delete_template_role(&self, project_id: i32, template_id: i32, role_id: i32) -> Result<()> {
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

        // Создаём таблицу template_role
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS template_role (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                template_id INTEGER NOT NULL,
                project_id INTEGER NOT NULL,
                role_id INTEGER NOT NULL,
                role_slug TEXT NOT NULL DEFAULT ''
            )"
        )
        .execute(db.get_sqlite_pool().unwrap())
        .await
        .unwrap();

        TestDb { db, _temp: temp }
    }

    #[tokio::test]
    async fn test_create_and_get_template_role() {
        let TestDb { db, _temp } = create_test_db().await;
        
        let role = TemplateRolePerm {
            id: 0,
            template_id: 1,
            project_id: 1,
            role_id: 2,
            role_slug: "admin".to_string(),
        };
        
        let created = db.create_template_role(role.clone()).await.unwrap();
        assert!(created.id > 0);
        
        let roles = db.get_template_roles(1, 1).await.unwrap();
        assert!(roles.len() >= 1);
        assert_eq!(roles[0].role_id, 2);
        
        // Cleanup
        let _ = db.close().await;
    }

    #[tokio::test]
    async fn test_update_template_role() {
        let TestDb { db, _temp } = create_test_db().await;
        
        let role = TemplateRolePerm {
            id: 0,
            template_id: 1,
            project_id: 1,
            role_id: 2,
            role_slug: "admin".to_string(),
        };
        
        let created = db.create_template_role(role).await.unwrap();
        
        let mut updated = created.clone();
        updated.role_id = 3;
        
        db.update_template_role(updated).await.unwrap();
        
        let roles = db.get_template_roles(1, 1).await.unwrap();
        assert_eq!(roles[0].role_id, 3);
        
        // Cleanup
        let _ = db.close().await;
    }

    #[tokio::test]
    async fn test_delete_template_role() {
        let TestDb { db, _temp } = create_test_db().await;
        
        let role = TemplateRolePerm {
            id: 0,
            template_id: 1,
            project_id: 1,
            role_id: 2,
            role_slug: "admin".to_string(),
        };
        
        let created = db.create_template_role(role).await.unwrap();
        
        db.delete_template_role(1, 1, created.id).await.unwrap();
        
        let roles = db.get_template_roles(1, 1).await.unwrap();
        assert!(roles.is_empty());
        
        // Cleanup
        let _ = db.close().await;
    }
}
