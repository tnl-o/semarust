# 🐳 Демонстрационное окружение Semaphore UI

Frontend (Nginx) + PostgreSQL с демо-данными. Backend запускается отдельно.

## 🚀 Быстрый старт

### 1. Запуск frontend и БД

```bash
./start.sh
```

### 2. Запуск backend (отдельно)

```bash
# Вариант 1: Через скрипт
./start.sh --backend

# Вариант 2: Напрямую
cd rust
cargo run -- server --host 0.0.0.0 --port 3000
```

## 📋 Доступ к системе

| Компонент | URL | Описание |
|-----------|-----|----------|
| **Frontend** | http://localhost | Nginx раздает статику |
| **Backend API** | http://localhost:3000/api | Rust backend |
| **PostgreSQL** | localhost:5432 | БД с демо-данными |

**Логин/пароль:**
- `admin` / `admin123`
- Демо: `john.doe` / `demo123`, `jane.smith` / `demo123`, `devops` / `demo123`

## 🛠 Команды управления

### Запуск

```bash
# Frontend + БД
./start.sh

# Frontend + БД + Backend
./start.sh --backend

# Пересборка образов
./start.sh --build
```

### Остановка

```bash
# Остановка сервисов
./stop.sh

# Остановка с очисткой БД
./stop.sh --clean
```

### Логи

```bash
# Все сервисы
./start.sh --logs

# Только БД
docker-compose logs -f db

# Только frontend
docker-compose logs -f frontend
```

## 📦 Что входит в стек

| Сервис | Образ | Порт | Назначение |
|--------|-------|------|------------|
| **db** | `postgres:15-alpine` | 5432 | PostgreSQL с демо-данными |
| **frontend** | `nginx:alpine` | 80 | Раздача статики Vue |

**Backend не входит в Docker** - запускается отдельно через `cargo run`.

## 🔧 Требования

- **Docker**: 20.x или новее
- **Docker Compose**: 2.x или новее
- **Rust**: 1.75+ (для запуска backend)

### Установка Docker

```bash
curl -fsSL https://get.docker.com | sh
sudo usermod -aG docker $USER
```

## 🗂 Структура

```
semaphore/
├── docker-compose.yml    # Frontend + PostgreSQL
├── nginx.conf            # Конфигурация Nginx
├── start.sh              # Скрипт запуска
├── stop.sh               # Скрипт остановки
├── rust/                 # Backend (запускается отдельно)
└── web/
    ├── public/           # Скомпилированный frontend
    └── build.sh          # Сборка frontend
```

## 🔍 Диагностика

### Статус сервисов

```bash
docker-compose ps
```

### Проверка БД

```bash
# Проверка готовности
docker-compose exec db pg_isready -U semaphore -d semaphore

# Подключение к БД
docker-compose exec db psql -U semaphore -d semaphore
```

### Проверка frontend

```bash
# Проверка файлов
ls -lh web/public/

# Тест Nginx
curl http://localhost
```

## 💾 Хранение данных

Данные PostgreSQL хранятся в Docker volume `postgres_data`.

### Экспорт БД

```bash
docker-compose exec db pg_dump -U semaphore semaphore > backup.sql
```

### Импорт БД

```bash
docker-compose exec -T db psql -U semaphore semaphore < backup.sql
```

### Сброс данных

```bash
./stop.sh --clean
./start.sh
```

## ⚙️ Конфигурация

### Переменные окружения БД

| Переменная | Значение |
|------------|----------|
| `POSTGRES_DB` | semaphore |
| `POSTGRES_USER` | semaphore |
| `POSTGRES_PASSWORD` | semaphore123 |

### Изменение порта frontend

Отредактируйте `docker-compose.yml`:

```yaml
services:
  frontend:
    ports:
      - "8080:80"  # Измените 80 на нужный порт
```

## 🐛 Решение проблем

### "Cannot connect to the Docker daemon"

```bash
sudo systemctl status docker
sudo systemctl start docker
```

### "Port 80 is already in use"

```bash
# Найдите процесс
lsof -i :80

# Остановите или измените порт в docker-compose.yml
```

### "Backend не подключается к БД"

Проверьте переменные окружения backend:

```bash
export SEMAPHORE_DB_DIALECT=postgres
export SEMAPHORE_DB_HOST=localhost
export SEMAPHORE_DB_PORT=5432
export SEMAPHORE_DB_NAME=semaphore
export SEMAPHORE_DB_USER=semaphore
export SEMAPHORE_DB_PASS=semaphore123
```

### "Frontend показывает ошибку подключения к API"

Убедитесь, что backend запущен:

```bash
# Проверка backend
curl http://localhost:3000/api/health

# Запуск backend
cd rust && cargo run -- server
```

## 📚 Документация

- [README.md](README.md) - основная документация
- [QUICK_START.md](QUICK_START.md) - шпаргалка
- [db/postgres/DEMO.md](db/postgres/DEMO.md) - демо-данные
