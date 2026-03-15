//! Модель роли

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Роль - набор разрешений
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub id: i32,
    pub project_id: i32,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    /// Битмаска разрешений
    #[serde(default)]
    pub permissions: i32,
}

impl Role {
    /// Создаёт новую роль
    pub fn new(project_id: i32, slug: String, name: String) -> Self {
        Self {
            id: 0,
            project_id,
            slug,
            name,
            description: None,
            permissions: 0,
        }
    }
}
