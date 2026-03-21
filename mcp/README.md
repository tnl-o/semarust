# Velum MCP Server

**AI-native control of Ansible, Terraform and DevOps automation via the Model Context Protocol.**

Connect Claude (or any MCP-compatible AI) to your Velum instance and control infrastructure using natural language.

```
"Запусти деплой backend в prod"           → run_template
"Покажи почему упала последняя задача"    → analyze_task_failure
"Создай расписание backup каждую ночь"    → create_schedule
"Какой health-статус проекта Infrastructure?" → get_project_health
"Выключи runner-2 на обслуживание"        → toggle_runner
```

---

## Возможности (60 инструментов)

| Категория | Инструменты | Уникально |
|---|---|---|
| Projects | list, get, create, update, delete | — |
| Templates | list, get, create, update, delete, **run**, stop_all | — |
| Tasks | list, get, run, stop, output, filter, confirm, reject, bulk_stop | — |
| Inventory | list, get, create, update, delete | — |
| Repositories | list, get, create, update, delete, **branches** | — |
| Environments | list, get, create, update, delete | — |
| Access Keys | list, get, create, delete | — |
| **Schedules** | list, get, create, **toggle**, delete, **validate_cron** | ★ Velum only |
| **Analytics** | project stats, trends, system, **health summary** | ★ Velum only |
| **Runners** | list, status, **toggle**, **clear_cache** | ★ Velum only |
| **Playbooks** | list, get, **run**, **sync**, history | ★ Velum only |
| **Audit** | audit_log, project_events, system_info | ★ Velum only |
| **AI Analysis** | **analyze_task_failure**, **bulk_analyze_failures** | ★★ Velum only |

---

## Быстрый старт

### Вариант 1 — Claude Code (рекомендуется)

```bash
# 1. Получить API токен: Velum → User Settings → API Tokens → + New Token

# 2. Добавить MCP сервер в Claude Code
claude mcp add velum \
  --env VELUM_URL=http://localhost:8088 \
  --env VELUM_API_TOKEN=your-token-here \
  -- uvx velum-mcp

# 3. Проверить подключение
claude "Покажи список проектов Velum"
```

### Вариант 2 — Docker (HTTP режим)

```bash
# 1. Запустить MCP сервер
docker run -d --name velum-mcp \
  --network host \
  -e VELUM_URL=http://localhost:8088 \
  -e VELUM_API_TOKEN=your-token-here \
  -e MCP_TRANSPORT=http \
  -p 8500:8500 \
  ghcr.io/tnl-o/velum-mcp:latest

# 2. Добавить в Claude Code
claude mcp add --transport http velum http://127.0.0.1:8500/mcp
```

### Вариант 3 — Claude Desktop

Редактировать файл конфигурации:
- **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
- **Linux**: `~/.config/claude-desktop/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "velum": {
      "command": "uvx",
      "args": ["velum-mcp"],
      "env": {
        "VELUM_URL": "http://localhost:8088",
        "VELUM_API_TOKEN": "your-token-here"
      }
    }
  }
}
```

### Вариант 4 — Docker Compose (рядом с Velum)

```bash
cd mcp/
cp .env.example .env
# Отредактировать .env — вставить VELUM_URL и VELUM_API_TOKEN
docker compose up -d
```

---

## Конфигурация

| Переменная | Обязательно | Умолчание | Описание |
|---|---|---|---|
| `VELUM_URL` | ✅ | — | URL Velum инстанса, напр. `http://localhost:8088` |
| `VELUM_API_TOKEN` | ✅ | — | API токен из Velum → User Settings |
| `VELUM_TIMEOUT` | — | `30` | Таймаут HTTP запросов в секундах |
| `MCP_TRANSPORT` | — | `stdio` | `stdio` или `http` |
| `MCP_HOST` | — | `0.0.0.0` | Адрес биндинга (только для HTTP) |
| `MCP_PORT` | — | `8500` | Порт (только для HTTP) |

---

## Примеры диалогов с Claude

### Запуск деплоя

```
Пользователь: Запусти деплой backend версии v2.3.1 в prod

Claude: Запускаю... [run_template → project=1, template=5, extra_vars={"version":"v2.3.1"}]
        Задача #1247 запущена. Статус: running.
        Ссылка: http://localhost:8088/task.html?id=1&task=1247
```

### Анализ ошибок

```
Пользователь: Почему упала последняя задача в проекте Infrastructure?

Claude: [get_latest_failed_task → project_id=2]
        Задача #1243 упала 18 минут назад.

        [analyze_task_failure → task_id=1243]

        **Диагноз:**
        Ошибка: `UNREACHABLE! => SSH Error: Permission denied (publickey)`

        **Вероятные причины:**
        1. SSH ключ не настроен для хоста 10.0.1.15 (вероятность: 80%)
        2. Ключ истёк или был сброшен (вероятность: 15%)
        3. Firewall блокирует порт 22 (вероятность: 5%)

        **Рекомендуемые действия:**
        1. Проверьте: ssh -i /path/to/key ansible@10.0.1.15
        2. Убедитесь что публичный ключ есть в ~/.ssh/authorized_keys на хосте
        3. Проверьте настройки ключа в Velum → Keys
```

### Управление расписаниями

```
Пользователь: Создай расписание для backup каждую ночь в 3:00

Claude: [create_schedule → template_id=3, cron="0 3 * * *", name="Ночной backup"]
        Расписание создано: ID 12, активно.
        Следующий запуск: сегодня в 03:00.
```

### Health-статус

```
Пользователь: Какой health-статус у всех проектов?

Claude: [list_projects] → 3 проекта
        [get_project_health x3]

        | Проект        | Запусков/нед | Успешность | Статус    |
        |---|---|---|---|
        | Backend       | 47           | 97.8%      | ✅ healthy |
        | Infrastructure| 12           | 83.3%      | ⚠️ degraded|
        | Terraform Prod| 5            | 60.0%      | 🔴 critical|

        Рекомендую проверить задачи в Terraform Prod — 2 из 5 упали за неделю.
```

---

## Разработка

```bash
# Клонировать репозиторий
git clone https://github.com/tnl-o/velum
cd velum/mcp

# Создать виртуальное окружение
python -m venv .venv
source .venv/bin/activate  # Windows: .venv\Scripts\activate

# Установить зависимости
pip install -e ".[dev]"

# Настроить переменные окружения
cp .env.example .env
# Отредактировать .env

# Запустить в HTTP режиме (для тестирования)
MCP_TRANSPORT=http python -m velum_mcp.server

# Протестировать
curl http://localhost:8500/mcp  # должен вернуть MCP endpoint info
```

---

## Отличие от semaphore-mcp

| Параметр | [cloin/semaphore-mcp](https://github.com/cloin/semaphore-mcp) | Velum MCP |
|---|---|---|
| Инструментов | ~35 | **60** |
| Schedules | ❌ | ✅ |
| Analytics | ❌ | ✅ |
| Runners | ❌ | ✅ |
| Playbooks | ❌ | ✅ |
| Audit log | ❌ | ✅ |
| AI Analysis | ✅ (external LLM API) | ✅ (нативно в контексте Claude) |
| Task confirm/reject | ❌ | ✅ |
| Health summary | ❌ | ✅ |
| Лицензия | AGPL-3.0 | **MIT** |
| Backend | Go Semaphore | **Rust Velum (быстрее)** |

---

## Лицензия

MIT — свободно для коммерческого использования.
