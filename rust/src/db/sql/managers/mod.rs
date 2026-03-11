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
//! - `WebhookManager` - управление webhook

pub mod connection;
pub mod migration;
pub mod options;
pub mod webhook;

// Ре-экспорт трейтов для удобства
pub use connection::*;
pub use migration::*;
pub use options::*;
pub use webhook::*;

// ============================================================================
// Store trait implementation
// ============================================================================

use async_trait::async_trait;

#[async_trait]
impl Store for SqlStore {}
