# 🦀 Semaphore UI на Rust

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.75+-blue.svg)](https://www.rust-lang.org)
[![Tests](https://img.shields.io/badge/tests-475%20passed-brightgreen.svg)]()
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()
[![Migration](https://img.shields.io/badge/migration-100%25-brightgreen.svg)]()
[![Frontend](https://img.shields.io/badge/frontend-Vue%202-brightgreen.svg)]()
[![Production Ready](https://img.shields.io/badge/status-production%20ready-brightgreen.svg)]()

**Полная миграция Semaphore UI на Rust** - высокопроизводительная, безопасная и надёжная система автоматизации для Ansible, Terraform, OpenTofu, Terragrunt, PowerShell и других DevOps-инструментов.

## 🎯 CRUD Демо

> **Попробуйте прямо сейчас!** Интерактивное демо с полным CRUD для всех сущностей.

```bash
# Быстрый старт
./demo-start.sh

# Откройте в браузере
http://localhost/demo-crud.html
```

**Учётные данные:**
- `admin` / `demo123` (администратор)
- `john.doe` / `demo123` (менеджер)
- `jane.smith` / `demo123` (менеджер)
- `devops` / `demo123` (исполнитель)

📖 **Подробная документация**: [CRUD_DEMO.md](CRUD_DEMO.md)

---

## 🚀 Быстрый Старт

### Требования

- Rust 1.75 или новее
- Cargo
- Docker (опционально, для Docker-режимов)

---

### 📋 Режимы запуска

#### Режим 1: Docker Full (Frontend + БД в Docker, Backend на хосте) ⭐ Рекомендуется

```bash
# Запуск всех сервисов
./start.sh docker-full

# Или просто (режим по умолчанию)
./start.sh
```

**Доступ:** http://localhost

**Учётные данные (демо):**
- `admin` / `demo123`
- `john.doe` / `demo123`
- `jane.smith` / `demo123`
- `devops` / `demo123`

**Полезные команды:**
```bash
./start.sh --stop          # Остановить сервисы
./start.sh --logs          # Просмотр логов
./start.sh --clean         # Очистить данные БД
./start.sh --backend       # Запустить только backend
```

---

#### Режим 2: SQLite (минимальные зависимости, для тестирования)

```bash
# Первый запуск с инициализацией
./start.sh sqlite --init

# Запуск сервера
./start.sh sqlite
```

**Доступ:** http://localhost:3000

**Учётные данные:**
- `admin` / `admin123`

**Полезные команды:**
```bash
./start.sh sqlite --stop   # Остановить backend
./start.sh sqlite --build  # Пересобрать backend
```

---

#### Режим 3: Docker All (всё в Docker, продакшен)

```bash
# Запуск всех сервисов в Docker
./start.sh docker-all
```

**Доступ:** http://localhost

**Учётные данные (демо):**
- `admin` / `demo123`

**Полезные команды:**
```bash
./start.sh docker-all --stop   # Остановить сервисы
./start.sh docker-all --logs   # Просмотр логов
./start.sh docker-all --clean  # Очистить volumes
```

---

### 🔧 Ручной запуск (для разработки)

#### Сборка frontend
```bash
./web/build.sh
```

#### Запуск backend напрямую
```bash
cd rust

# С SQLite
export SEMAPHORE_DB_DIALECT=sqlite
export SEMAPHORE_DB_PATH=/tmp/semaphore.db
cargo run -- server --host 0.0.0.0 --port 3000

# С PostgreSQL
export SEMAPHORE_DB_DIALECT=postgres
export SEMAPHORE_DB_URL=postgres://semaphore:semaphore_pass@localhost:5432/semaphore
cargo run -- server --host 0.0.0.0 --port 3000
```

#### Создание администратора
```bash
cd rust
cargo run -- user add \
  --username admin \
  --name "Administrator" \
  --email admin@localhost \
  --password admin123 \
  --admin
```
cd rust
export SEMAPHORE_DB_URL="postgres://semaphore:semaphore_pass@localhost:5432/semaphore"
cargo run -- server --host 0.0.0.0 --port 3000
```

**Доступ:** http://localhost:3000

---

## 📚 Основные команды

```bash
# Запуск сервера
cargo run -- server --host 0.0.0.0 --port 3000

# Создание пользователя
cargo run -- user add --username <name> --email <email> --password <pwd> --admin

# Версия
cargo run -- version

# Тесты
cargo test
```

---

## 📖 Документация

| Документ | Описание |
|----------|----------|
| [CRUD_DEMO.md](CRUD_DEMO.md) | 🎯 CRUD Демо - полное руководство |
| [CONFIG.md](CONFIG.md) | Переменные окружения и конфигурация |
| [API.md](API.md) | REST API документация |
| [AUTH.md](AUTH.md) | Аутентификация и авторизация |
| [DOCKER_DEMO.md](DOCKER_DEMO.md) | Docker демонстрация |
| [scripts/README.md](scripts/README.md) | Скрипты запуска |

---

## 🛠 Технологический Стек

- **Backend:** Rust + Axum + SQLx
- **Frontend:** Vue 2
- **Базы данных:** SQLite, PostgreSQL, MySQL
- **Аутентификация:** JWT + bcrypt

---

## 📝 Лицензия

MIT © [Alexander Vashurin](https://github.com/alexandervashurin)

Оригинальный проект [Semaphore UI](https://github.com/semaphoreui/semaphore) на Go.
