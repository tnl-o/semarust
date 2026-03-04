# План дальнейших работ Semaphore Rust

**Дата:** 2026-03-05  
**Базовое состояние:** Сессия 12 завершена (490 passed, 0 failed, 6 ignored)

---

## Текущее состояние

| Компонент | Статус |
|-----------|--------|
| Сборка lib | ✅ 0 ошибок |
| Тесты | ✅ 490 passed, 0 failed, 6 ignored |
| Сервер | ✅ Запускается |
| API | ✅ Работает |
| Frontend | ✅ Базовая версия (vanilla JS) |

---

## 1. Исправление игнорируемых тестов (приоритет 1)

**Цель:** Убрать `#[ignore]` и добиться прохождения всех 6 тестов.

| Тест | Файл | Проблема | Действие |
|------|------|----------|----------|
| test_verify_recovery_code_normalization | config/config_helpers.rs | Нормализация пробелов не работает | Проверить логику verify_recovery_code, возможно баг в хешировании |
| test_validate_config_empty_tmp_path | config/validator.rs | Config::validate не проверяет tmp_path | Добавить проверку tmp_path в Config::validate или изменить тест |
| test_get_template_params | services/local_job/cli.rs | params не object | Проверить get_template_params, исправить структуру job в тесте |
| test_get_environment_env | services/local_job/environment.rs | env не пустой | Проверить get_environment_env, возможно env содержит дефолтные переменные |
| test_kill_task | services/task_pool_runner.rs | kill_task возвращает Err | Реализовать mock для kill или изменить тест под тестовую среду |
| test_user_add_command | cli/cmd_user.rs | Требует инициализированную БД | Использовать sqlite::memory: в тесте |

---

## 2. Frontend — дополнительные страницы (приоритет 2)

**Цель:** Расширить UI для полной работы с проектом.

| Страница | Описание | API endpoints |
|----------|----------|---------------|
| Задачи (Tasks) | Список задач проекта, статусы, логи | GET /api/project/{id}/tasks |
| Шаблоны (Templates) | CRUD шаблонов, запуск задач | GET/POST /api/project/{id}/templates |
| Инвентарь (Inventory) | Управление инвентарём | GET/POST /api/project/{id}/inventory |
| Репозитории | Список репозиториев | GET /api/project/{id}/repositories |
| Ключи доступа | SSH/другие ключи | GET /api/project/{id}/keys |
| Окружения | Environment variables | GET /api/project/{id}/environment |

**Файлы:** `web/public/index.html`, `web/public/app.js`, `web/public/styles.css`

---

## 3. Обработка ошибок API (приоритет 3)

**Цель:** Единообразные и информативные ответы об ошибках.

- [ ] Стандартизировать формат ErrorResponse (code, message, details)
- [ ] Добавить маппинг Error → HTTP status
- [ ] Логирование ошибок с request_id для отладки
- [ ] Валидация входных данных с детальными сообщениями

**Файлы:** `rust/src/api/middleware/`, `rust/src/error.rs`

---

## 4. Unit-тесты для handlers (приоритет 4)

**Цель:** Покрытие API endpoints тестами.

- [ ] Тесты для auth handlers (login, logout)
- [ ] Тесты для project handlers (list, create, get)
- [ ] Тесты для task handlers
- [ ] Интеграционные тесты с test client (axum::test)

**Подход:** Использовать `axum::test::TestServer` или `tower::ServiceExt`

---

## 5. Очистка warnings (приоритет 5, опционально)

**Цель:** Убрать `#![allow(unused_imports, ...)]` и исправить предупреждения вручную.

- [ ] Удалить allow из lib.rs
- [ ] Исправить unused imports (удалить или использовать)
- [ ] Исправить unused variables (префикс _ или использование)
- [ ] Исправить dead_code (удалить или #[allow] локально)

**Оценка:** ~241 предупреждение в 80+ файлах

---

## 6. Технические долги (низкий приоритет)

| Задача | Описание |
|--------|----------|
| SQLx трейты | Глубокая интеграция Type/Decode для Task и др. |
| Exporter traits | Рефакторинг архитектуры экспорта |
| Clone для dyn traits | Изменение архитектуры callback |
| Дублирование TaskPool | Унификация task_pool.rs и task_pool_types.rs |

---

## Порядок выполнения

| Этап | Задача | Оценка |
|------|--------|--------|
| 1 | Исправить 5 #[ignore] тестов | 1–2 сессии |
| 2 | Frontend: страницы задач, шаблонов, инвентаря | 2–3 сессии |
| 3 | Обработка ошибок API | 0.5 сессии |
| 4 | Unit-тесты для handlers | 1–2 сессии |
| 5 | Очистка warnings | 1 сессия (опционально) |

---

## Чеклист для каждой сессии

1. `cargo build --lib` — сборка без ошибок
2. `cargo test --lib` — все тесты проходят
3. `cargo run -- server` — сервер запускается
4. Обновить BUILD_ERRORS.md при изменении статуса
