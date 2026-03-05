# PostgreSQL для Semaphore

## Быстрый старт с Docker Compose

### 1. Запуск PostgreSQL с демонстрационными данными

```bash
# Вариант 1: Использование скрипта (рекомендуется)
./scripts/postgres-demo-start.sh

# Вариант 2: Вручную через docker-compose
docker-compose -f docker-compose.postgres.yml up -d
```

Это создаст:
- Контейнер `semaphore_postgres` на порту **5433** (чтобы не конфликтовал с 5432)
- БД `semaphore` с пользователем `semaphore` / паролем `semaphore_pass`
- Применит полную схему и демонстрационные данные из `db/postgres/init-demo.sql`

### 2. Проверка подключения

```bash
docker exec -it semaphore_postgres psql -U semaphore -d semaphore
```

Внутри psql:
```sql
\dt              # показать таблицы
SELECT * FROM "user";  # показать пользователей
SELECT * FROM project;  # показать проекты
SELECT * FROM template;  # показать шаблоны
\q               # выйти
```

### 3. Демонстрационные данные

База данных инициализируется следующими данными:

**Пользователи** (пароль для всех: `demo123`):
| Username | Name | Role |
|----------|------|------|
| admin | Administrator | администратор |
| john.doe | John Doe | менеджер |
| jane.smith | Jane Smith | менеджер |
| devops | DevOps Engineer | исполнитель |

**Проекты**:
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
- Backup Databases
- Security Scan
- и другие...

**Расписания** (4 шт.):
- Weekly Server Update
- Daily Database Backup
- Weekly Security Scan
- Daily Compliance Check

### 4. Запуск Semaphore с PostgreSQL

**Автоматический запуск** (рекомендуется):

```bash
./scripts/postgres-demo-start.sh
```

Скрипт автоматически:
- Запустит PostgreSQL с демонстрационными данными
- Создаст `.env` файл
- Проверит готовность БД

**Ручной запуск**:

Создайте `.env` файл:

```env
SEMAPHORE_DB_DIALECT=postgres
SEMAPHORE_DB_HOST=localhost
SEMAPHORE_DB_PORT=5433
SEMAPHORE_DB_USER=semaphore
SEMAPHORE_DB_PASS=semaphore_pass
SEMAPHORE_DB_NAME=semaphore

SEMAPHORE_HTTP_PORT=3000
SEMAPHORE_ADMIN=admin
SEMAPHORE_ADMIN_PASSWORD=changeme
SEMAPHORE_ADMIN_NAME=Administrator
SEMAPHORE_ADMIN_EMAIL=admin@localhost

RUST_LOG=info
```

Запустите сервер:

```bash
cargo run -- server
```

Или с готовым connection string:

```bash
export DATABASE_URL="postgres://semaphore:semaphore_pass@localhost:5433/semaphore"
cargo run -- server
```

## Connection String формат

```
postgres://USER:PASSWORD@HOST:PORT/DB_NAME?OPTIONS
```

Примеры опций для PostgreSQL:
- `sslmode=disable` - отключить SSL (для локальной разработки)
- `connect_timeout=10` - таймаут подключения в секундах
- `application_name=semaphore` - имя приложения для логов PostgreSQL

Полный пример:
```
postgres://semaphore:semaphore_pass@localhost:5432/semaphore?sslmode=disable&connect_timeout=10
```

## Кастомные настройки PostgreSQL

Для продакшена рекомендуется настроить в `postgresql.conf`:

```conf
# Память
shared_buffers = 256MB
effective_cache_size = 1GB
work_mem = 16MB

# Логирование
log_statement = 'ddl'
log_min_duration_statement = 1000

# Подключения
max_connections = 100
```

И в `pg_hba.conf` настроить доступ:

```conf
# Локальный доступ
local   semaphore   semaphore                       trust
# IPv4
host    semaphore   semaphore   127.0.0.1/32        md5
# IPv6
host    semaphore   semaphore   ::1/128             md5
```

## Остановка и очистка

```bash
# Остановить контейнер
docker-compose -f docker-compose.postgres.yml down

# Остановить и удалить данные (volume)
docker-compose -f docker-compose.postgres.yml down -v

# Очистить и перезапустить демонстрационные данные
./scripts/postgres-demo-start.sh --clean
```

## Структура БД

Полная схема включает следующие таблицы:

| Таблица | Описание |
|---------|----------|
| `user` | Пользователи системы |
| `project` | Проекты |
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

Демонстрационные данные включают:
- 4 пользователя
- 4 проекта
- 5 ключей доступа
- 5 инвентарей
- 5 репозиториев
- 5 окружений
- 12 шаблонов
- 4 расписания
- 6 задач (включая выполненные и запущенные)

## Troubleshooting

### Ошибка "unable to open database file"

Убедитесь что:
1. PostgreSQL запущен: `docker ps | grep postgres`
2. Connection string правильный: `postgres://user:pass@host:port/dbname`
3. БД существует: `psql -h localhost -U semaphore -l | grep semaphore`

### Ошибка подключения

Проверьте логи PostgreSQL:
```bash
docker logs semaphore_postgres
```

Пересоздайте контейнер:
```bash
docker-compose -f docker-compose.postgres.yml down -v
docker-compose -f docker-compose.postgres.yml up -d
```
