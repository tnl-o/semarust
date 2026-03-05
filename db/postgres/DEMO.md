# Демонстрационное окружение Semaphore

## Быстрый старт

### 1. Запуск

```bash
# Запуск PostgreSQL с демонстрационными данными
./scripts/postgres-demo-start.sh
```

### 2. Вход в систему

**URL**: http://localhost:3000

**Учетные данные** (пароль для всех: `demo123`):

| Логин | Имя | Роль |
|-------|-----|------|
| `admin` | Administrator | Администратор (полный доступ) |
| `john.doe` | John Doe | Менеджер проектов |
| `jane.smith` | Jane Smith | Менеджер проектов |
| `devops` | DevOps Engineer | Исполнитель задач |

### 3. Что включено

**Проекты** (4):
- 🏗️ Demo Infrastructure
- 🌐 Web Application Deployment
- 🗄️ Database Management
- 🔒 Security & Compliance

**Шаблоны** (12):
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

**Расписания** (4):
- Weekly Server Update (воскресенье, 03:00)
- Daily Database Backup (ежедневно, 02:00)
- Weekly Security Scan (понедельник, 04:00)
- Daily Compliance Check (ежедневно, 06:00)

**Инвентари**:
- Production Servers (webservers, databases, monitoring)
- Staging Environment
- Web App Cluster (frontend, backend, loadbalancer)
- Database Cluster (PostgreSQL, MySQL)
- Security Scan Targets

**Ключи доступа**:
- Demo SSH Key
- Demo Login/Password
- Web App SSH Key
- DB Admin Key
- Security Audit Key

### 4. Полезные команды

```bash
# Просмотр логов PostgreSQL
docker logs semaphore_postgres

# Подключение к БД
docker exec -it semaphore_postgres psql -U semaphore -d semaphore

# Проверка данных в БД
docker exec -it semaphore_postgres psql -U semaphore -d semaphore -c "SELECT username, name, admin FROM \"user\";"

# Остановка
docker-compose -f docker-compose.postgres.yml down

# Перезапуск с очисткой данных
./scripts/postgres-demo-start.sh --clean
```

### 5. Структура БД

```
semaphore (БД)
├── user (4 пользователя)
├── project (4 проекта)
├── project_user (связи)
├── access_key (5 ключей)
├── inventory (5 инвентарей)
├── repository (5 репозиториев)
├── environment (5 окружений)
├── template (12 шаблонов)
├── schedule (4 расписания)
├── task (6 задач)
├── task_output (вывод задач)
└── event (события)
```

### 6. Примеры запросов

```sql
-- Показать все проекты
SELECT id, name, type, created FROM project;

-- Показать шаблоны для проекта
SELECT t.name, t.playbook, t.description 
FROM template t 
JOIN project p ON t.project_id = p.id 
WHERE p.name = 'Demo Infrastructure';

-- Показать последние задачи
SELECT t.id, t.status, t.created, u.name as user_name, tmp.name as template
FROM task t
JOIN "user" u ON t.user_id = u.id
JOIN template tmp ON t.template_id = tmp.id
ORDER BY t.created DESC
LIMIT 10;

-- Показать расписания
SELECT s.name, s.cron, s.active, p.name as project, tmp.name as template
FROM schedule s
JOIN project p ON s.project_id = p.id
JOIN template tmp ON s.template_id = tmp.id;
```

## Примечания

- Все пароли пользователей: `demo123`
- Данные хранятся в Docker volume `postgres_data`
- Для сброса данных используйте `--clean` флаг
- PostgreSQL доступен на порту 5433
