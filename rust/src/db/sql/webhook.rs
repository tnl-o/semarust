//! Webhook SQL реализация
//!
//! Заглушка для совместимости. Полная реализация требует доработки.

use crate::error::{Error, Result};
use crate::models::webhook::{Webhook, UpdateWebhook, WebhookLog};
use super::SqlDb;

impl SqlDb {
    /// Получает webhook по ID
    pub async fn get_webhook(&self, _webhook_id: i64) -> Result<Webhook> {
        Err(Error::NotFound("Webhook not implemented yet".to_string()))
    }

    /// Получает webhook проекта
    pub async fn get_webhooks_by_project(&self, _project_id: i64) -> Result<Vec<Webhook>> {
        Ok(Vec::new())
    }

    /// Создаёт webhook
    pub async fn create_webhook(&self, _webhook: Webhook) -> Result<Webhook> {
        Err(Error::Other("Webhook not implemented yet".to_string()))
    }

    /// Обновляет webhook
    pub async fn update_webhook(&self, _webhook_id: i64, _webhook: UpdateWebhook) -> Result<Webhook> {
        Err(Error::Other("Webhook not implemented yet".to_string()))
    }

    /// Удаляет webhook
    pub async fn delete_webhook(&self, _webhook_id: i64) -> Result<()> {
        Ok(())
    }

    /// Получает логи webhook
    pub async fn get_webhook_logs(&self, _webhook_id: i64) -> Result<Vec<WebhookLog>> {
        Ok(Vec::new())
    }

    /// Создаёт лог webhook
    pub async fn create_webhook_log(&self, _log: WebhookLog) -> Result<WebhookLog> {
        Err(Error::Other("Webhook not implemented yet".to_string()))
    }
}
