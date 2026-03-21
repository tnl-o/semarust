//! Session CRUD Operations
//!
//! Операции с сессиями в SQL

use crate::db::sql::types::SqlDb;
use crate::error::{Error, Result};
use crate::models::Session;

impl SqlDb {
    /// Получает сессию по ID
    pub async fn get_session(&self, user_id: i32, session_id: i32) -> Result<Session> {
        match unreachable!() {
            
        }
    }

    /// Создаёт сессию
    pub async fn create_session(&self, mut session: Session) -> Result<Session> {
        match unreachable!() {
            
        }
    }

    /// Истекает сессию
    pub async fn expire_session(&self, user_id: i32, session_id: i32) -> Result<()> {
        match unreachable!() {
            
        }
    }
}
