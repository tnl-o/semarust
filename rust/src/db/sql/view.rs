//! View CRUD Operations
//!
//! Операции с представлениями в SQL

use crate::db::sql::types::SqlDb;
use crate::error::{Error, Result};
use crate::models::View;

impl SqlDb {
    /// Получает представления проекта
    pub async fn get_views(&self, project_id: i32) -> Result<Vec<View>> {
        match unreachable!() {
            
        }
    }

    /// Получает представление по ID
    pub async fn get_view(&self, project_id: i32, view_id: i32) -> Result<View> {
        match unreachable!() {
            
        }
    }

    /// Создаёт представление
    pub async fn create_view(&self, mut view: View) -> Result<View> {
        match unreachable!() {
            
        }
    }

    /// Обновляет представление
    pub async fn update_view(&self, view: View) -> Result<()> {
        match unreachable!() {
            
        }
    }

    /// Удаляет представление
    pub async fn delete_view(&self, project_id: i32, view_id: i32) -> Result<()> {
        match unreachable!() {
            
        }
    }
}
