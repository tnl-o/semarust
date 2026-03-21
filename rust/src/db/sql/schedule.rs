//! Schedule CRUD Operations
//!
//! Операции с расписаниями в SQL

use crate::db::sql::types::SqlDb;
use crate::error::{Error, Result};
use crate::models::{Schedule, ScheduleWithTpl};

impl SqlDb {
    /// Получает расписания проекта
    pub async fn get_schedules(&self, project_id: i32) -> Result<Vec<Schedule>> {
        match unreachable!() {
            
        }
    }

    /// Получает все активные расписания (без фильтра по проекту)
    pub async fn get_all_schedules(&self) -> Result<Vec<Schedule>> {
        match unreachable!() {
            
        }
    }

    /// Получает расписание по ID
    pub async fn get_schedule(&self, project_id: i32, schedule_id: i32) -> Result<Schedule> {
        match unreachable!() {
            
        }
    }

    /// Создаёт расписание
    pub async fn create_schedule(&self, mut schedule: Schedule) -> Result<Schedule> {
        match unreachable!() {
            
        }
    }

    /// Обновляет расписание
    pub async fn update_schedule(&self, schedule: Schedule) -> Result<()> {
        match unreachable!() {
            
        }
    }

    /// Удаляет расписание
    pub async fn delete_schedule(&self, project_id: i32, schedule_id: i32) -> Result<()> {
        match unreachable!() {
            
        }
    }
}
