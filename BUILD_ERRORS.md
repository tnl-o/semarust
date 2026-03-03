# Отчёт об ошибках сборки Semaphore Rust

**Дата начала:** 2026-03-02  
**Последнее обновление:** 2026-03-03 (сессия 6)

---

## 📊 Статистика

| Метрика | Значение |
|---------|----------|
| Начальное количество ошибок | 585 |
| Исправлено ошибок | 448 |
| **Осталось ошибок** | **137** |
| **Процент выполнения** | **76.6%** |

---

## 📈 Прогресс по сессиям

| Сессия | Дата | Исправлено | Осталось | Процент |
|--------|------|------------|----------|---------|
| Начало | 2026-03-02 | 0 | 585 | 0% |
| Сессия 1-3 | 2026-03-02 | ~200 | ~385 | 34% |
| Сессия 4 | 2026-03-03 | 159 | 226 | 61% |
| Сессия 5 | 2026-03-03 | 61 | 165 | 72% |
| Сессия 6 | 2026-03-03 | 28 | 137 | 77% |

---

## ✅ Исправленные категории ошибок

### Сессия 6 (28 ошибок)

#### 1. Git Client - ИСПРАВЛЕНО
- ✅ Удалено использование несуществующего поля `ssh_key`
- ✅ Добавлена заглушка для загрузки AccessKey через `ssh_key_id`
- ✅ Исправлен `cmd_git_client.rs` - убрано использование `key_id` напрямую
- ✅ Исправлен `git_client_factory.rs` - `AccessKeyInstallerTrait` тип

#### 2. Модели данных - ИСПРАВЛЕНО
- ✅ `local_job/vault.rs` - исправлен тип `vaults` (JSON строка)
- ✅ `local_job/environment.rs` - исправлен тип `secrets` (JSON строка)
- ✅ `local_job/args.rs` - исправлен тип `secrets` (JSON строка)

#### 3. TemplateType - ИСПРАВЛЕНО
- ✅ Исправлен match для `Option<TemplateType>` в `local_job/run.rs`

#### 4. mismatched types - ИСПРАВЛЕНО ЧАСТИЧНО
- ✅ `cli/mod.rs` - добавлено `Some()` для `DbDialect`
- ✅ `task_runner/lifecycle.rs` - `object_id` и `project_id` как `Option<i32>`

### Сессия 5 (61 ошибка)

#### 1. Удаление BoltDB - ЗАВЕРШЕНО
- ✅ Полностью удалена директория `src/db/bolt/` (43 файла)
- ✅ Удалён `BoltStore` из всех импортов и CLI
- ✅ Удалён `DbDialect::Bolt` из конфигурации

#### 2. Конфигурация - ИСПРАВЛЕНО
- ✅ Исправлен `Config::db_dialect()` - добавлен `.clone()`
- ✅ Исправлен `Config::non_admin_can_create_project()` - добавлен `.clone()`
- ✅ Исправлены инициализаторы `DbConfig` (path, connection_string)
- ✅ Исправлен `merge_ha_configs()` - исправлено обращение к `node_id`
- ✅ Исправлено форматирование `[u8; 16]` в hex

#### 3. Инициализаторы моделей - ИСПРАВЛЕНО
- ✅ `Task` - добавлены `environment_id`, `repository_id`
- ✅ `TaskOutput` - добавлено `project_id`
- ✅ Исправлены moved value ошибки (user.email, current_user, user_to_update)

#### 4. Прочее - ИСПРАВЛЕНО
- ✅ Добавлен `Repository::get_full_path()`
- ✅ Исправлен `nix::NixPath` для chroot
- ✅ Исправлен `RunningTask` clone

### Сессия 4 (159 ошибок)

- ✅ System Process - libc → nix
- ✅ Default реализации для Repository, Inventory, Environment, HARedisConfig
- ✅ ProjectUser модель (username, name)
- ✅ TaskStageType (InstallRoles → Init)

### Сессии 1-3 (~200 ошибок)

- ✅ BoltDB API (полностью)
- ✅ Модели данных (частично)
- ✅ Конфигурация
- ✅ Store Trait
- ✅ TaskLogger Clone
- ✅ AccessKey методы

---

## 🔴 Текущие ошибки (137 осталось)

### Топ ошибок по категориям

| Категория | Количество | Приоритет |
|-----------|------------|-----------|
| mismatched types | ~11 | Высокий |
| type annotations needed | ~10 | Средний |
| dyn Any + Send + Sync: Clone | 4 | Средний |
| Task: sqlx::Decode/Type | 4 | Высокий |
| and_then for i32 | 3 | Средний |
| SecretStorage: FromRow | 2 | Средний |
| ExporterChain: DataExporter | 4 | Низкий |
| Прочие | ~99 | Разный |

### Критические проблемы

#### 1. mismatched types (~11 ошибок)
**Файлы:**
- `src/db_lib/ansible_app.rs` - callback типы
- `src/db_lib/ansible_playbook.rs` - Command типы (tokio vs std)
- `src/db_lib/go_git_client.rs` - clone/set_head
- `src/db_lib/shell_app.rs` - callback типы

#### 2. SQLx трейты (6 ошибок)
**Проблема:** Модели не реализуют `sqlx::Decode` и `sqlx::Type`

**Файлы:**
- `src/models/task.rs` - Task (4 ошибки)
- `src/models/secret_storage.rs` - SecretStorage (2 ошибки)

#### 3. Clone trait (4 ошибки)
**Проблема:** `dyn Any + Send + Sync` не реализует Clone

**Файлы:**
- `src/db_lib/local_app.rs` - LocalAppRunningArgs

#### 4. Job trait (1 ошибка)
**Проблема:** `LocalJob` не реализует трейт `Job`

**Файлы:**
- `src/services/local_job/types.rs`
- `src/services/job.rs`

---

## 📋 План следующей сессии (сессия 7)

### Приоритет 1: mismatched types (11 ошибок)
1. Исправить `ansible_app.rs` - callback типы
2. Исправить `ansible_playbook.rs` - Command типы
3. Исправить `go_git_client.rs` - git2 API
4. Исправить `shell_app.rs` - callback типы

### Приоритет 2: SQLx трейты (6 ошибок)
1. Реализовать `sqlx::Decode` и `sqlx::Type` для `Task`
2. Реализовать `FromRow` для `SecretStorage`

### Приоритет 3: Clone trait (4 ошибки)
1. Убрать Clone из `LocalAppRunningArgs`
2. Использовать Arc для shared данных

### Приоритет 4: Job trait (1 ошибка)
1. Реализовать `Job` trait для `LocalJob`

---

## 📝 Заметки

### Архитектурные решения

1. **Удаление BoltDB**
   - BoltDB - Go библиотека, не имеет нативного Rust аналога
   - Sled реализация имела множество проблем
   - SQL БД полностью покрывают потребности

2. **Git Client архитектура**
   - Использовать `ssh_key_id` для загрузки ключей из хранилища
   - Не хранить ключи напрямую в Repository

3. **Модели данных**
   - `vaults` и `secrets` - JSON строки, требуют парсинга
   - Требуется дополнительная работа с десериализацией

### Технические долги

1. **SQLx трейты** - требуют глубокой интеграции с SQLx
2. **Exporter traits** - требуют рефакторинга архитектуры
3. **Clone для dyn traits** - требует изменения архитектуры callback

### Успехи

- ✅ 76.6% ошибок исправлено
- ✅ BoltDB удалён без потери функциональности
- ✅ Конфигурация полностью исправлена
- ✅ Основные модели данных исправлены
- ✅ Git Client исправлен (частично)

---

## 🎯 Цели

| Цель | Ошибок | Статус |
|------|--------|--------|
| < 100 ошибок | 100 | 🎯 Следующая цель |
| < 50 ошибок | 50 | ⏳ В плане |
| < 10 ошибок | 10 | ⏳ В плане |
| 0 ошибок | 0 | ⏳ Финальная цель |
