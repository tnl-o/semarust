//! Менеджеры хранилища данных
//!
//! Этот модуль содержит реализации трейтов менеджеров для SqlStore
//!
//! # Структура
//!
//! Каждый менеджер реализует свой трейт из `crate::db::store`:
//! - `ConnectionManager` - управление подключением к БД
//! - `MigrationManager` - управление миграциями схемы
//! - `OptionsManager` - управление опциями приложения
//! - `UserManager` - управление пользователями
//! - `ProjectStore` - управление проектами
//! - `TemplateManager` - управление шаблонами
//! - `TaskManager` - управление задачами
//! - `InventoryManager` - управление инвентарями
//! - `RepositoryManager` - управление репозиториями
//! - `EnvironmentManager` - управление окружениями
//! - `AccessKeyManager` - управление ключами доступа
//! - `WebhookManager` - управление webhook

pub mod connection;
pub mod migration;
pub mod options;
pub mod user;
pub mod project;
pub mod template;
pub mod task;
pub mod inventory;
pub mod repository;
pub mod environment;
pub mod access_key;
pub mod webhook;

// Ре-экспорт трейтов для удобства
pub use connection::*;
pub use migration::*;
pub use options::*;
pub use user::*;
pub use project::*;
pub use template::*;
pub use task::*;
pub use inventory::*;
pub use repository::*;
pub use environment::*;
pub use access_key::*;
pub use webhook::*;

// ============================================================================
// Store trait implementation
// ============================================================================

use async_trait::async_trait;

#[async_trait]
impl Store for SqlStore {}
