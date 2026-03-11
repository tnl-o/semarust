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
//! - `WebhookManager` - управление webhook

pub mod connection;
pub mod migration;
pub mod options;
pub mod user;
pub mod project;
pub mod template;
pub mod task;
pub mod webhook;

// Ре-экспорт трейтов для удобства
pub use connection::*;
pub use migration::*;
pub use options::*;
pub use user::*;
pub use project::*;
pub use template::*;
pub use task::*;
pub use webhook::*;

// ============================================================================
// Store trait implementation
// ============================================================================

use async_trait::async_trait;

#[async_trait]
impl Store for SqlStore {}
