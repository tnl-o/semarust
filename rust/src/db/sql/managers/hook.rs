//! Менеджер хранилища данных
//!
//! Автоматически извлечён из mod.rs в рамках декомпозиции

use crate::db::sql::SqlStore;
use crate::db::store::*;
use crate::error::{Error, Result};
use async_trait::async_trait;

#[async_trait]
impl HookManager for SqlStore {
    async fn get_hooks_by_template(&self, template_id: i32) -> Result<Vec<Hook>> {
        // Заглушка - возвращаем пустой список
        Ok(Vec::new())
    }
}

