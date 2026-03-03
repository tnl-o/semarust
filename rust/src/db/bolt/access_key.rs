//! AccessKey - операции с ключами доступа в BoltDB
//!
//! Аналог db/bolt/access_key.go из Go версии

use crate::db::bolt::BoltStore;
use crate::error::Result;
use crate::models::AccessKey;

impl BoltStore {
    /// Получает ключ доступа по ID
    pub async fn get_access_key(&self, project_id: i32, access_key_id: i32) -> Result<AccessKey> {
        self.get_object(project_id, "access_keys", access_key_id).await
    }

    /// Получает рефереры ключа доступа
    pub async fn get_access_key_refs(&self, project_id: i32, access_key_id: i32) -> Result<crate::models::ObjectReferrers> {
        self.get_object_refs(project_id, "access_keys", access_key_id).await
    }

    /// Получает ключи доступа проекта
    pub async fn get_access_keys(&self, project_id: i32) -> Result<Vec<AccessKey>> {
        let params = crate::db::store::RetrieveQueryParams {
            offset: 0,
            count: None,
            filter: None,
            sort_by: None,
            sort_inverted: false,
        };
        self.get_objects::<AccessKey>(project_id, "access_keys", params).await
    }

    /// Обновляет ключ доступа
    pub async fn update_access_key(&self, key: AccessKey) -> Result<()> {
        self.update_object(key.project_id.unwrap_or(0), "access_keys", key).await
    }

    /// Создаёт ключ доступа
    pub async fn create_access_key(&self, mut key: AccessKey) -> Result<AccessKey> {
        let key_clone = key.clone();

        let tree = self.db.open_tree(b"access_keys")
            .map_err(|e| crate::error::Error::Database(sqlx::Error::Protocol(e.to_string())))?;

        let id = self.get_next_id("access_keys")?;
        let id_key = (i64::MAX - id as i64).to_be_bytes();

        let mut key_with_id = key_clone;
        key_with_id.id = id as i32;

        let str = serde_json::to_vec(&key_with_id)
            .map_err(|e| crate::error::Error::Json(e))?;

        tree.insert(id_key, str)
            .map_err(|e| crate::error::Error::Database(sqlx::Error::Protocol(e.to_string())))?;

        Ok(key_with_id)
    }

    /// Удаляет ключ доступа
    pub async fn delete_access_key(&self, project_id: i32, access_key_id: i32) -> Result<()> {
        self.delete_object(project_id, "access_keys", access_key_id).await
    }

    /// Перешифровывает ключи доступа (заглушка)
    pub async fn rekey_access_keys(&self, old_key: &str) -> Result<()> {
        // TODO: Реализовать перешифровку ключей
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::path::PathBuf;
    use crate::models::{AccessKeyType, AccessKeyOwner};

    fn create_test_bolt_db() -> BoltStore {
        let path = PathBuf::from("/tmp/test_access_keys.db");
        BoltStore::new(path).unwrap()
    }

    fn create_test_access_key(project_id: i32, name: &str) -> AccessKey {
        AccessKey {
            id: 0,
            project_id: Some(project_id),
            name: name.to_string(),
            r#type: AccessKeyType::None,
            user_id: None,
            login_password_login: None,
            login_password_password: None,
            ssh_key: None,
            ssh_passphrase: None,
            access_key_access_key: None,
            access_key_secret_key: None,
            secret_storage_id: None,
            owner: Some(AccessKeyOwner::Shared),
            environment_id: None,
        }
    }

    #[tokio::test]
    async fn test_create_access_key() {
        let db = create_test_bolt_db();
        let key = create_test_access_key(1, "Test Key");

        let result = db.create_access_key(key).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_access_key() {
        let db = create_test_bolt_db();
        let key = create_test_access_key(1, "Test Key");
        let created = db.create_access_key(key).await.unwrap();

        let retrieved = db.get_access_key(1, created.id).await;
        assert!(retrieved.is_ok());
        assert_eq!(retrieved.unwrap().name, "Test Key");
    }

    #[tokio::test]
    async fn test_get_access_keys() {
        let db = create_test_bolt_db();

        // Создаём несколько ключей
        for i in 0..5 {
            let key = create_test_access_key(1, &format!("Key {}", i));
            db.create_access_key(key).await.unwrap();
        }

        let keys = db.get_access_keys(1).await;
        assert!(keys.is_ok());
        assert!(keys.unwrap().len() >= 5);
    }

    #[tokio::test]
    async fn test_update_access_key() {
        let db = create_test_bolt_db();
        let key = create_test_access_key(1, "Test Key");
        let mut created = db.create_access_key(key).await.unwrap();

        created.name = "Updated Key".to_string();

        let result = db.update_access_key(created).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_access_key() {
        let db = create_test_bolt_db();
        let key = create_test_access_key(1, "Test Key");
        let created = db.create_access_key(key).await.unwrap();

        let result = db.delete_access_key(1, created.id).await;
        assert!(result.is_ok());

        let retrieved = db.get_access_key(1, created.id).await;
        assert!(retrieved.is_err());
    }
}
