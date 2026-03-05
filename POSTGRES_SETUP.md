# Настройка PostgreSQL для Semaphore

## 🎉 Демонстрационное окружение

### Быстрый старт с готовыми данными

Для быстрого знакомства с Semaphore используйте демонстрационное окружение:

```bash
# Запуск PostgreSQL с демонстрационными данными
./scripts/postgres-demo-start.sh
```

**Что включено:**
- ✅ Полная схема БД (16 таблиц)
- ✅ 4 пользователя с готовыми учетными записями
- ✅ 4 проекта с различной тематикой
- ✅ 12 шаблонов задач
- ✅ 5 инвентарей
- ✅ 5 репозиториев
- ✅ 5 окружений
- ✅ 4 расписания
- ✅ 6 задач (выполненные, запущенные, ожидающие)

**Доступ к системе:**
- URL: http://localhost:3000
- Логин: `admin`, `john.doe`, `jane.smith`, `devops`
- Пароль: `demo123` (для всех)

📖 **Подробная документация**: [db/postgres/DEMO.md](db/postgres/DEMO.md)

---

## ⚙️ Текущее состояние поддержки PostgreSQL

Поддержка PostgreSQL **полностью реализована и готова к использованию**!

**Что реализовано:**
- ✅ Парсинг PostgreSQL connection string (`postgres://user:pass@host:port/db`)
- ✅ Файл миграции `db/postgres/init-demo.sql` для инициализации БД
- ✅ Docker Compose для запуска PostgreSQL
- ✅ `SqlStore` поддерживает `PgPool` через sqlx
- ✅ Все методы адаптированы для PostgreSQL ($1 параметры)
- ✅ Демонстрационные данные

## Быстрый старт

### 1. Запуск PostgreSQL

```bash
# Вариант A: С демонстрационными данными (рекомендуется)
./scripts/postgres-demo-start.sh

# Вариант B: Через docker-compose
docker-compose -f docker-compose.postgres.yml up -d

# Вариант C: Через docker run
docker run -d --name semaphore-postgres \
  -e POSTGRES_USER=semaphore \
  -e POSTGRES_PASSWORD=semaphore_pass \
  -e POSTGRES_DB=semaphore \
  -p 5433:5432 \
  postgres:16-alpine
```

### 2. Проверка подключения

```bash
# Проверка готовности PostgreSQL
docker exec semaphore_postgres pg_isready -U semaphore -d semaphore

# Подключение к БД
docker exec -it semaphore_postgres psql -U semaphore -d semaphore

# Показать таблицы
\dt

# Показать пользователей (для demo-окружения)
SELECT id, username, name, email, admin FROM "user";

# Показать проекты
SELECT id, name, type FROM project;

# Показать шаблоны
SELECT t.name, t.playbook, p.name as project
FROM template t
JOIN project p ON t.project_id = p.id;
```

### 3. Запуск Semaphore

Создайте `.env` файл в корне проекта:

```env
# Для демонстрационного окружения
SEMAPHORE_DB_URL=postgres://semaphore:semaphore_pass@localhost:5433/semaphore
SEMAPHORE_HTTP_PORT=3000
SEMAPHORE_WEB_HOST=http://localhost:3000
RUST_LOG=info
```

Запустите сервер:

```bash
cd rust
cargo run -- server
```

Или используйте переменные окружения:

```bash
export SEMAPHORE_DB_URL="postgres://semaphore:semaphore_pass@localhost:5433/semaphore"
cargo run -- server
```

## Connection String

Формат для PostgreSQL:
```
postgres://USER:PASSWORD@HOST:PORT/DB_NAME?OPTIONS
```

**Примеры:**
- Локально: `postgres://semaphore:semaphore_pass@localhost:5433/semaphore?sslmode=disable`
- С таймаутом: `postgres://user:pass@host:5432/db?connect_timeout=10`
- Продакшен: `postgres://user:pass@host:5432/db?sslmode=require`

**Опции:**
- `sslmode=disable` - отключить SSL (для локальной разработки)
- `sslmode=require` - требовать SSL (для продакшена)
- `connect_timeout=10` - таймаут подключения в секундах
- `application_name=semaphore` - имя приложения для логов

## Структура БД

### Полная схема (16 таблиц)

| Таблица | Описание |
|---------|----------|
| `user` | Пользователи (id, username, email, password, admin, totp) |
| `project` | Проекты (id, name, alert, max_parallel_tasks) |
| `project_user` | Связи пользователей с проектами |
| `template` | Шаблоны задач (плейбуки) |
| `inventory` | Инвентари (хосты) |
| `repository` | Git репозитории |
| `environment` | Переменные окружения |
| `access_key` | Ключи доступа (SSH, login/password) |
| `task` | Задачи |
| `task_output` | Вывод задач |
| `schedule` | Расписания |
| `session` | Сессии пользователей |
| `api_token` | API токены |
| `event` | События |
| `option` | Опции системы |
| `migration` | Миграции БД |

### Демонстрационные данные

**Пользователи** (пароль: `demo123`):
| ID | Username | Name | Role |
|----|----------|------|------|
| 1 | admin | Administrator | администратор |
| 2 | john.doe | John Doe | менеджер |
| 3 | jane.smith | Jane Smith | менеджер |
| 4 | devops | DevOps Engineer | исполнитель |

**Проекты:**
1. Demo Infrastructure
2. Web Application Deployment
3. Database Management
4. Security & Compliance

**Шаблоны** (12 шт.):
- Deploy Infrastructure
- Update Servers
- Staging Deploy
- Deploy Web App
- Rollback Web App
- Scale Web App
- Backup Databases
- Restore Database
- DB Health Check
- Security Scan
- Compliance Check
- Patch Security

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
   psql postgres://semaphore:semaphore_pass@localhost:5433/semaphore
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

3. Проверьте что порт свободен:
   ```bash
   lsof -i :5433
   ```

### Ошибка компиляции Rust

Если видите ошибки связанные с `sqlx`:

```bash
# Очистите и пересоберите
cd rust
cargo clean
cargo build

# Проверьте SQLx офлайн режим
cargo sqlx prepare
```

### Сброс демонстрационных данных

Для полного сброса и повторной инициализации:

```bash
# Остановить и удалить данные
docker-compose -f docker-compose.postgres.yml down -v

# Запустить заново
./scripts/postgres-demo-start.sh --clean
```

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
      -c log_min_duration_statement=1000
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

# Запуск тестов
cargo test db::sql::init::tests::test_postgres_connection

# Проверка подключения
cargo run -- migrate --db-url "postgres://semaphore:semaphore_pass@localhost:5433/semaphore"
```

## Примеры SQL запросов

```sql
-- Показать все проекты с количеством шаблонов
SELECT 
  p.id,
  p.name,
  COUNT(t.id) as template_count
FROM project p
LEFT JOIN template t ON p.id = t.project_id
GROUP BY p.id, p.name;

-- Показать последние задачи
SELECT 
  t.id,
  t.status,
  t.created,
  u.name as user_name,
  tmp.name as template_name
FROM task t
JOIN "user" u ON t.user_id = u.id
JOIN template tmp ON t.template_id = tmp.id
ORDER BY t.created DESC
LIMIT 10;

-- Показать расписания
SELECT 
  s.name,
  s.cron,
  s.active,
  p.name as project,
  tmp.name as template
FROM schedule s
JOIN project p ON s.project_id = p.id
JOIN template tmp ON s.template_id = tmp.id;

-- Показать связи пользователей с проектами
SELECT 
  u.username,
  p.name as project,
  pu.role
FROM project_user pu
JOIN "user" u ON pu.user_id = u.id
JOIN project p ON pu.project_id = p.id
ORDER BY p.name, u.username;
```

## Миграции

Миграции применяются автоматически при запуске Semaphore.

Для ручного применения:

```bash
cd rust
cargo run -- migrate --upgrade
```

## Дополнительные ресурсы

- [DEMO.md](db/postgres/DEMO.md) - Документация по демонстрационному окружению
- [db/postgres/README.md](db/postgres/README.md) - Базовая документация PostgreSQL
- [scripts/README.md](scripts/README.md) - Скрипты запуска
- [CONFIG.md](CONFIG.md) - Конфигурация Semaphore
