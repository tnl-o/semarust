//! Менеджер хранилища данных
//!
//! Автоматически извлечён из mod.rs в рамках декомпозиции

use crate::db::sql::SqlStore;
use crate::db::store::*;
use crate::error::{Error, Result};
use async_trait::async_trait;

#[async_trait]
impl ConnectionManager for SqlStore {
    async fn connect(&self) -> Result<()> {
        Ok(())
    }

    async fn close(&self) -> Result<()> {
        match self.get_dialect() {
            SqlDialect::SQLite => self.get_sqlite_pool()?.close().await,
            SqlDialect::PostgreSQL => self.get_postgres_pool()?.close().await,
            SqlDialect::MySQL => self.get_mysql_pool()?.close().await,
        }
        Ok(())
    }

    fn is_permanent(&self) -> bool {
        true
    }
}

