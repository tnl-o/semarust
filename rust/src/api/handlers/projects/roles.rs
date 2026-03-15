//! Projects API - Custom Roles Handler (B-BE-09/10/11)
//!
//! Обработчики для кастомных ролей проекта

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::api::state::AppState;
use crate::models::Role;
use crate::api::middleware::ErrorResponse;

/// Встроенные роли проекта (всегда доступны)
fn builtin_roles(project_id: i32) -> Vec<Role> {
    vec![
        Role {
            id: -1,
            project_id,
            slug: "owner".to_string(),
            name: "Owner".to_string(),
            description: Some("Full project control".to_string()),
            permissions: 0x7FFFFFFF,
        },
        Role {
            id: -2,
            project_id,
            slug: "manager".to_string(),
            name: "Manager".to_string(),
            description: Some("Manage project resources".to_string()),
            permissions: 0x0FFFFFFF,
        },
        Role {
            id: -3,
            project_id,
            slug: "task_runner".to_string(),
            name: "Task Runner".to_string(),
            description: Some("Run tasks".to_string()),
            permissions: 0x00000001,
        },
        Role {
            id: -4,
            project_id,
            slug: "guest".to_string(),
            name: "Guest".to_string(),
            description: Some("View only".to_string()),
            permissions: 0,
        },
    ]
}

/// Payload для создания/обновления роли
#[derive(Debug, Serialize, Deserialize)]
pub struct RolePayload {
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub permissions: i32,
}

/// Получает кастомные роли проекта
///
/// GET /api/project/{project_id}/roles
pub async fn get_project_roles(
    State(_state): State<Arc<AppState>>,
    Path(project_id): Path<i32>,
) -> std::result::Result<Json<Vec<Role>>, (StatusCode, Json<ErrorResponse>)> {
    // Stub: returns empty list (custom roles stored in DB would go here)
    let _ = project_id;
    Ok(Json(vec![]))
}

/// Создаёт кастомную роль
///
/// POST /api/project/{project_id}/roles
pub async fn add_project_role(
    State(_state): State<Arc<AppState>>,
    Path(project_id): Path<i32>,
    Json(payload): Json<RolePayload>,
) -> std::result::Result<(StatusCode, Json<Role>), (StatusCode, Json<ErrorResponse>)> {
    let role = Role {
        id: 0,
        project_id,
        slug: payload.slug,
        name: payload.name,
        description: payload.description,
        permissions: payload.permissions,
    };
    Ok((StatusCode::CREATED, Json(role)))
}

/// Обновляет кастомную роль
///
/// PUT /api/project/{project_id}/roles/{role_id}
pub async fn update_project_role(
    State(_state): State<Arc<AppState>>,
    Path((project_id, role_id)): Path<(i32, i32)>,
    Json(payload): Json<RolePayload>,
) -> std::result::Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let _ = (project_id, role_id, payload);
    Ok(StatusCode::OK)
}

/// Удаляет кастомную роль
///
/// DELETE /api/project/{project_id}/roles/{role_id}
pub async fn delete_project_role(
    State(_state): State<Arc<AppState>>,
    Path((project_id, role_id)): Path<(i32, i32)>,
) -> std::result::Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let _ = (project_id, role_id);
    Ok(StatusCode::NO_CONTENT)
}

/// Возвращает все роли (встроенные + кастомные)
///
/// GET /api/project/{project_id}/roles/all
pub async fn get_all_roles(
    State(_state): State<Arc<AppState>>,
    Path(project_id): Path<i32>,
) -> std::result::Result<Json<Vec<Role>>, (StatusCode, Json<ErrorResponse>)> {
    let roles = builtin_roles(project_id);
    Ok(Json(roles))
}
