//! Session - операции с сессиями и API токенами в BoltDB
//!
//! Аналог db/bolt/session.go из Go версии

use crate::db::bolt::BoltStore;
use crate::error::Result;
use crate::models::{Session, APIToken, SessionVerificationMethod};
use chrono::Utc;

impl BoltStore {
    /// Создаёт новую сессию
    pub async fn create_session(&self, mut session: Session) -> Result<Session> {
        session.created = Utc::now();
        session.last_active = Utc::now();

        let session_clone = session.clone();

        let bucket_name = format!("sessions_{}", session.user_id);
        let tree = self.db.open_tree(bucket_name.as_bytes())
            .map_err(|e| crate::error::Error::Database(sqlx::Error::Protocol(e.to_string())))?;

        let id = self.get_next_id(&bucket_name)?;
        let id_key = (i64::MAX - id as i64).to_be_bytes();

        let mut session_with_id = session_clone;
        session_with_id.id = id as i32;

        let str = serde_json::to_vec(&session_with_id)
            .map_err(|e| crate::error::Error::Json(e))?;

        tree.insert(id_key, str)
            .map_err(|e| crate::error::Error::Database(sqlx::Error::Protocol(e.to_string())))?;

        Ok(session_with_id)
    }

    /// Получает сессию по ID
    pub async fn get_session(&self, user_id: i32, session_id: i32) -> Result<Session> {
        let bucket_name = format!("sessions_{}", user_id);
        let tree = self.db.open_tree(bucket_name.as_bytes())
            .map_err(|e| crate::error::Error::Database(sqlx::Error::Protocol(e.to_string())))?;

        let key = (i64::MAX - session_id as i64).to_be_bytes();

        if let Some(v) = tree.get(key)
            .map_err(|e| crate::error::Error::Database(sqlx::Error::Protocol(e.to_string())))? {
            let session: Session = serde_json::from_slice(&v)
                .map_err(|e| crate::error::Error::Json(e))?;
            Ok(session)
        } else {
            Err(crate::error::Error::NotFound("Сессия не найдена".to_string()))
        }
    }

    /// Завершает сессию
    pub async fn expire_session(&self, user_id: i32, session_id: i32) -> Result<()> {
        let mut session = self.get_session(user_id, session_id).await?;
        session.expired = true;
        self.update_session(user_id, session).await
    }

    /// Устанавливает метод верификации сессии
    pub async fn set_session_verification_method(&self, user_id: i32, session_id: i32, method: SessionVerificationMethod) -> Result<()> {
        let mut session = self.get_session(user_id, session_id).await?;
        session.verification_method = method;
        self.update_session(user_id, session).await
    }

    /// Подтверждает сессию
    pub async fn verify_session(&self, user_id: i32, session_id: i32) -> Result<()> {
        let mut session = self.get_session(user_id, session_id).await?;
        session.verified = true;
        self.update_session(user_id, session).await
    }

    /// Обновляет время активности сессии
    pub async fn touch_session(&self, user_id: i32, session_id: i32) -> Result<()> {
        let mut session = self.get_session(user_id, session_id).await?;
        session.last_active = Utc::now();
        self.update_session(user_id, session).await
    }

    /// Обновляет сессию
    async fn update_session(&self, user_id: i32, session: Session) -> Result<()> {
        let bucket_name = format!("sessions_{}", user_id);
        let tree = self.db.open_tree(bucket_name.as_bytes())
            .map_err(|e| crate::error::Error::Database(sqlx::Error::Protocol(e.to_string())))?;

        let key = (i64::MAX - session.id as i64).to_be_bytes();

        if tree.get(key)
            .map_err(|e| crate::error::Error::Database(sqlx::Error::Protocol(e.to_string())))?.is_none() {
            return Err(crate::error::Error::NotFound("Сессия не найдена".to_string()));
        }

        let str = serde_json::to_vec(&session)
            .map_err(|e| crate::error::Error::Json(e))?;

        tree.insert(key, str)
            .map_err(|e| crate::error::Error::Database(sqlx::Error::Protocol(e.to_string())))?;

        Ok(())
    }

    /// Создаёт API токен
    pub async fn create_api_token(&self, mut token: APIToken) -> Result<APIToken> {
        token.created = Utc::now();
        token.expired = false;

        let token_clone = token.clone();
        let user_id = token.user_id;

        let bucket_name = format!("tokens_{}", user_id);
        let tree = self.db.open_tree(bucket_name.as_bytes())
            .map_err(|e| crate::error::Error::Database(sqlx::Error::Protocol(e.to_string())))?;

        let id = self.get_next_id(&bucket_name)?;
        let id_key = format!("token_{:x}", i64::MAX - id as i64);

        let mut token_with_id = token_clone;
        token_with_id.id = id_key;

        let str = serde_json::to_vec(&token_with_id)
            .map_err(|e| crate::error::Error::Json(e))?;

        tree.insert(token_with_id.id.as_bytes(), str)
            .map_err(|e| crate::error::Error::Database(sqlx::Error::Protocol(e.to_string())))?;

        Ok(token_with_id)
    }

    /// Получает API токен по ID
    pub async fn get_api_token(&self, token_id: &str) -> Result<APIToken> {
        // Ищем токен во всех пользователях
        let all_users = self.get_all_users().await?;

        for user in all_users {
            let tokens = self.get_api_tokens(user.id).await?;
            for token in tokens {
                if token.id == token_id {
                    return Ok(token);
                }
            }
        }

        Err(crate::error::Error::NotFound("Токен не найден".to_string()))
    }

    /// Завершает API токен
    pub async fn expire_api_token(&self, user_id: i32, token_id: &str) -> Result<()> {
        let tokens = self.get_api_tokens(user_id).await?;

        for token in tokens {
            if token.id.starts_with(token_id) {
                return self.update_api_token(user_id, token_id, true).await;
            }
        }

        Err(crate::error::Error::NotFound("Токен не найден".to_string()))
    }

    /// Удаляет API токен
    pub async fn delete_api_token(&self, user_id: i32, token_id: &str) -> Result<()> {
        let tokens = self.get_api_tokens(user_id).await?;

        for token in tokens {
            if token.id.starts_with(token_id) {
                let bucket_name = format!("tokens_{}", user_id);
                let tree = self.db.open_tree(bucket_name.as_bytes())
                    .map_err(|e| crate::error::Error::Database(sqlx::Error::Protocol(e.to_string())))?;

                tree.remove(token.id.as_bytes())
                    .map_err(|e| crate::error::Error::Database(sqlx::Error::Protocol(e.to_string())))?;

                return Ok(());
            }
        }

        Err(crate::error::Error::NotFound("Токен не найден".to_string()))
    }

    /// Получает все API токены пользователя
    pub async fn get_api_tokens(&self, user_id: i32) -> Result<Vec<APIToken>> {
        let mut tokens = Vec::new();

        let bucket_name = format!("tokens_{}", user_id);
        let tree = self.db.open_tree(bucket_name.as_bytes())
            .map_err(|e| crate::error::Error::Database(sqlx::Error::Protocol(e.to_string())))?;

        for item in tree.iter() {
            let (_k, v) = item
                .map_err(|e| crate::error::Error::Database(sqlx::Error::Protocol(e.to_string())))?;

            let token: APIToken = serde_json::from_slice(&v)
                .map_err(|e| crate::error::Error::Json(e))?;
            tokens.push(token);
        }

        // Сортируем по дате создания (новые первые)
        tokens.sort_by(|a, b| b.created.cmp(&a.created));

        Ok(tokens)
    }

    /// Обновляет API токен
    async fn update_api_token(&self, user_id: i32, token_id: &str, expired: bool) -> Result<()> {
        let bucket_name = format!("tokens_{}", user_id);
        let tree = self.db.open_tree(bucket_name.as_bytes())
            .map_err(|e| crate::error::Error::Database(sqlx::Error::Protocol(e.to_string())))?;

        // Ищем токен по префиксу
        for item in tree.iter() {
            let (k, v) = item
                .map_err(|e| crate::error::Error::Database(sqlx::Error::Protocol(e.to_string())))?;

            let mut token: APIToken = serde_json::from_slice(&v)
                .map_err(|e| crate::error::Error::Json(e))?;

            if token.id.starts_with(token_id) {
                token.expired = expired;
                let str = serde_json::to_vec(&token)
                    .map_err(|e| crate::error::Error::Json(e))?;

                tree.insert(k, str)
                    .map_err(|e| crate::error::Error::Database(sqlx::Error::Protocol(e.to_string())))?;

                return Ok(());
            }
        }

        Err(crate::error::Error::NotFound("Токен не найден".to_string()))
    }

    /// Получает всех пользователей (вспомогательный метод)
    async fn get_all_users(&self) -> Result<Vec<crate::models::User>> {
        // Заглушка - в реальной реализации нужно получить всех пользователей
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::path::PathBuf;

    fn create_test_bolt_db() -> BoltStore {
        let path = PathBuf::from("/tmp/test_sessions.db");
        BoltStore::new(path).unwrap()
    }

    fn create_test_session(user_id: i32) -> Session {
        Session {
            id: 0,
            user_id,
            created: Utc::now(),
            last_active: Utc::now(),
            ip: "127.0.0.1".to_string(),
            user_agent: "Test".to_string(),
            expired: false,
            verification_method: SessionVerificationMethod::None,
            verified: true,
        }
    }

    fn create_test_token(user_id: i32) -> APIToken {
        APIToken {
            id: format!("test_token_{}", user_id),
            user_id,
            created: Utc::now(),
            expired: false,
        }
    }

    #[tokio::test]
    async fn test_create_session() {
        let db = create_test_bolt_db();
        let session = create_test_session(1);

        let result = db.create_session(session).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_session() {
        let db = create_test_bolt_db();
        let session = create_test_session(1);
        let created = db.create_session(session).await.unwrap();

        let retrieved = db.get_session(1, created.id).await;
        assert!(retrieved.is_ok());
        assert_eq!(retrieved.unwrap().id, created.id);
    }

    #[tokio::test]
    async fn test_expire_session() {
        let db = create_test_bolt_db();
        let session = create_test_session(1);
        let created = db.create_session(session).await.unwrap();

        let result = db.expire_session(1, created.id).await;
        assert!(result.is_ok());

        let retrieved = db.get_session(1, created.id).await;
        assert!(retrieved.is_ok());
        assert!(retrieved.unwrap().expired);
    }

    #[tokio::test]
    async fn test_verify_session() {
        let db = create_test_bolt_db();
        let session = create_test_session(1);
        let created = db.create_session(session).await.unwrap();

        let result = db.verify_session(1, created.id).await;
        assert!(result.is_ok());

        let retrieved = db.get_session(1, created.id).await;
        assert!(retrieved.is_ok());
        assert!(retrieved.unwrap().verified);
    }

    #[tokio::test]
    async fn test_touch_session() {
        let db = create_test_bolt_db();
        let session = create_test_session(1);
        let created = db.create_session(session).await.unwrap();

        let old_last_active = created.last_active;

        let result = db.touch_session(1, created.id).await;
        assert!(result.is_ok());

        let retrieved = db.get_session(1, created.id).await;
        assert!(retrieved.is_ok());
        assert!(retrieved.unwrap().last_active > old_last_active);
    }

    #[tokio::test]
    async fn test_create_api_token() {
        let db = create_test_bolt_db();
        let token = create_test_token(1);

        let result = db.create_api_token(token).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_api_tokens() {
        let db = create_test_bolt_db();

        // Создаём несколько токенов
        for i in 0..5 {
            let token = create_test_token(1);
            db.create_api_token(token).await.unwrap();
        }

        let tokens = db.get_api_tokens(1).await;
        assert!(tokens.is_ok());
        assert!(tokens.unwrap().len() >= 5);
    }

    #[tokio::test]
    async fn test_expire_api_token() {
        let db = create_test_bolt_db();
        let token = create_test_token(1);
        let created = db.create_api_token(token).await.unwrap();

        let result = db.expire_api_token(1, &created.id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_api_token() {
        let db = create_test_bolt_db();
        let token = create_test_token(1);
        let created = db.create_api_token(token).await.unwrap();

        let result = db.delete_api_token(1, &created.id).await;
        assert!(result.is_ok());
    }
}
