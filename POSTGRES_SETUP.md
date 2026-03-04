# Настройка PostgreSQL для Semaphore

## ⚠️ Текущее состояние

Поддержка PostgreSQL находится в процессе разработки. 

**Что уже готово:**
- ✅ Парсинг PostgreSQL connection string (`postgres://user:pass@host:port/db`)
- ✅ Файл миграции `db/postgres/init.sql` для инициализации БД
- ✅ Docker Compose для запуска PostgreSQL
- ✅ Базовая инфраструктура подключения

**Что требует доработки:**
- ❌ `SqlStore` в `rust/src/db/sql/mod.rs` использует только `SqlitePool`
- ❌ Для полноценной поддержки нужно рефакторить `SqlStore` на enum с `PgPool`/`MySqlPool`

## Временное решение

Пока используйте SQLite для разработки:

```bash
# .env
SEMAPHORE_DB_DIALECT=sqlite
SEMAPHORE_DB_PATH=./data/semaphore.db
```

## Для разработчиков

Если вы хотите помочь с реализацией PostgreSQL, нужно:

1. Изменить `SqlStore` на enum:
```rust
pub enum SqlStore {
    Sqlite(SqlitePool),
    Postgres(PgPool),
    MySql(MySqlPool),
}
```

2. Обновить все методы для поддержки разных диалектов
3. Заменить `?` параметры на `$1` для PostgreSQL

---

## Созданные файлы

| Файл | Описание |
|------|----------|
| `db/postgres/init.sql` | Минимальный SQL файл для инициализации БД |
| `docker-compose.postgres.yml` | Docker Compose для быстрого запуска PostgreSQL |
| `db/postgres/README.md` | Подробная документация по PostgreSQL |
| `scripts/postgres-quick-start.sh` | Скрипт быстрого запуска и проверки |

## Быстрый старт

### 1. Запуск PostgreSQL

```bash
# Вариант A: Через docker-compose
docker-compose -f docker-compose.postgres.yml up -d

# Вариант B: Через скрипт
./scripts/postgres-quick-start.sh
```

### 2. Проверка подключения

```bash
docker exec -it semaphore_postgres psql -U semaphore -d semaphore -c "\dt"
```

Должны появиться таблицы:
- `migration`
- `user`
- `project`
- `project_user`

### 3. Запуск Semaphore

Создайте `.env` файл в корне проекта:

```env
SEMAPHORE_DB_URL=postgres://semaphore:semaphore_pass@localhost:5433/semaphore
SEMAPHORE_HTTP_PORT=3000
SEMAPHORE_ADMIN=admin
SEMAPHORE_ADMIN_PASSWORD=changeme
SEMAPHORE_ADMIN_NAME=Administrator
SEMAPHORE_ADMIN_EMAIL=admin@localhost
RUST_LOG=info
```

Запустите сервер:

```bash
cd rust
cargo run -- server
```

## Connection String

Формат для PostgreSQL:
```
postgres://USER:PASSWORD@HOST:PORT/DB_NAME?OPTIONS
```

Примеры:
- Локально: `postgres://semaphore:semaphore_pass@localhost:5432/semaphore?sslmode=disable`
- С таймаутом: `postgres://user:pass@host:5432/db?connect_timeout=10`
- Продакшен: `postgres://user:pass@host:5432/db?sslmode=require`

## Решение проблем

### Ошибка "unable to open database file"

**Причина:** Неправильный формат connection string или PostgreSQL не запущен.

**Решение:**
1. Убедитесь что PostgreSQL запущен:
   ```bash
   docker ps | grep postgres
   ```

2. Проверьте connection string:
   ```bash
   echo $SEMAPHORE_DB_URL
   # Должен быть: postgres://user:pass@host:port/db?options
   ```

3. Проверьте подключение напрямую:
   ```bash
   psql postgres://semaphore:semaphore_pass@localhost:5432/semaphore
   ```

### Ошибка подключения к БД

1. Проверьте логи PostgreSQL:
   ```bash
   docker logs semaphore_postgres
   ```

2. Перезапустите контейнер:
   ```bash
   docker-compose -f docker-compose.postgres.yml restart
   ```

3. Проверьте что порт 5432 не занят:
   ```bash
   lsof -i :5432
   ```

### Ошибка компиляции Rust

Если видите ошибки связанные с `sqlx`:

```bash
# Очистите и пересоберите
cd rust
cargo clean
cargo build
```

## Структура БД

Минимальная схема (`db/postgres/init.sql`) включает:

```sql
migration      -- Таблица версионирования миграций
user           -- Пользователи (id, username, email, password)
project        -- Проекты (id, name)
project_user   -- Связи пользователей с проектами
```

Полная схема применяется автоматически при запуске Semaphore.

## Остановка и очистка

```bash
# Остановить контейнер
docker-compose -f docker-compose.postgres.yml down

# Остановить и удалить данные
docker-compose -f docker-compose.postgres.yml down -v

# Удалить контейнер вручную
docker rm -f semaphore_postgres
```

## Кастомные настройки PostgreSQL

Для продакшена отредактируйте `docker-compose.postgres.yml`:

```yaml
services:
  postgres:
    command: >
      postgres
      -c shared_buffers=256MB
      -c effective_cache_size=1GB
      -c work_mem=16MB
      -c log_statement=ddl
```

Или создайте файл конфигурации и подключите через volume:
```yaml
volumes:
  - ./postgres.conf:/etc/postgresql/postgresql.conf:ro
command: -c config_file=/etc/postgresql/postgresql.conf
```

## Тестирование подключения из Rust

```bash
cd rust
cargo test db::sql::init::tests::test_sqlite_connection
```

Для PostgreSQL тесты требуют запущенный инстанс БД.
