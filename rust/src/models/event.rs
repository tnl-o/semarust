//! Модель события

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Тип события
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    TaskCreated,
    TaskUpdated,
    TaskDeleted,
    TemplateCreated,
    TemplateUpdated,
    TemplateDeleted,
    InventoryCreated,
    InventoryUpdated,
    InventoryDeleted,
    RepositoryCreated,
    RepositoryUpdated,
    RepositoryDeleted,
    EnvironmentCreated,
    EnvironmentUpdated,
    EnvironmentDeleted,
    AccessKeyCreated,
    AccessKeyUpdated,
    AccessKeyDeleted,
    IntegrationCreated,
    IntegrationUpdated,
    IntegrationDeleted,
    ScheduleCreated,
    ScheduleUpdated,
    ScheduleDeleted,
    UserJoined,
    UserLeft,
    UserUpdated,
    ProjectUpdated,
    Other,
}

/// Событие системы
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Event {
    pub id: i32,
    pub project_id: Option<i32>,
    pub user_id: Option<i32>,
    pub object_id: Option<i32>,
    pub object_type: String,
    pub description: String,
    pub created: DateTime<Utc>,
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::TaskCreated => write!(f, "task_created"),
            EventType::TaskUpdated => write!(f, "task_updated"),
            EventType::TaskDeleted => write!(f, "task_deleted"),
            EventType::TemplateCreated => write!(f, "template_created"),
            EventType::TemplateUpdated => write!(f, "template_updated"),
            EventType::TemplateDeleted => write!(f, "template_deleted"),
            EventType::InventoryCreated => write!(f, "inventory_created"),
            EventType::InventoryUpdated => write!(f, "inventory_updated"),
            EventType::InventoryDeleted => write!(f, "inventory_deleted"),
            EventType::RepositoryCreated => write!(f, "repository_created"),
            EventType::RepositoryUpdated => write!(f, "repository_updated"),
            EventType::RepositoryDeleted => write!(f, "repository_deleted"),
            EventType::EnvironmentCreated => write!(f, "environment_created"),
            EventType::EnvironmentUpdated => write!(f, "environment_updated"),
            EventType::EnvironmentDeleted => write!(f, "environment_deleted"),
            EventType::AccessKeyCreated => write!(f, "access_key_created"),
            EventType::AccessKeyUpdated => write!(f, "access_key_updated"),
            EventType::AccessKeyDeleted => write!(f, "access_key_deleted"),
            EventType::IntegrationCreated => write!(f, "integration_created"),
            EventType::IntegrationUpdated => write!(f, "integration_updated"),
            EventType::IntegrationDeleted => write!(f, "integration_deleted"),
            _ => write!(f, "unknown"),
        }
    }
}
