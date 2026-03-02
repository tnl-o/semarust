# Отчёт об ошибках сборки Semaphore Rust
Дата: 2026-03-02

## Статистика
- **Всего ошибок:** 585
- **Предупреждений:** 277

## Категории ошибок

### 1. Проблемы моделей данных (Model Errors)
#### Missing Fields (E0063)
Множественные структуры инициализированы без обязательных полей:
- `Project` - missing `alert`, `alert_chat`, `default_secret_storage_id`, `type`
- `Template` - missing `arguments`, `build_version`, `start_version`
- `Task` - missing поля для `repository_id`, `environment_id`
- `AccessKey` - missing `key_type`, `login_password`, `access_key`, `environment_id`
- `Inventory` - missing `inventory_type`, `ssh_key_id`, `vaults`, `become_key_id`
- `Repository` - missing `ssh_key_id`, `git_branch`
- `Schedule` - missing `cron_format`, `last_commit_hash`, `repository_id`
- `User` - missing `created`
- `APIToken` - missing `created`
- `ProjectUser` - missing `created`
- `TaskOutput` - missing `stage_id`, `project_id`
- `TaskStage` - missing `project_id`, `stage_type`
- `View` - missing `name`
- `Environment` - missing `env`, `secrets`
- `IntegrationMatcher` - missing `project_id`, `matcher_type`, `matcher_value`
- `IntegrationExtractValue` - missing `project_id`, `value_name`, `value_type`
- `ProjectInvite` - missing `inviter_user_id`, `token`
- `Role` - missing `id`, `project_id`
- `Runner` - `project_id` is `Option<i32>` вместо `i32`

#### Несоответствие типов SQLx Decode/Encode (E0277)
Проблемы с реализацией трейтов для SQLx:
- `UserTotp` - не реализованы `sqlx::Decode`, `sqlx::Type`
- `UserEmailOtp` - не реализованы `sqlx::Decode`, `sqlx::Type`
- `Task` - не реализованы `sqlx::Decode`, `sqlx::Type` (из-за `HashMap<String, JsonValue>`)
- `TaskWithTpl` - не реализован `FromRow`
- `ProjectInvite` - не реализованы `sqlx::Decode`, `sqlx::Type`
- `TemplateType` - `Option<TemplateType>` не реализует `Display`

#### Неправильные типы полей
- `params.offset` - `usize` вместо `Option<usize>`
- `template.template_type` - `Option<TemplateType>` вместо `TemplateType`
- `task.repository_id`, `task.environment_id` - поля отсутствуют
- `inventory.inventory_type` - поле отсутствует, есть `inventory_data`
- `repository.git_branch` - поле отсутствует
- `environment.env` - поле отсутствует, есть `json`
- `access_key.owner` - поле отсутствует
- `schedule.cron_format` - поле отсутствует

### 2. Проблемы trait implementation (Trait Errors)
#### Job trait (E0050)
Метод `Job::run` требует 4 параметра, но реализации имеют 1:
- `LocalJob::run`
- `AnsibleJob::run`
- `TerraformJob::run`
- `ShellJob::run`

#### Store trait (E0599)
- `Box<dyn Store>` не реализует `Clone`
- Отсутствуют методы: `get_project_users`, `get_secret_storages`, `get_secret_storage`, `create_secret_storage`, `update_secret_storage`, `delete_secret_storage`, `get_template_users`, `get_task_alert_chat`

#### LocalApp trait (E0277)
- `AnsibleApp` не реализует `LocalApp`
- `TerraformApp` не реализует `LocalApp`

#### Exporter traits (E0277)
- `ExporterChain` не реализует `DataExporter`
- `ValueMap<T>` не реализует `TypeExporter`

### 3. Проблемы Git клиента (Git Client Errors)
#### GoGitClient implementation (E0195, E0053)
Несоответствие сигнатур методов трейту `GitClient`:
- `clone` - lifetime параметры не совпадают
- `pull` - lifetime параметры не совпадают
- `checkout` - lifetime параметры не совпадают
- `can_be_pulled` - тип параметра `GitRepository` вместо `&GitRepository`
- `get_last_commit_message` - lifetime параметры не совпадают
- `get_last_commit_hash` - lifetime параметры не совпадают
- `get_last_remote_commit_hash` - lifetime параметры не совпадают
- `get_remote_branches` - lifetime параметры не совпадают

#### Missing methods
- `Repository::get_full_path` - метод не найден
- `Template::extract_params` - метод не найден
- `Template::validate` - метод не найден
- `AccessKey::validate` - метод не найден

### 4. Проблемы BoltDB (BoltDB Errors)
#### Missing methods
- `Db::update` - метод не найден
- `Db::view` - метод не найден
- `BoltStore::get_project_user` - метод не найден
- `BoltStore::get_object_refs` - метод не найден

#### Type errors
- `Sized` не реализован для `[u8]` в контексте BoltDB transactions
- `ProjectInviteWithUser` - неправильная структура полей
- `ScheduleWithTpl` - missing `template_name`
- `TemplateWithPerms` - missing `permissions`
- `TaskStageWithResult` - неправильная структура полей

### 5. Проблемы CLI и конфигурации (CLI/Config Errors)
#### Config fields
- `Config::non_admin_can_create_project` - поле отсутствует
- `Config::db_dialect` - поле отсутствует
- `Config::db_path` - поле отсутствует
- `Config::database_url()` - метод не найден
- `DbDialect::PostgreSQL` - варианта нет (есть `Postgres`)

#### Missing dependencies
- `which` crate - не добавлена в Cargo.toml
- `libc` crate - используется но не импортирована

#### Config methods
- `Config::from_env` - метод не найден
- `HARedisConfig::default` - не реализован

### 6. Проблемы API handlers (API Errors)
#### State extractor (E0308)
- `axum::extract::State` используется неправильно
- `state.store.clone()` - `Box<dyn Store>` не реализует `Clone`

#### RetrieveQueryParams (E0308)
- Неправильное использование в методах store
- `api::users::RetrieveQueryParams` vs `store::RetrieveQueryParams`

#### Method signature mismatches
- `get_events` - неправильные параметры (limit: usize вместо RetrieveQueryParams)
- `get_access_keys` - лишние параметры
- `get_integrations` - лишние параметры
- `get_options` - лишние параметры
- `get_template` - missing `project_id` параметр

### 7. Проблемы сервисов (Service Errors)
#### Task Runner
- `Job` trait требует 4 параметра в `run`
- `LocalJob` не реализует `Job`
- `RunningTask` не реализует `Clone`
- `TaskLogger` не реализует `Clone`

#### Backup/Restore
- `BackupFormat` поля не соответствуют моделям
- `RestoreDB` поля не соответствуют моделям
- Асинхронные методы вызываются в синхронном контексте

#### Exporter
- `ExporterChain` не реализует требуемые трейты
- `ValueMap<T>` не реализует `TypeExporter`

### 8. Проблемы TemplateType (TemplateType Errors)
#### Missing variants
- `TemplateType::Ansible` - варианта нет
- `TemplateType::Terraform` - варианта нет
- `TemplateType::Shell` - варианта нет
- `TemplateType::Task` - варианта нет
- `TemplateType::Deploy` - варианта нет
- `TemplateType::Build` - варианта нет

### 9. Проблемы AccessKey (AccessKey Errors)
#### Missing variants
- `AccessKeyType::Ssh` - варианта нет (есть `SSH`)
- `AccessKeyOwner::Shared` - варианта нет

#### Missing fields
- `key_type` - поле отсутствует
- `login_password` - поле отсутствует
- `access_key` - поле отсутствует
- `environment_id` - поле отсутствует
- `owner` - поле отсутствует
- `override_secret` - поле отсутствует
- `created` - поле отсутствует

### 10. Проблемы Task (Task Errors)
#### Missing fields
- `repository_id` - поле отсутствует
- `environment_id` - поле отсутствует
- `params` - тип `Option<HashMap<String, Value>>` вместо ожидаемого

### 11. Проблемы FFI (FFI Errors)
#### Store boxing (E0277)
- `Box<dyn Store>` не может быть преобразован в `Box<dyn Store + Send + Sync>`
- `Arc<dyn Store>` не может быть преобразован в `Box<dyn Store>`

### 12. Проблемы SQLx типов (SQLx Type Errors)
#### HashMap encoding
- `HashMap<String, JsonValue>` не реализует `sqlx::Encode`, `sqlx::Type`

#### Option<usize> formatting
- `Option<usize>` не реализует `Display` для format!()

### 13. Проблемы Ansible/Terraform (Ansible/Terraform Errors)
#### Missing fields
- `TerraformTaskParams::backend_init_required` - поле отсутствует
- `TerraformTaskParams::backend_config` - поле отсутствует
- `TerraformTaskParams::workspace` - поле отсутствует
- `Inventory::variables` - поле отсутствует
- `Template::hooks` - поле отсутствует
- `Template::params` - поле отсутствует
- `Repository::ssh_key` - поле отсутствует

#### Type mismatches
- `tokio::process::Command` vs `std::process::Command`
- Callback типы не совпадают

### 14. Проблемы Project Invite (Project Invite Errors)
#### Missing fields
- `ProjectInvite::token` - поле отсутствует
- `ProjectInvite::inviter_user_id` - поле отсутствует
- `ProjectInviteWithUser` - неправильная структура

### 15. Проблемы Schedule (Schedule Errors)
#### Missing fields
- `Schedule::cron_format` - поле отсутствует
- `Schedule::last_commit_hash` - поле отсутствует
- `Schedule::repository_id` - поле отсутствует

### 16. Проблемы View (View Errors)
#### Missing fields
- `View::name` - поле отсутствует (есть `title`)

### 17. Проблемы Environment (Environment Errors)
#### Missing fields
- `Environment::env` - поле отсутствует (есть `json`)
- `Environment::secrets` - поле отсутствует

### 18. Проблемы Integration (Integration Errors)
#### Missing fields
- `IntegrationMatcher::project_id` - поле отсутствует
- `IntegrationMatcher::matcher_type` - поле отсутствует
- `IntegrationMatcher::matcher_value` - поле отсутствует
- `IntegrationExtractValue::project_id` - поле отсутствует
- `IntegrationExtractValue::value_name` - поле отсутствует
- `IntegrationExtractValue::value_type` - поле отсутствует

### 19. Проблемы Role (Role Errors)
#### Missing fields
- `Role::id` - поле отсутствует
- `Role::project_id` - поле отсутствует

### 20. Проблемы Runner (Runner Errors)
#### Option type
- `Runner::project_id` - `Option<i32>` вместо `i32`

### 21. Проблемы Async/Sync (Async/Sync Errors)
#### Sync bound
- future cannot be sent between threads safely (BoltDB filter)

#### Async in sync context
- `restore()` методы синхронные, но вызывают асинхронные store методы

### 22. Проблемы Clone trait (Clone Errors)
- `RunningTask` не реализует `Clone`
- `TaskLogger` не реализует `Clone`
- `Box<dyn Store>` не реализует `Clone`
- `Box<dyn TaskLogger>` не реализует `Clone`
- `AccessKeyInstallerImpl` не реализует `Clone`

### 23. Проблемы форматирования (Formatting Errors)
#### Display trait
- `Option<TemplateType>` не реализует `Display`
- `[u8; 16]` не реализует `UpperHex`/`LowerHex`
- `Option<usize>` не реализует `Display`

### 24. Missing crate dependencies
- `which` - не добавлена
- `libc` - используется но не импортирована явно

### 25. Проблемы пропущенных полей в инициализаторах
Множественные структуры инициализированы с неправильным набором полей.
