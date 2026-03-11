//! Playbook API - заглушка

use axum::{http::StatusCode, Json, response::IntoResponse};
use serde_json::json;

pub async fn get_project_playbooks() -> impl IntoResponse {
    Json(json!({"message": "Используйте /api/project/{id}/inventory для управления playbook"}))
}

pub async fn create_playbook() -> impl IntoResponse {
    Json(json!({"message": "Используйте /api/project/{id}/inventory"}))
}

pub async fn get_playbook() -> impl IntoResponse {
    Json(json!({"message": "Используйте /api/project/{id}/inventory"}))
}

pub async fn update_playbook() -> impl IntoResponse {
    Json(json!({"message": "Используйте /api/project/{id}/inventory"}))
}

pub async fn delete_playbook() -> impl IntoResponse {
    Json(json!({"message": "Используйте /api/project/{id}/inventory"}))
}
