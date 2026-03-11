//! SQL-хранилище (SQLite)

pub mod runner;
pub mod types;
pub mod init;
pub mod migrations;
pub mod queries;
pub mod utils;
pub mod audit_log;
pub mod webhook;
pub mod managers;

#[cfg(test)]
pub mod test_helpers;

// Decomposed modules by dialect
pub mod sqlite;
pub mod postgres;
pub mod mysql;

// Legacy modules (to be removed)
pub mod template_crud;
pub mod template_vault;
pub mod template_roles;
pub mod template_utils;
pub mod user_crud;
pub mod user_auth;
pub mod user_totp;
pub mod task_crud;
pub mod task_output;
pub mod task_stage;
pub mod integration_crud;
pub mod integration_matcher;
pub mod integration_extract;
pub mod project_invite;
pub mod terraform_inventory;
pub mod access_key;
pub mod environment;
pub mod event;
pub mod inventory;
pub mod repository;
pub mod schedule;
pub mod session;
pub mod view;

use crate::db::store::*;
use crate::models::{User, UserTotp, Hook, Project, Task, TaskWithTpl, TaskOutput, TaskStage, Template, Inventory, Repository, Environment, AccessKey, Integration, Schedule, Session, APIToken, Event, Runner, View, Role, ProjectInvite, ProjectInviteWithUser, ProjectUser, RetrieveQueryParams, TerraformInventoryAlias, TerraformInventoryState, SecretStorage, SessionVerificationMethod};
use crate::models::playbook::{Playbook, PlaybookCreate, PlaybookUpdate};
use crate::models::audit_log::{AuditAction, AuditObjectType, AuditLevel, AuditLog, AuditLogFilter, AuditLogResult};
use crate::error::{Error, Result};
use crate::services::task_logger::TaskStatus;
use crate::db::sql::types::{SqlDb, SqlDialect};
use async_trait::async_trait;
use sqlx::{SqlitePool, PgPool, MySqlPool, Row};
use std::collections::HashMap;
use chrono::Utc;

/// SQL-хранилище данных (на базе SQLite, MySQL, PostgreSQL)
pub struct SqlStore {
    db: SqlDb,
}

impl SqlStore {
    /// Создаёт новое SQL-хранилище
    pub async fn new(database_url: &str) -> Result<Self> {
        // Используем функцию создания подключения из init.rs
        let db = init::create_database_connection(database_url).await?;

        let store = Self { db };
        store.ensure_schema().await?;
        Ok(store)
    }

    /// Инициализирует схему БД при первом запуске (если таблицы не существуют)
    async fn ensure_schema(&self) -> Result<()> {
        if self.get_dialect() != SqlDialect::SQLite {
            return Ok(());
        }
        let pool = self.get_sqlite_pool()?;

        // Проверяем, есть ли таблица project (основная для CRUD)
        let project_exists: Option<String> = sqlx::query_scalar(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='project'",
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        if project_exists.is_some() {
            // Миграция: добавить колонку created в project__user, если её нет
            Self::migrate_project_user_created(pool).await?;
            return Ok(());
        }

        // Проверяем, есть ли таблица user (для обратной совместимости со старыми БД)
        let user_exists: Option<String> = sqlx::query_scalar(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='user'",
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        tracing::info!("Инициализация схемы БД (создание недостающих таблиц)...");

        // Таблица миграций
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS migration (version INTEGER PRIMARY KEY, name TEXT)",
        )
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        if user_exists.is_none() {
            // Таблица пользователей (только при первом запуске)
            sqlx::query(
                "CREATE TABLE IF NOT EXISTS user (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    username TEXT NOT NULL,
                    name TEXT NOT NULL,
                    email TEXT NOT NULL,
                    password TEXT NOT NULL,
                    admin INTEGER NOT NULL,
                    external INTEGER NOT NULL,
                    alert INTEGER NOT NULL,
                    pro INTEGER NOT NULL,
                    created DATETIME NOT NULL,
                    totp TEXT,
                    email_otp TEXT
                )",
            )
            .execute(pool)
            .await
            .map_err(|e| Error::Database(e))?;

            // Таблица опций
            sqlx::query(
                "CREATE TABLE IF NOT EXISTS option (key TEXT PRIMARY KEY, value TEXT NOT NULL)",
            )
            .execute(pool)
            .await
            .map_err(|e| Error::Database(e))?;

            sqlx::query("INSERT OR IGNORE INTO migration (version, name) VALUES (1, 'initial_schema')")
                .execute(pool)
                .await
                .map_err(|e| Error::Database(e))?;
        }

        // Таблица проектов (для CRUD) — создаём если отсутствует
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS project (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                created DATETIME NOT NULL,
                alert INTEGER NOT NULL DEFAULT 0,
                alert_chat TEXT,
                max_parallel_tasks INTEGER NOT NULL DEFAULT 0,
                type TEXT NOT NULL DEFAULT '',
                default_secret_storage_id INTEGER
            )",
        )
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        // project__user для связи пользователей с проектами
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS project__user (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id INTEGER NOT NULL REFERENCES project(id) ON DELETE CASCADE,
                user_id INTEGER NOT NULL REFERENCES user(id) ON DELETE CASCADE,
                role TEXT NOT NULL,
                created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(project_id, user_id)
            )",
        )
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        // task_output для логов задач (GET /api/.../tasks/{id}/output)
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS task_output (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                task_id INTEGER NOT NULL,
                project_id INTEGER NOT NULL,
                output TEXT NOT NULL,
                time DATETIME NOT NULL,
                stage_id INTEGER
            )",
        )
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        // project_invite для приглашений в проект
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS project_invite (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id INTEGER NOT NULL REFERENCES project(id) ON DELETE CASCADE,
                user_id INTEGER NOT NULL REFERENCES user(id) ON DELETE CASCADE,
                role TEXT NOT NULL,
                created DATETIME NOT NULL,
                updated DATETIME NOT NULL,
                token TEXT NOT NULL DEFAULT '',
                inviter_user_id INTEGER NOT NULL
            )",
        )
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        tracing::info!("Схема БД инициализирована");
        Ok(())
    }

    /// Миграция: добавить колонку created в project__user, если её нет
    async fn migrate_project_user_created(pool: &SqlitePool) -> Result<()> {
        let table_exists: Option<String> = sqlx::query_scalar(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='project__user'",
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        if table_exists.is_none() {
            return Ok(());
        }

        let has_created: Option<i64> = sqlx::query_scalar(
            "SELECT COUNT(*) FROM pragma_table_info('project__user') WHERE name='created'",
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        if has_created.unwrap_or(0) == 0 {
            sqlx::query(
                "ALTER TABLE project__user ADD COLUMN created DATETIME NOT NULL DEFAULT '2020-01-01 00:00:00'",
            )
            .execute(pool)
            .await
            .map_err(|e| Error::Database(e))?;
            tracing::info!("Миграция: добавлена колонка created в project__user");
        }
        Ok(())
    }

    #[cfg(test)]
    /// Инициализирует таблицу user для тестов (без миграций)
    pub async fn init_user_table_for_test(&self) -> Result<()> {
        let schema = "CREATE TABLE IF NOT EXISTS user (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            password TEXT NOT NULL,
            admin INTEGER NOT NULL,
            external INTEGER NOT NULL,
            alert INTEGER NOT NULL,
            pro INTEGER NOT NULL,
            created DATETIME NOT NULL,
            totp TEXT,
            email_otp TEXT
        )";
        sqlx::query(schema)
            .execute(self.get_sqlite_pool()?)
            .await
            .map_err(|e| Error::Database(e))?;
        Ok(())
    }

    /// Получает диалект БД
    fn get_dialect(&self) -> SqlDialect {
        self.db.get_dialect()
    }

    /// Получает SQLite pool
    fn get_sqlite_pool(&self) -> Result<&SqlitePool> {
        self.db.get_sqlite_pool()
            .ok_or_else(|| Error::Other("SQLite pool not found".to_string()))
    }

    /// Получает PostgreSQL pool
    fn get_postgres_pool(&self) -> Result<&PgPool> {
        self.db.get_postgres_pool()
            .ok_or_else(|| Error::Other("PostgreSQL pool not found".to_string()))
    }

    /// Получает MySQL pool
    fn get_mysql_pool(&self) -> Result<&MySqlPool> {
        self.db.get_mysql_pool()
            .ok_or_else(|| Error::Other("MySQL pool not found".to_string()))
    }
}

#[async_trait]
impl HookManager for SqlStore {
    async fn get_hooks_by_template(&self, template_id: i32) -> Result<Vec<Hook>> {
        // Заглушка - возвращаем пустой список
        Ok(Vec::new())
    }
}

#[async_trait]
impl ScheduleManager for SqlStore {
    async fn get_schedules(&self, project_id: i32) -> Result<Vec<Schedule>> {
        match self.get_dialect() {
            SqlDialect::SQLite => {
                let query = "SELECT * FROM schedule WHERE project_id = ? ORDER BY name";
                let rows = sqlx::query(query).bind(project_id).fetch_all(self.get_sqlite_pool()?).await.map_err(|e| Error::Database(e))?;
                Ok(rows.into_iter().map(|row| Schedule {
                    id: row.get("id"),
                    project_id: row.get("project_id"),
                    template_id: row.get("template_id"),
                    cron: row.get("cron"),
                    cron_format: row.try_get("cron_format").ok().flatten(),
                    name: row.get("name"),
                    active: row.get("active"),
                    last_commit_hash: row.try_get("last_commit_hash").ok().flatten(),
                    repository_id: row.try_get("repository_id").ok(),
                    created: row.try_get("created").ok(),
                }).collect())
            }
            SqlDialect::PostgreSQL => {
                let query = "SELECT * FROM schedule WHERE project_id = $1 ORDER BY name";
                let rows = sqlx::query(query).bind(project_id).fetch_all(self.get_postgres_pool()?).await.map_err(|e| Error::Database(e))?;
                Ok(rows.into_iter().map(|row| Schedule {
                    id: row.get("id"),
                    project_id: row.get("project_id"),
                    template_id: row.get("template_id"),
                    cron: row.get("cron"),
                    cron_format: row.try_get("cron_format").ok().flatten(),
                    name: row.get("name"),
                    active: row.get("active"),
                    last_commit_hash: row.try_get("last_commit_hash").ok().flatten(),
                    repository_id: row.try_get("repository_id").ok(),
                    created: row.try_get("created").ok(),
                }).collect())
            }
            SqlDialect::MySQL => {
                let query = "SELECT * FROM `schedule` WHERE project_id = ? ORDER BY name";
                let rows = sqlx::query(query).bind(project_id).fetch_all(self.get_mysql_pool()?).await.map_err(|e| Error::Database(e))?;
                Ok(rows.into_iter().map(|row| Schedule {
                    id: row.get("id"),
                    project_id: row.get("project_id"),
                    template_id: row.get("template_id"),
                    cron: row.get("cron"),
                    cron_format: row.try_get("cron_format").ok().flatten(),
                    name: row.get("name"),
                    active: row.get("active"),
                    last_commit_hash: row.try_get("last_commit_hash").ok().flatten(),
                    repository_id: row.try_get("repository_id").ok(),
                    created: row.try_get("created").ok(),
                }).collect())
            }
        }
    }

    async fn get_schedule(&self, _project_id: i32, schedule_id: i32) -> Result<Schedule> {
        match self.get_dialect() {
            SqlDialect::SQLite => {
                let query = "SELECT * FROM schedule WHERE id = ?";
                let row = sqlx::query(query).bind(schedule_id).fetch_one(self.get_sqlite_pool()?).await.map_err(|e| match e {
                    sqlx::Error::RowNotFound => Error::NotFound("Расписание не найдено".to_string()),
                    _ => Error::Database(e),
                })?;
                Ok(Schedule {
                    id: row.get("id"),
                    project_id: row.get("project_id"),
                    template_id: row.get("template_id"),
                    cron: row.get("cron"),
                    cron_format: row.try_get("cron_format").ok().flatten(),
                    name: row.get("name"),
                    active: row.get("active"),
                    last_commit_hash: row.try_get("last_commit_hash").ok().flatten(),
                    repository_id: row.try_get("repository_id").ok(),
                    created: row.try_get("created").ok(),
                })
            }
            SqlDialect::PostgreSQL => {
                let query = "SELECT * FROM schedule WHERE id = $1";
                let row = sqlx::query(query).bind(schedule_id).fetch_one(self.get_postgres_pool()?).await.map_err(|e| match e {
                    sqlx::Error::RowNotFound => Error::NotFound("Расписание не найдено".to_string()),
                    _ => Error::Database(e),
                })?;
                Ok(Schedule {
                    id: row.get("id"),
                    project_id: row.get("project_id"),
                    template_id: row.get("template_id"),
                    cron: row.get("cron"),
                    cron_format: row.try_get("cron_format").ok().flatten(),
                    name: row.get("name"),
                    active: row.get("active"),
                    last_commit_hash: row.try_get("last_commit_hash").ok().flatten(),
                    repository_id: row.try_get("repository_id").ok(),
                    created: row.try_get("created").ok(),
                })
            }
            SqlDialect::MySQL => {
                let query = "SELECT * FROM `schedule` WHERE id = ?";
                let row = sqlx::query(query).bind(schedule_id).fetch_one(self.get_mysql_pool()?).await.map_err(|e| match e {
                    sqlx::Error::RowNotFound => Error::NotFound("Расписание не найдено".to_string()),
                    _ => Error::Database(e),
                })?;
                Ok(Schedule {
                    id: row.get("id"),
                    project_id: row.get("project_id"),
                    template_id: row.get("template_id"),
                    cron: row.get("cron"),
                    cron_format: row.try_get("cron_format").ok().flatten(),
                    name: row.get("name"),
                    active: row.get("active"),
                    last_commit_hash: row.try_get("last_commit_hash").ok().flatten(),
                    repository_id: row.try_get("repository_id").ok(),
                    created: row.try_get("created").ok(),
                })
            }
        }
    }

    async fn create_schedule(&self, mut schedule: Schedule) -> Result<Schedule> {
        match self.get_dialect() {
            SqlDialect::SQLite => {
                let query = "INSERT INTO schedule (project_id, template_id, cron, name, active, created) VALUES (?, ?, ?, ?, ?, ?) RETURNING id";
                let id: i32 = sqlx::query_scalar(query)
                    .bind(schedule.project_id)
                    .bind(schedule.template_id)
                    .bind(&schedule.cron)
                    .bind(&schedule.name)
                    .bind(schedule.active)
                    .bind(&schedule.created)
                    .fetch_one(self.get_sqlite_pool()?).await.map_err(|e| Error::Database(e))?;
                schedule.id = id;
                Ok(schedule)
            }
            SqlDialect::PostgreSQL => {
                let query = "INSERT INTO schedule (project_id, template_id, cron, name, active, created) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id";
                let id: i32 = sqlx::query_scalar(query)
                    .bind(schedule.project_id)
                    .bind(schedule.template_id)
                    .bind(&schedule.cron)
                    .bind(&schedule.name)
                    .bind(schedule.active)
                    .bind(&schedule.created)
                    .fetch_one(self.get_postgres_pool()?).await.map_err(|e| Error::Database(e))?;
                schedule.id = id;
                Ok(schedule)
            }
            SqlDialect::MySQL => {
                let query = "INSERT INTO `schedule` (project_id, template_id, cron, name, active, created) VALUES (?, ?, ?, ?, ?, ?)";
                let result = sqlx::query(query)
                    .bind(schedule.project_id)
                    .bind(schedule.template_id)
                    .bind(&schedule.cron)
                    .bind(&schedule.name)
                    .bind(schedule.active)
                    .bind(&schedule.created)
                    .execute(self.get_mysql_pool()?).await.map_err(|e| Error::Database(e))?;
                schedule.id = result.last_insert_id() as i32;
                Ok(schedule)
            }
        }
    }

    async fn update_schedule(&self, schedule: Schedule) -> Result<()> {
        match self.get_dialect() {
            SqlDialect::SQLite => {
                let query = "UPDATE schedule SET cron = ?, name = ?, active = ? WHERE id = ?";
                sqlx::query(query)
                    .bind(&schedule.cron)
                    .bind(&schedule.name)
                    .bind(schedule.active)
                    .bind(schedule.id)
                    .execute(self.get_sqlite_pool()?).await.map_err(|e| Error::Database(e))?;
            }
            SqlDialect::PostgreSQL => {
                let query = "UPDATE schedule SET cron = $1, name = $2, active = $3 WHERE id = $4";
                sqlx::query(query)
                    .bind(&schedule.cron)
                    .bind(&schedule.name)
                    .bind(schedule.active)
                    .bind(schedule.id)
                    .execute(self.get_postgres_pool()?).await.map_err(|e| Error::Database(e))?;
            }
            SqlDialect::MySQL => {
                let query = "UPDATE `schedule` SET cron = ?, name = ?, active = ? WHERE id = ?";
                sqlx::query(query)
                    .bind(&schedule.cron)
                    .bind(&schedule.name)
                    .bind(schedule.active)
                    .bind(schedule.id)
                    .execute(self.get_mysql_pool()?).await.map_err(|e| Error::Database(e))?;
            }
        }
        Ok(())
    }

    async fn delete_schedule(&self, _project_id: i32, schedule_id: i32) -> Result<()> {
        match self.get_dialect() {
            SqlDialect::SQLite => {
                let query = "DELETE FROM schedule WHERE id = ?";
                sqlx::query(query).bind(schedule_id).execute(self.get_sqlite_pool()?).await.map_err(|e| Error::Database(e))?;
            }
            SqlDialect::PostgreSQL => {
                let query = "DELETE FROM schedule WHERE id = $1";
                sqlx::query(query).bind(schedule_id).execute(self.get_postgres_pool()?).await.map_err(|e| Error::Database(e))?;
            }
            SqlDialect::MySQL => {
                let query = "DELETE FROM `schedule` WHERE id = ?";
                sqlx::query(query).bind(schedule_id).execute(self.get_mysql_pool()?).await.map_err(|e| Error::Database(e))?;
            }
        }
        Ok(())
    }

    async fn set_schedule_active(&self, _project_id: i32, schedule_id: i32, active: bool) -> Result<()> {
        match self.get_dialect() {
            SqlDialect::SQLite => {
                let query = "UPDATE schedule SET active = ? WHERE id = ?";
                sqlx::query(query).bind(active).bind(schedule_id).execute(self.get_sqlite_pool()?).await.map_err(|e| Error::Database(e))?;
            }
            SqlDialect::PostgreSQL => {
                let query = "UPDATE schedule SET active = $1 WHERE id = $2";
                sqlx::query(query).bind(active).bind(schedule_id).execute(self.get_postgres_pool()?).await.map_err(|e| Error::Database(e))?;
            }
            SqlDialect::MySQL => {
                let query = "UPDATE `schedule` SET active = ? WHERE id = ?";
                sqlx::query(query).bind(active).bind(schedule_id).execute(self.get_mysql_pool()?).await.map_err(|e| Error::Database(e))?;
            }
        }
        Ok(())
    }

    async fn set_schedule_commit_hash(&self, _project_id: i32, schedule_id: i32, hash: &str) -> Result<()> {
        match self.get_dialect() {
            SqlDialect::SQLite => {
                sqlx::query("UPDATE schedule SET last_commit_hash = ? WHERE id = ?")
                    .bind(hash)
                    .bind(schedule_id)
                    .execute(self.get_sqlite_pool()?)
                    .await
                    .map_err(|e| Error::Database(e))?;
            }
            SqlDialect::PostgreSQL => {
                sqlx::query("UPDATE schedule SET last_commit_hash = $1 WHERE id = $2")
                    .bind(hash)
                    .bind(schedule_id)
                    .execute(self.get_postgres_pool()?)
                    .await
                    .map_err(|e| Error::Database(e))?;
            }
            SqlDialect::MySQL => {
                sqlx::query("UPDATE `schedule` SET last_commit_hash = ? WHERE id = ?")
                    .bind(hash)
                    .bind(schedule_id)
                    .execute(self.get_mysql_pool()?)
                    .await
                    .map_err(|e| Error::Database(e))?;
            }
        }
        Ok(())
    }
}

#[async_trait]
impl SessionManager for SqlStore {
    async fn get_session(&self, _user_id: i32, session_id: i32) -> Result<Session> {
        match self.get_dialect() {
            SqlDialect::SQLite => {
                let query = "SELECT * FROM session WHERE id = ?";
                let row = sqlx::query(query).bind(session_id).fetch_one(self.get_sqlite_pool()?).await.map_err(|e| match e {
                    sqlx::Error::RowNotFound => Error::NotFound("Сессия не найдена".to_string()),
                    _ => Error::Database(e),
                })?;
                Ok(Session {
                    id: row.get("id"),
                    user_id: row.get("user_id"),
                    created: row.get("created"),
                    last_active: row.get("last_active"),
                    ip: row.try_get("ip").ok().unwrap_or_default(),
                    user_agent: row.try_get("user_agent").ok().unwrap_or_default(),
                    expired: row.get("expired"),
                    verification_method: row.try_get("verification_method").ok().unwrap_or(SessionVerificationMethod::None),
                    verified: row.try_get("verified").ok().unwrap_or(false),
                })
            }
            SqlDialect::PostgreSQL => {
                let query = "SELECT * FROM session WHERE id = $1";
                let row = sqlx::query(query).bind(session_id).fetch_one(self.get_postgres_pool()?).await.map_err(|e| match e {
                    sqlx::Error::RowNotFound => Error::NotFound("Сессия не найдена".to_string()),
                    _ => Error::Database(e),
                })?;
                Ok(Session {
                    id: row.get("id"),
                    user_id: row.get("user_id"),
                    created: row.get("created"),
                    last_active: row.get("last_active"),
                    ip: row.try_get("ip").ok().unwrap_or_default(),
                    user_agent: row.try_get("user_agent").ok().unwrap_or_default(),
                    expired: row.get("expired"),
                    verification_method: row.try_get("verification_method").ok().unwrap_or(SessionVerificationMethod::None),
                    verified: row.try_get("verified").ok().unwrap_or(false),
                })
            }
            SqlDialect::MySQL => {
                let query = "SELECT * FROM `session` WHERE id = ?";
                let row = sqlx::query(query).bind(session_id).fetch_one(self.get_mysql_pool()?).await.map_err(|e| match e {
                    sqlx::Error::RowNotFound => Error::NotFound("Сессия не найдена".to_string()),
                    _ => Error::Database(e),
                })?;
                Ok(Session {
                    id: row.get("id"),
                    user_id: row.get("user_id"),
                    created: row.get("created"),
                    last_active: row.get("last_active"),
                    ip: row.try_get("ip").ok().unwrap_or_default(),
                    user_agent: row.try_get("user_agent").ok().unwrap_or_default(),
                    expired: row.get("expired"),
                    verification_method: row.try_get("verification_method").ok().unwrap_or(SessionVerificationMethod::None),
                    verified: row.try_get("verified").ok().unwrap_or(false),
                })
            }
        }
    }

    async fn create_session(&self, mut session: Session) -> Result<Session> {
        match self.get_dialect() {
            SqlDialect::SQLite => {
                let query = "INSERT INTO session (user_id, created, last_active, ip, user_agent, expired, verification_method, verified) VALUES (?, ?, ?, ?, ?, ?, ?, ?) RETURNING id";
                let id: i32 = sqlx::query_scalar(query)
                    .bind(session.user_id)
                    .bind(session.created)
                    .bind(session.last_active)
                    .bind(&session.ip)
                    .bind(&session.user_agent)
                    .bind(session.expired)
                    .bind(&session.verification_method)
                    .bind(session.verified)
                    .fetch_one(self.get_sqlite_pool()?).await.map_err(|e| Error::Database(e))?;
                session.id = id;
                Ok(session)
            }
            SqlDialect::PostgreSQL => {
                let query = "INSERT INTO session (user_id, created, last_active, ip, user_agent, expired, verification_method, verified) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id";
                let id: i32 = sqlx::query_scalar(query)
                    .bind(session.user_id)
                    .bind(session.created)
                    .bind(session.last_active)
                    .bind(&session.ip)
                    .bind(&session.user_agent)
                    .bind(session.expired)
                    .bind(&session.verification_method)
                    .bind(session.verified)
                    .fetch_one(self.get_postgres_pool()?).await.map_err(|e| Error::Database(e))?;
                session.id = id;
                Ok(session)
            }
            SqlDialect::MySQL => {
                let query = "INSERT INTO `session` (user_id, created, last_active, ip, user_agent, expired, verification_method, verified) VALUES (?, ?, ?, ?, ?, ?, ?, ?)";
                let result = sqlx::query(query)
                    .bind(session.user_id)
                    .bind(session.created)
                    .bind(session.last_active)
                    .bind(&session.ip)
                    .bind(&session.user_agent)
                    .bind(session.expired)
                    .bind(&session.verification_method)
                    .bind(session.verified)
                    .execute(self.get_mysql_pool()?).await.map_err(|e| Error::Database(e))?;
                session.id = result.last_insert_id() as i32;
                Ok(session)
            }
        }
    }

    async fn expire_session(&self, _user_id: i32, session_id: i32) -> Result<()> {
        match self.get_dialect() {
            SqlDialect::SQLite => {
                let query = "UPDATE session SET expired = 1 WHERE id = ?";
                sqlx::query(query).bind(session_id).execute(self.get_sqlite_pool()?).await.map_err(|e| Error::Database(e))?;
            }
            SqlDialect::PostgreSQL => {
                let query = "UPDATE session SET expired = TRUE WHERE id = $1";
                sqlx::query(query).bind(session_id).execute(self.get_postgres_pool()?).await.map_err(|e| Error::Database(e))?;
            }
            SqlDialect::MySQL => {
                let query = "UPDATE `session` SET expired = 1 WHERE id = ?";
                sqlx::query(query).bind(session_id).execute(self.get_mysql_pool()?).await.map_err(|e| Error::Database(e))?;
            }
        }
        Ok(())
    }

    async fn verify_session(&self, _user_id: i32, _session_id: i32) -> Result<()> {
        // TODO: реализовать проверку сессии
        Ok(())
    }

    async fn touch_session(&self, _user_id: i32, session_id: i32) -> Result<()> {
        match self.get_dialect() {
            SqlDialect::SQLite => {
                let query = "UPDATE session SET last_active = ? WHERE id = ?";
                sqlx::query(query).bind(Utc::now()).bind(session_id).execute(self.get_sqlite_pool()?).await.map_err(|e| Error::Database(e))?;
            }
            SqlDialect::PostgreSQL => {
                let query = "UPDATE session SET last_active = $1 WHERE id = $2";
                sqlx::query(query).bind(Utc::now()).bind(session_id).execute(self.get_postgres_pool()?).await.map_err(|e| Error::Database(e))?;
            }
            SqlDialect::MySQL => {
                let query = "UPDATE `session` SET last_active = ? WHERE id = ?";
                sqlx::query(query).bind(Utc::now()).bind(session_id).execute(self.get_mysql_pool()?).await.map_err(|e| Error::Database(e))?;
            }
        }
        Ok(())
    }
}

#[async_trait]
impl TokenManager for SqlStore {
    async fn get_api_tokens(&self, user_id: i32) -> Result<Vec<APIToken>> {
        match self.get_dialect() {
            SqlDialect::SQLite => {
                let query = "SELECT * FROM api_token WHERE user_id = ? ORDER BY created DESC";
                let rows = sqlx::query(query).bind(user_id).fetch_all(self.get_sqlite_pool()?).await.map_err(|e| Error::Database(e))?;
                Ok(rows.into_iter().map(|row| APIToken {
                    id: row.get("id"),
                    user_id: row.get("user_id"),
                    name: row.get("name"),
                    token: row.get("token"),
                    created: row.get("created"),
                    expired: row.get("expired"),
                }).collect())
            }
            SqlDialect::PostgreSQL => {
                let query = "SELECT * FROM api_token WHERE user_id = $1 ORDER BY created DESC";
                let rows = sqlx::query(query).bind(user_id).fetch_all(self.get_postgres_pool()?).await.map_err(|e| Error::Database(e))?;
                Ok(rows.into_iter().map(|row| APIToken {
                    id: row.get("id"),
                    user_id: row.get("user_id"),
                    name: row.get("name"),
                    token: row.get("token"),
                    created: row.get("created"),
                    expired: row.get("expired"),
                }).collect())
            }
            SqlDialect::MySQL => {
                let query = "SELECT * FROM `api_token` WHERE user_id = ? ORDER BY created DESC";
                let rows = sqlx::query(query).bind(user_id).fetch_all(self.get_mysql_pool()?).await.map_err(|e| Error::Database(e))?;
                Ok(rows.into_iter().map(|row| APIToken {
                    id: row.get("id"),
                    user_id: row.get("user_id"),
                    name: row.get("name"),
                    token: row.get("token"),
                    created: row.get("created"),
                    expired: row.get("expired"),
                }).collect())
            }
        }
    }

    async fn create_api_token(&self, mut token: APIToken) -> Result<APIToken> {
        match self.get_dialect() {
            SqlDialect::SQLite => {
                let query = "INSERT INTO api_token (user_id, name, token, created, expired) VALUES (?, ?, ?, ?, ?) RETURNING id";
                let id: i32 = sqlx::query_scalar(query)
                    .bind(token.user_id)
                    .bind(&token.name)
                    .bind(&token.token)
                    .bind(token.created)
                    .bind(token.expired)
                    .fetch_one(self.get_sqlite_pool()?).await.map_err(|e| Error::Database(e))?;
                token.id = id;
                Ok(token)
            }
            SqlDialect::PostgreSQL => {
                let query = "INSERT INTO api_token (user_id, name, token, created, expired) VALUES ($1, $2, $3, $4, $5) RETURNING id";
                let id: i32 = sqlx::query_scalar(query)
                    .bind(token.user_id)
                    .bind(&token.name)
                    .bind(&token.token)
                    .bind(token.created)
                    .bind(token.expired)
                    .fetch_one(self.get_postgres_pool()?).await.map_err(|e| Error::Database(e))?;
                token.id = id;
                Ok(token)
            }
            SqlDialect::MySQL => {
                let query = "INSERT INTO `api_token` (user_id, name, token, created, expired) VALUES (?, ?, ?, ?, ?)";
                let result = sqlx::query(query)
                    .bind(token.user_id)
                    .bind(&token.name)
                    .bind(&token.token)
                    .bind(token.created)
                    .bind(token.expired)
                    .execute(self.get_mysql_pool()?).await.map_err(|e| Error::Database(e))?;
                token.id = result.last_insert_id() as i32;
                Ok(token)
            }
        }
    }

    async fn get_api_token(&self, token_id: i32) -> Result<APIToken> {
        match self.get_dialect() {
            SqlDialect::SQLite => {
                let query = "SELECT * FROM api_token WHERE id = ?";
                let row = sqlx::query(query).bind(token_id).fetch_one(self.get_sqlite_pool()?).await.map_err(|e| match e {
                    sqlx::Error::RowNotFound => Error::NotFound("Токен не найден".to_string()),
                    _ => Error::Database(e),
                })?;
                Ok(APIToken {
                    id: row.get("id"),
                    user_id: row.get("user_id"),
                    name: row.get("name"),
                    token: row.get("token"),
                    created: row.get("created"),
                    expired: row.get("expired"),
                })
            }
            SqlDialect::PostgreSQL => {
                let query = "SELECT * FROM api_token WHERE id = $1";
                let row = sqlx::query(query).bind(token_id).fetch_one(self.get_postgres_pool()?).await.map_err(|e| match e {
                    sqlx::Error::RowNotFound => Error::NotFound("Токен не найден".to_string()),
                    _ => Error::Database(e),
                })?;
                Ok(APIToken {
                    id: row.get("id"),
                    user_id: row.get("user_id"),
                    name: row.get("name"),
                    token: row.get("token"),
                    created: row.get("created"),
                    expired: row.get("expired"),
                })
            }
            SqlDialect::MySQL => {
                let query = "SELECT * FROM `api_token` WHERE id = ?";
                let row = sqlx::query(query).bind(token_id).fetch_one(self.get_mysql_pool()?).await.map_err(|e| match e {
                    sqlx::Error::RowNotFound => Error::NotFound("Токен не найден".to_string()),
                    _ => Error::Database(e),
                })?;
                Ok(APIToken {
                    id: row.get("id"),
                    user_id: row.get("user_id"),
                    name: row.get("name"),
                    token: row.get("token"),
                    created: row.get("created"),
                    expired: row.get("expired"),
                })
            }
        }
    }

    async fn expire_api_token(&self, _user_id: i32, token_id: i32) -> Result<()> {
        match self.get_dialect() {
            SqlDialect::SQLite => {
                let query = "UPDATE api_token SET expired = 1 WHERE id = ?";
                sqlx::query(query).bind(token_id).execute(self.get_sqlite_pool()?).await.map_err(|e| Error::Database(e))?;
            }
            SqlDialect::PostgreSQL => {
                let query = "UPDATE api_token SET expired = TRUE WHERE id = $1";
                sqlx::query(query).bind(token_id).execute(self.get_postgres_pool()?).await.map_err(|e| Error::Database(e))?;
            }
            SqlDialect::MySQL => {
                let query = "UPDATE `api_token` SET expired = 1 WHERE id = ?";
                sqlx::query(query).bind(token_id).execute(self.get_mysql_pool()?).await.map_err(|e| Error::Database(e))?;
            }
        }
        Ok(())
    }

    async fn delete_api_token(&self, _user_id: i32, token_id: i32) -> Result<()> {
        match self.get_dialect() {
            SqlDialect::SQLite => {
                let query = "DELETE FROM api_token WHERE id = ?";
                sqlx::query(query).bind(token_id).execute(self.get_sqlite_pool()?).await.map_err(|e| Error::Database(e))?;
            }
            SqlDialect::PostgreSQL => {
                let query = "DELETE FROM api_token WHERE id = $1";
                sqlx::query(query).bind(token_id).execute(self.get_postgres_pool()?).await.map_err(|e| Error::Database(e))?;
            }
            SqlDialect::MySQL => {
                let query = "DELETE FROM `api_token` WHERE id = ?";
                sqlx::query(query).bind(token_id).execute(self.get_mysql_pool()?).await.map_err(|e| Error::Database(e))?;
            }
        }
        Ok(())
    }
}

#[async_trait]
impl EventManager for SqlStore {
    async fn get_events(&self, project_id: Option<i32>, limit: usize) -> Result<Vec<Event>> {
        match self.get_dialect() {
            SqlDialect::SQLite => {
                let query = if project_id.is_some() {
                    "SELECT * FROM event WHERE project_id = ? ORDER BY created DESC LIMIT ?"
                } else {
                    "SELECT * FROM event ORDER BY created DESC LIMIT ?"
                };
                let mut q = sqlx::query(query);
                if let Some(pid) = project_id {
                    q = q.bind(pid);
                }
                let rows = q.bind(limit as i64).fetch_all(self.get_sqlite_pool()?).await.map_err(|e| Error::Database(e))?;
                Ok(rows.into_iter().map(|row| Event {
                    id: row.get("id"),
                    project_id: row.try_get("project_id").ok(),
                    user_id: row.try_get("user_id").ok(),
                    object_id: row.try_get("object_id").ok(),
                    object_type: row.get("object_type"),
                    description: row.get("description"),
                    created: row.get("created"),
                }).collect())
            }
            SqlDialect::PostgreSQL => {
                let query = if project_id.is_some() {
                    "SELECT * FROM event WHERE project_id = $1 ORDER BY created DESC LIMIT $2"
                } else {
                    "SELECT * FROM event ORDER BY created DESC LIMIT $1"
                };
                let mut q = sqlx::query(query);
                if let Some(pid) = project_id {
                    q = q.bind(pid);
                }
                let rows = q.bind(limit as i64).fetch_all(self.get_postgres_pool()?).await.map_err(|e| Error::Database(e))?;
                Ok(rows.into_iter().map(|row| Event {
                    id: row.get("id"),
                    project_id: row.try_get("project_id").ok(),
                    user_id: row.try_get("user_id").ok(),
                    object_id: row.try_get("object_id").ok(),
                    object_type: row.get("object_type"),
                    description: row.get("description"),
                    created: row.get("created"),
                }).collect())
            }
            SqlDialect::MySQL => {
                let query = if project_id.is_some() {
                    "SELECT * FROM `event` WHERE project_id = ? ORDER BY created DESC LIMIT ?"
                } else {
                    "SELECT * FROM `event` ORDER BY created DESC LIMIT ?"
                };
                let mut q = sqlx::query(query);
                if let Some(pid) = project_id {
                    q = q.bind(pid);
                }
                let rows = q.bind(limit as i64).fetch_all(self.get_mysql_pool()?).await.map_err(|e| Error::Database(e))?;
                Ok(rows.into_iter().map(|row| Event {
                    id: row.get("id"),
                    project_id: row.try_get("project_id").ok(),
                    user_id: row.try_get("user_id").ok(),
                    object_id: row.try_get("object_id").ok(),
                    object_type: row.get("object_type"),
                    description: row.get("description"),
                    created: row.get("created"),
                }).collect())
            }
        }
    }

    async fn create_event(&self, mut event: Event) -> Result<Event> {
        match self.get_dialect() {
            SqlDialect::SQLite => {
                let query = "INSERT INTO event (project_id, user_id, object_id, object_type, description, created) VALUES (?, ?, ?, ?, ?, ?) RETURNING id";
                let id: i32 = sqlx::query_scalar(query)
                    .bind(&event.project_id)
                    .bind(&event.user_id)
                    .bind(&event.object_id)
                    .bind(&event.object_type)
                    .bind(&event.description)
                    .bind(event.created)
                    .fetch_one(self.get_sqlite_pool()?).await.map_err(|e| Error::Database(e))?;
                event.id = id;
                Ok(event)
            }
            SqlDialect::PostgreSQL => {
                let query = "INSERT INTO event (project_id, user_id, object_id, object_type, description, created) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id";
                let id: i32 = sqlx::query_scalar(query)
                    .bind(&event.project_id)
                    .bind(&event.user_id)
                    .bind(&event.object_id)
                    .bind(&event.object_type)
                    .bind(&event.description)
                    .bind(event.created)
                    .fetch_one(self.get_postgres_pool()?).await.map_err(|e| Error::Database(e))?;
                event.id = id;
                Ok(event)
            }
            SqlDialect::MySQL => {
                let query = "INSERT INTO `event` (project_id, user_id, object_id, object_type, description, created) VALUES (?, ?, ?, ?, ?, ?)";
                let result = sqlx::query(query)
                    .bind(&event.project_id)
                    .bind(&event.user_id)
                    .bind(&event.object_id)
                    .bind(&event.object_type)
                    .bind(&event.description)
                    .bind(event.created)
                    .execute(self.get_mysql_pool()?).await.map_err(|e| Error::Database(e))?;
                event.id = result.last_insert_id() as i32;
                Ok(event)
            }
        }
    }
}

#[async_trait]
impl RunnerManager for SqlStore {
    async fn get_runners(&self, _project_id: Option<i32>) -> Result<Vec<Runner>> { Ok(vec![]) }
    async fn get_runner(&self, _runner_id: i32) -> Result<Runner> { Err(Error::NotFound("Раннер не найден".to_string())) }
    async fn create_runner(&self, _runner: Runner) -> Result<Runner> { Err(Error::Other("Не реализовано".to_string())) }
    async fn update_runner(&self, _runner: Runner) -> Result<()> { Err(Error::Other("Не реализовано".to_string())) }
    async fn delete_runner(&self, _runner_id: i32) -> Result<()> { Err(Error::Other("Не реализовано".to_string())) }
}

#[async_trait]
impl ViewManager for SqlStore {
    async fn get_views(&self, _project_id: i32) -> Result<Vec<View>> { Ok(vec![]) }
    async fn get_view(&self, _project_id: i32, _view_id: i32) -> Result<View> { Err(Error::NotFound("Представление не найдено".to_string())) }
    async fn create_view(&self, _view: View) -> Result<View> { Err(Error::Other("Не реализовано".to_string())) }
    async fn update_view(&self, _view: View) -> Result<()> { Err(Error::Other("Не реализовано".to_string())) }
    async fn delete_view(&self, _project_id: i32, _view_id: i32) -> Result<()> { Err(Error::Other("Не реализовано".to_string())) }
    
    async fn set_view_positions(&self, project_id: i32, positions: Vec<(i32, i32)>) -> Result<()> {
        // positions: Vec<(view_id, position)>
        for (view_id, position) in positions {
            match self.get_dialect() {
                SqlDialect::SQLite => {
                    let query = "UPDATE view SET position = ? WHERE id = ? AND project_id = ?";
                    sqlx::query(query)
                        .bind(position)
                        .bind(view_id)
                        .bind(project_id)
                        .execute(self.get_sqlite_pool()?)
                        .await
                        .map_err(|e| Error::Database(e))?;
                }
                SqlDialect::PostgreSQL => {
                    let query = "UPDATE view SET position = $1 WHERE id = $2 AND project_id = $3";
                    sqlx::query(query)
                        .bind(position)
                        .bind(view_id)
                        .bind(project_id)
                        .execute(self.get_postgres_pool()?)
                        .await
                        .map_err(|e| Error::Database(e))?;
                }
                SqlDialect::MySQL => {
                    let query = "UPDATE `view` SET position = ? WHERE id = ? AND project_id = ?";
                    sqlx::query(query)
                        .bind(position)
                        .bind(view_id)
                        .bind(project_id)
                        .execute(self.get_mysql_pool()?)
                        .await
                        .map_err(|e| Error::Database(e))?;
                }
            }
        }
        Ok(())
    }
}

#[async_trait]
impl IntegrationManager for SqlStore {
    async fn get_integrations(&self, _project_id: i32) -> Result<Vec<Integration>> { Ok(vec![]) }
    async fn get_integration(&self, _project_id: i32, _integration_id: i32) -> Result<Integration> { Err(Error::NotFound("Интеграция не найдена".to_string())) }
    async fn create_integration(&self, _integration: Integration) -> Result<Integration> { Err(Error::Other("Не реализовано".to_string())) }
    async fn update_integration(&self, _integration: Integration) -> Result<()> { Err(Error::Other("Не реализовано".to_string())) }
    async fn delete_integration(&self, _project_id: i32, _integration_id: i32) -> Result<()> { Err(Error::Other("Не реализовано".to_string())) }
}

#[async_trait]
impl ProjectInviteManager for SqlStore {
    async fn get_project_invites(&self, project_id: i32, params: RetrieveQueryParams) -> Result<Vec<ProjectInviteWithUser>> {
        self.get_project_invites(project_id, params).await
    }

    async fn create_project_invite(&self, invite: ProjectInvite) -> Result<ProjectInvite> {
        self.create_project_invite(invite).await
    }

    async fn get_project_invite(&self, project_id: i32, invite_id: i32) -> Result<ProjectInvite> {
        self.get_project_invite(project_id, invite_id).await
    }

    async fn get_project_invite_by_token(&self, token: &str) -> Result<ProjectInvite> {
        self.get_project_invite_by_token(token).await
    }

    async fn update_project_invite(&self, invite: ProjectInvite) -> Result<()> {
        self.update_project_invite(invite).await
    }

    async fn delete_project_invite(&self, project_id: i32, invite_id: i32) -> Result<()> {
        self.delete_project_invite(project_id, invite_id).await
    }
}

#[async_trait]
impl TerraformInventoryManager for SqlStore {
    async fn create_terraform_inventory_alias(&self, alias: TerraformInventoryAlias) -> Result<TerraformInventoryAlias> {
        self.create_terraform_inventory_alias(alias).await
    }

    async fn update_terraform_inventory_alias(&self, alias: TerraformInventoryAlias) -> Result<()> {
        self.update_terraform_inventory_alias(alias).await
    }

    async fn get_terraform_inventory_alias_by_alias(&self, alias: &str) -> Result<TerraformInventoryAlias> {
        self.get_terraform_inventory_alias_by_alias(alias).await
    }

    async fn get_terraform_inventory_alias(&self, project_id: i32, inventory_id: i32, alias_id: &str) -> Result<TerraformInventoryAlias> {
        self.get_terraform_inventory_alias(project_id, inventory_id, alias_id).await
    }

    async fn get_terraform_inventory_aliases(&self, project_id: i32, inventory_id: i32) -> Result<Vec<TerraformInventoryAlias>> {
        self.get_terraform_inventory_aliases(project_id, inventory_id).await
    }

    async fn delete_terraform_inventory_alias(&self, project_id: i32, inventory_id: i32, alias_id: &str) -> Result<()> {
        self.delete_terraform_inventory_alias(project_id, inventory_id, alias_id).await
    }

    async fn get_terraform_inventory_states(&self, project_id: i32, inventory_id: i32, params: RetrieveQueryParams) -> Result<Vec<TerraformInventoryState>> {
        self.get_terraform_inventory_states(project_id, inventory_id, params).await
    }

    async fn create_terraform_inventory_state(&self, state: TerraformInventoryState) -> Result<TerraformInventoryState> {
        self.create_terraform_inventory_state(state).await
    }

    async fn delete_terraform_inventory_state(&self, project_id: i32, inventory_id: i32, state_id: i32) -> Result<()> {
        self.delete_terraform_inventory_state(project_id, inventory_id, state_id).await
    }

    async fn get_terraform_inventory_state(&self, project_id: i32, inventory_id: i32, state_id: i32) -> Result<TerraformInventoryState> {
        self.get_terraform_inventory_state(project_id, inventory_id, state_id).await
    }

    async fn get_terraform_state_count(&self) -> Result<i32> {
        self.get_terraform_state_count().await
    }
}

#[async_trait]
impl SecretStorageManager for SqlStore {
    async fn get_secret_storages(&self, project_id: i32) -> Result<Vec<SecretStorage>> {
        match self.get_dialect() {
            SqlDialect::SQLite => {
                let storages = sqlx::query_as::<_, SecretStorage>(
                    "SELECT * FROM secret_storage WHERE project_id = ? ORDER BY name"
                )
                .bind(project_id)
                .fetch_all(self.get_sqlite_pool()?)
                .await
                .map_err(|e| Error::Database(e))?;

                Ok(storages)
            }
            SqlDialect::PostgreSQL => {
                let storages = sqlx::query_as::<_, SecretStorage>(
                    "SELECT * FROM secret_storage WHERE project_id = $1 ORDER BY name"
                )
                .bind(project_id)
                .fetch_all(self.get_postgres_pool()?)
                .await
                .map_err(|e| Error::Database(e))?;

                Ok(storages)
            }
            SqlDialect::MySQL => {
                let storages = sqlx::query_as::<_, SecretStorage>(
                    "SELECT * FROM secret_storage WHERE project_id = ? ORDER BY name"
                )
                .bind(project_id)
                .fetch_all(self.get_mysql_pool()?)
                .await
                .map_err(|e| Error::Database(e))?;

                Ok(storages)
            }
        }
    }

    async fn get_secret_storage(&self, project_id: i32, storage_id: i32) -> Result<SecretStorage> {
        match self.get_dialect() {
            SqlDialect::SQLite => {
                let storage = sqlx::query_as::<_, SecretStorage>(
                    "SELECT * FROM secret_storage WHERE id = ? AND project_id = ?"
                )
                .bind(storage_id)
                .bind(project_id)
                .fetch_optional(self.get_sqlite_pool()?)
                .await
                .map_err(|e| Error::Database(e))?;

                storage.ok_or(Error::NotFound("SecretStorage not found".to_string()))
            }
            SqlDialect::PostgreSQL => {
                let storage = sqlx::query_as::<_, SecretStorage>(
                    "SELECT * FROM secret_storage WHERE id = $1 AND project_id = $2"
                )
                .bind(storage_id)
                .bind(project_id)
                .fetch_optional(self.get_postgres_pool()?)
                .await
                .map_err(|e| Error::Database(e))?;

                storage.ok_or(Error::NotFound("SecretStorage not found".to_string()))
            }
            SqlDialect::MySQL => {
                let storage = sqlx::query_as::<_, SecretStorage>(
                    "SELECT * FROM secret_storage WHERE id = ? AND project_id = ?"
                )
                .bind(storage_id)
                .bind(project_id)
                .fetch_optional(self.get_mysql_pool()?)
                .await
                .map_err(|e| Error::Database(e))?;

                storage.ok_or(Error::NotFound("SecretStorage not found".to_string()))
            }
        }
    }

    async fn create_secret_storage(&self, mut storage: SecretStorage) -> Result<SecretStorage> {
        match self.get_dialect() {
            SqlDialect::SQLite => {
                let result = sqlx::query(
                    "INSERT INTO secret_storage (project_id, name, type, params, read_only) VALUES (?, ?, ?, ?, ?)"
                )
                .bind(storage.project_id)
                .bind(&storage.name)
                .bind(&storage.r#type.to_string())
                .bind(&storage.params)
                .bind(storage.read_only)
                .execute(self.get_sqlite_pool()?)
                .await
                .map_err(|e| Error::Database(e))?;

                storage.id = result.last_insert_rowid() as i32;
                Ok(storage)
            }
            SqlDialect::PostgreSQL => {
                let query = "INSERT INTO secret_storage (project_id, name, type, params, read_only) VALUES ($1, $2, $3, $4, $5) RETURNING id";
                let id: i32 = sqlx::query_scalar(query)
                    .bind(storage.project_id)
                    .bind(&storage.name)
                    .bind(&storage.r#type.to_string())
                    .bind(&storage.params)
                    .bind(storage.read_only)
                    .fetch_one(self.get_postgres_pool()?)
                    .await
                    .map_err(|e| Error::Database(e))?;

                storage.id = id;
                Ok(storage)
            }
            SqlDialect::MySQL => {
                let result = sqlx::query(
                    "INSERT INTO secret_storage (project_id, name, type, params, read_only) VALUES (?, ?, ?, ?, ?)"
                )
                .bind(storage.project_id)
                .bind(&storage.name)
                .bind(&storage.r#type.to_string())
                .bind(&storage.params)
                .bind(storage.read_only)
                .execute(self.get_mysql_pool()?)
                .await
                .map_err(|e| Error::Database(e))?;

                storage.id = result.last_insert_id() as i32;
                Ok(storage)
            }
        }
    }

    async fn update_secret_storage(&self, storage: SecretStorage) -> Result<()> {
        match self.get_dialect() {
            SqlDialect::SQLite => {
                sqlx::query(
                    "UPDATE secret_storage SET name = ?, type = ?, params = ?, read_only = ? WHERE id = ? AND project_id = ?"
                )
                .bind(&storage.name)
                .bind(&storage.r#type.to_string())
                .bind(&storage.params)
                .bind(storage.read_only)
                .bind(storage.id)
                .bind(storage.project_id)
                .execute(self.get_sqlite_pool()?)
                .await
                .map_err(|e| Error::Database(e))?;
            }
            SqlDialect::PostgreSQL => {
                sqlx::query(
                    "UPDATE secret_storage SET name = $1, type = $2, params = $3, read_only = $4 WHERE id = $5 AND project_id = $6"
                )
                .bind(&storage.name)
                .bind(&storage.r#type.to_string())
                .bind(&storage.params)
                .bind(storage.read_only)
                .bind(storage.id)
                .bind(storage.project_id)
                .execute(self.get_postgres_pool()?)
                .await
                .map_err(|e| Error::Database(e))?;
            }
            SqlDialect::MySQL => {
                sqlx::query(
                    "UPDATE secret_storage SET name = ?, type = ?, params = ?, read_only = ? WHERE id = ? AND project_id = ?"
                )
                .bind(&storage.name)
                .bind(&storage.r#type.to_string())
                .bind(&storage.params)
                .bind(storage.read_only)
                .bind(storage.id)
                .bind(storage.project_id)
                .execute(self.get_mysql_pool()?)
                .await
                .map_err(|e| Error::Database(e))?;
            }
        }

        Ok(())
    }

    async fn delete_secret_storage(&self, project_id: i32, storage_id: i32) -> Result<()> {
        match self.get_dialect() {
            SqlDialect::SQLite => {
                sqlx::query("DELETE FROM secret_storage WHERE id = ? AND project_id = ?")
                    .bind(storage_id)
                    .bind(project_id)
                    .execute(self.get_sqlite_pool()?)
                    .await
                    .map_err(|e| Error::Database(e))?;
            }
            SqlDialect::PostgreSQL => {
                sqlx::query("DELETE FROM secret_storage WHERE id = $1 AND project_id = $2")
                    .bind(storage_id)
                    .bind(project_id)
                    .execute(self.get_postgres_pool()?)
                    .await
                    .map_err(|e| Error::Database(e))?;
            }
            SqlDialect::MySQL => {
                sqlx::query("DELETE FROM secret_storage WHERE id = ? AND project_id = ?")
                    .bind(storage_id)
                    .bind(project_id)
                    .execute(self.get_mysql_pool()?)
                    .await
                    .map_err(|e| Error::Database(e))?;
            }
        }

        Ok(())
    }
}

#[async_trait]
impl AuditLogManager for SqlStore {
    async fn create_audit_log(
        &self,
        project_id: Option<i64>,
        user_id: Option<i64>,
        username: Option<String>,
        action: &AuditAction,
        object_type: &AuditObjectType,
        object_id: Option<i64>,
        object_name: Option<String>,
        description: String,
        level: &AuditLevel,
        ip_address: Option<String>,
        user_agent: Option<String>,
        details: Option<serde_json::Value>,
    ) -> Result<AuditLog> {
        self.db
            .create_audit_log(
                project_id,
                user_id,
                username,
                action,
                object_type,
                object_id,
                object_name,
                description,
                level,
                ip_address,
                user_agent,
                details,
            )
            .await
    }

    async fn get_audit_log(&self, id: i64) -> Result<AuditLog> {
        self.db.get_audit_log(id).await
    }

    async fn search_audit_logs(&self, filter: &AuditLogFilter) -> Result<AuditLogResult> {
        self.db.search_audit_logs(filter).await
    }

    async fn get_audit_logs_by_project(&self, project_id: i64, limit: i64, offset: i64) -> Result<Vec<AuditLog>> {
        self.db.get_audit_logs_by_project(project_id, limit, offset).await
    }

    async fn get_audit_logs_by_user(&self, user_id: i64, limit: i64, offset: i64) -> Result<Vec<AuditLog>> {
        self.db.get_audit_logs_by_user(user_id, limit, offset).await
    }

    async fn get_audit_logs_by_action(&self, action: &AuditAction, limit: i64, offset: i64) -> Result<Vec<AuditLog>> {
        self.db.get_audit_logs_by_action(action, limit, offset).await
    }

    async fn delete_audit_logs_before(&self, before: chrono::DateTime<Utc>) -> Result<u64> {
        self.db.delete_audit_logs_before(before).await
    }

    async fn clear_audit_log(&self) -> Result<u64> {
        self.db.clear_audit_log().await
    }
}

#[async_trait]
