//! GraphQL типы - минимальная версия

use async_graphql::{SimpleObject, InputObject};

/// Пользователь
#[derive(SimpleObject, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub name: String,
    pub email: String,
    pub admin: bool,
}

/// Проект
#[derive(SimpleObject, Debug, Clone)]
pub struct Project {
    pub id: i32,
    pub name: String,
}

/// Шаблон (Template)
#[derive(SimpleObject, Debug, Clone)]
pub struct Template {
    pub id: i32,
    pub project_id: i32,
    pub name: String,
    pub playbook: String,
}

/// Задача (Task)
#[derive(SimpleObject, Debug, Clone)]
pub struct Task {
    pub id: i32,
    pub template_id: i32,
    pub status: String,
}

/// Input для создания пользователя
#[derive(InputObject, Debug)]
pub struct CreateUserInput {
    pub username: String,
    pub name: String,
    pub email: String,
    pub password: String,
    #[graphql(default = false)]
    pub admin: bool,
}

/// Input для создания проекта
#[derive(InputObject, Debug)]
pub struct CreateProjectInput {
    pub name: String,
}

/// Input для создания шаблона
#[derive(InputObject, Debug)]
pub struct CreateTemplateInput {
    pub project_id: i32,
    pub name: String,
    pub playbook: String,
}
