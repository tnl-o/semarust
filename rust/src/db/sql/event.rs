//! Event CRUD Operations
//!
//! Операции с событиями в SQL

use crate::db::sql::types::SqlDb;
use crate::error::{Error, Result};
use crate::models::Event;

impl SqlDb {
    /// Получает события проекта
    pub async fn get_events(&self, project_id: Option<i32>, limit: usize) -> Result<Vec<Event>> {
        let query = if let Some(pid) = project_id {
                    sqlx::query_as::<_, Event>(
                        "SELECT * FROM event WHERE project_id = ? ORDER BY created DESC LIMIT ?"
                    )
                    .bind(pid)
                    .bind(limit as i64)
                } else {
                    sqlx::query_as::<_, Event>(
                        "SELECT * FROM event ORDER BY created DESC LIMIT ?"
                    )
                    .bind(limit as i64)
                };

                let events = query
                    .fetch_all(self.get_postgres_pool().ok_or(Error::Other("SQLite pool not found".to_string()))?)
                    .await
                    .map_err(Error::Database)?;

                Ok(events)
    }

    /// Создаёт событие
    pub async fn create_event(&self, mut event: Event) -> Result<Event> {
        match unreachable!() {
            
        }
    }
}
