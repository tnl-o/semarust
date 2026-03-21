# MASTER_PLAN V3 — Velum: Стать лучше AWX и Ansible Tower

> **Последнее обновление:** 2026-03-21
> **Версия:** 3.0
> **Статус:** АКТИВНЫЙ ПЛАН РАЗРАБОТКИ

---

## Обзор стратегии

Velum — Rust-реализация Semaphore. Цель V3: **стать лучше AWX и Ansible Tower** по всем ключевым параметрам.

### Конкурентная матрица

| Параметр | AWX/Tower | Velum сейчас | Velum V3 цель |
|---|---|---|---|
| Память | 500MB–2GB | ~80MB | ~80MB ✅ |
| Старт | 30–90 сек | <1 сек ✅ | <1 сек ✅ |
| Terraform | 3rd-party плагин | First-class ✅ | First-class ✅ |
| Workflow DAG | ✅ | ❌ | ✅ |
| MCP интеграция | ❌ | ❌ | ✅ |
| AI анализ ошибок | ❌ | ❌ | ✅ |
| Survey формы | ✅ | ❌ | ✅ |
| Drift Detection | ❌ | ❌ | ✅ |
| CLI инструмент | ✅ | ❌ | ✅ |
| Rollback | ❌ | ❌ | ✅ |
| Slack/Teams уведомления | ✅ | ❌ | ✅ |
| LDAP Group Sync | ✅ | ❌ | ✅ |
| Лицензия | GPLv3 / $14K/год | MIT ✅ | MIT ✅ |

---

## Фаза 1 — MCP Server (приоритет: немедленно)

### Что такое MCP и зачем

**Model Context Protocol (MCP)** — открытый протокол от Anthropic для подключения AI-ассистентов (Claude, Cursor, VS Code Copilot) к внешним инструментам. Velum MCP сервер позволяет:

```
"Запусти деплой backend в prod" → Claude → velum_mcp → Velum API → задача запущена
"Покажи последние ошибки в проекте Infrastructure" → Claude → анализ логов + объяснение
"Создай расписание для backup каждую ночь в 3:00" → Claude → velum_mcp → cron создан
```

### Референс: semaphore-mcp

Существующий open-source проект [cloin/semaphore-mcp](https://github.com/cloin/semaphore-mcp) — MCP для Go-оригинала Semaphore (Python, AGPL-3.0). Velum MCP будет:
- Совместим по инструментам (те же имена tools)
- Расширен уникальными Velum-фичами (analytics, runners, schedules, playbooks, drift, AI)
- Лицензия MIT (в отличие от AGPL у оригинала)

### Архитектура Velum MCP

```
┌─────────────────────────────────────────────────────────┐
│  AI Client (Claude Desktop / Claude Code / Cursor)       │
│  "Запусти деплой prod"                                   │
└──────────────────────────────┬──────────────────────────┘
                               │ MCP Protocol (HTTP/stdio)
                               ▼
┌─────────────────────────────────────────────────────────┐
│  velum-mcp (Python, FastMCP)                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │  Projects    │  │  Tasks       │  │  AI Analysis │  │
│  │  Templates   │  │  Schedules   │  │  Drift Check │  │
│  │  Inventory   │  │  Runners     │  │  Notifications│ │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
└──────────────────────────────┬──────────────────────────┘
                               │ REST API (token auth)
                               ▼
┌─────────────────────────────────────────────────────────┐
│  Velum Backend (Rust/Axum) — http://localhost:8088      │
└─────────────────────────────────────────────────────────┘
```

### Инструменты MCP (55 tools)

#### Projects (5)
- `list_projects` — список всех проектов
- `get_project` — детали проекта
- `create_project` — создать проект
- `update_project` — обновить проект
- `delete_project` — удалить проект

#### Templates / Job Templates (7)
- `list_templates` — список шаблонов в проекте
- `get_template` — детали шаблона
- `create_template` — создать шаблон
- `update_template` — обновить шаблон
- `delete_template` — удалить шаблон
- `run_template` — запустить шаблон немедленно
- `stop_all_template_tasks` — остановить все задачи шаблона

#### Tasks (11)
- `list_tasks` — список задач
- `get_task` — детали задачи
- `run_task` — запустить задачу
- `stop_task` — остановить задачу
- `get_task_output` — получить вывод задачи
- `filter_tasks` — фильтрация задач по статусу/шаблону
- `get_latest_failed_task` — последняя упавшая задача
- `get_waiting_tasks` — задачи ожидающие запуска
- `bulk_stop_tasks` — массовая остановка
- `analyze_task_failure` — AI-анализ ошибки задачи
- `bulk_analyze_failures` — анализ нескольких ошибок

#### Inventory (5)
- `list_inventory` — список инвентарей
- `get_inventory` — детали инвентаря
- `create_inventory` — создать инвентарь
- `update_inventory` — обновить инвентарь
- `delete_inventory` — удалить инвентарь

#### Repositories (5)
- `list_repositories` — список репозиториев
- `get_repository` — детали репозитория
- `create_repository` — добавить репозиторий
- `update_repository` — обновить репозиторий
- `delete_repository` — удалить репозиторий

#### Environments (5)
- `list_environments` — список окружений
- `get_environment` — детали окружения
- `create_environment` — создать окружение
- `update_environment` — обновить окружение
- `delete_environment` — удалить окружение

#### Access Keys (4)
- `list_access_keys` — список ключей
- `get_access_key` — детали ключа
- `create_access_key` — создать ключ
- `delete_access_key` — удалить ключ

#### Schedules (5) ★ Уникально для Velum MCP
- `list_schedules` — список расписаний
- `get_schedule` — детали расписания
- `create_schedule` — создать cron расписание
- `toggle_schedule` — включить/выключить расписание
- `delete_schedule` — удалить расписание

#### Analytics (3) ★ Уникально для Velum MCP
- `get_project_analytics` — статистика задач проекта
- `get_system_analytics` — системная статистика
- `get_task_trends` — тренды успешности задач

#### Runners (3) ★ Уникально для Velum MCP
- `list_runners` — список runner-агентов
- `get_runner_status` — статус runner
- `toggle_runner` — включить/выключить runner

#### Playbooks (4) ★ Уникально для Velum MCP
- `list_playbooks` — список playbooks
- `run_playbook` — запустить playbook напрямую
- `sync_playbook` — синхронизировать из Git
- `get_playbook_history` — история запусков

#### Audit & Activity (2) ★ Уникально для Velum MCP
- `get_audit_log` — журнал действий (кто что делал)
- `get_project_events` — события проекта

#### System (1)
- `get_system_info` — версия, uptime, статус БД

**Итого: 60 инструментов** vs 35 у semaphore-mcp

### Файловая структура

```
mcp/
├── README.md                    — документация MCP сервера
├── pyproject.toml               — зависимости (fastmcp, httpx)
├── Dockerfile                   — контейнер для деплоя
├── docker-compose.yml           — запуск с Velum
├── .env.example                 — пример конфигурации
└── src/
    └── velum_mcp/
        ├── __init__.py
        ├── server.py            — точка входа, FastMCP app
        ├── client.py            — HTTP клиент к Velum API
        ├── tools/
        │   ├── __init__.py
        │   ├── projects.py      — CRUD проектов
        │   ├── templates.py     — CRUD шаблонов
        │   ├── tasks.py         — запуск, мониторинг, вывод
        │   ├── inventory.py     — CRUD инвентарей
        │   ├── repositories.py  — CRUD репозиториев
        │   ├── environments.py  — CRUD окружений
        │   ├── keys.py          — CRUD ключей доступа
        │   ├── schedules.py     — управление расписаниями
        │   ├── analytics.py     — метрики и аналитика
        │   ├── runners.py       — управление runner-агентами
        │   ├── playbooks.py     — управление playbooks
        │   ├── audit.py         — журнал аудита
        │   └── system.py        — системная информация
        └── analysis/
            ├── __init__.py
            └── ai_analyzer.py   — AI-анализ ошибок задач
```

### Статус: 🔴 В разработке (Фаза 1)

---

## Фаза 2 — Workflow DAG Builder (v2.2)

### Описание

Визуальный редактор пайплайнов — **главная фича для Enterprise миграции с AWX**.

```
[Terraform Plan] ──success──► [Terraform Apply] ──success──► [Run Ansible]
        │                              │                            │
      failure                        failure                     always
        │                              │                            │
        ▼                              ▼                            ▼
 [Notify Slack]              [Rollback State]              [Send Report]
```

### Backend (Rust)

**Новые таблицы БД:**
```sql
CREATE TABLE workflows (
    id          INTEGER PRIMARY KEY,
    project_id  INTEGER NOT NULL REFERENCES projects(id),
    name        TEXT NOT NULL,
    description TEXT,
    created     DATETIME,
    updated     DATETIME
);

CREATE TABLE workflow_nodes (
    id          INTEGER PRIMARY KEY,
    workflow_id INTEGER NOT NULL REFERENCES workflows(id),
    template_id INTEGER REFERENCES templates(id),
    label       TEXT,
    pos_x       INTEGER DEFAULT 0,
    pos_y       INTEGER DEFAULT 0
);

CREATE TABLE workflow_edges (
    id          INTEGER PRIMARY KEY,
    workflow_id INTEGER NOT NULL REFERENCES workflows(id),
    from_node   INTEGER NOT NULL REFERENCES workflow_nodes(id),
    to_node     INTEGER NOT NULL REFERENCES workflow_nodes(id),
    condition   TEXT NOT NULL CHECK (condition IN ('success','failure','always'))
);

CREATE TABLE workflow_runs (
    id          INTEGER PRIMARY KEY,
    workflow_id INTEGER NOT NULL REFERENCES workflows(id),
    status      TEXT NOT NULL,
    started     DATETIME,
    finished    DATETIME
);
```

**Новые API endpoints:**
```
GET    /api/projects/{id}/workflows
POST   /api/projects/{id}/workflows
GET    /api/projects/{id}/workflows/{wid}
PUT    /api/projects/{id}/workflows/{wid}
DELETE /api/projects/{id}/workflows/{wid}
POST   /api/projects/{id}/workflows/{wid}/run
GET    /api/projects/{id}/workflows/{wid}/runs
GET    /api/projects/{id}/workflow-runs/{rid}
```

### Frontend

**Новая страница:** `web/public/workflow.html`
- Canvas-редактор на SVG/Canvas API (без зависимостей)
- Drag-and-drop нод (шаблонов)
- Рисование стрелок-переходов
- Цветовое кодирование условий (зелёный/красный/серый)
- Live статус выполнения WorkflowRun

### MCP Tool добавить в Фазе 2
- `list_workflows` / `run_workflow` / `get_workflow_status`

### Статус: 🔵 Запланировано

---

## Фаза 3 — Survey (Интерактивные формы) (v2.3)

### Описание

Форма вопросов перед запуском задачи. **Делает автоматизацию self-service** для не-технических пользователей.

```
┌─────────────────────────────────────────────┐
│ 🚀 Запуск: "Deploy Backend"                 │
├─────────────────────────────────────────────┤
│ Версия для деплоя:  [ v2.3.1          ]    │
│ Окружение:         ○ dev ○ staging ● prod   │
│ Количество реплик: [ 3 ]                    │
│ Очистить кеш:      ☑ Да                     │
│                                             │
│              [Отмена]  [🚀 Запустить]       │
└─────────────────────────────────────────────┘
```

### Backend

Новое поле `survey_vars` (JSON) в таблице `templates`:
```json
[
  {"name": "version", "type": "text", "label": "Версия", "required": true, "default": "latest"},
  {"name": "env", "type": "select", "label": "Окружение", "options": ["dev","staging","prod"]},
  {"name": "replicas", "type": "integer", "label": "Реплики", "min": 1, "max": 10, "default": 2},
  {"name": "flush_cache", "type": "boolean", "label": "Очистить кеш", "default": false}
]
```

Значения попадают в `extra_vars` при запуске задачи.

### Статус: 🔵 Запланировано

---

## Фаза 4 — AI-интеграция (v2.4)

### AI Анализ ошибок

При падении задачи — автоматический анализ вывода через LLM:

```
❌ Задача упала → AI анализирует stderr/stdout →
"Ошибка: невозможно подключиться к хосту 10.0.1.5 по SSH.
Возможные причины:
  1. Хост недоступен (проверьте ping 10.0.1.5)
  2. SSH-ключ не настроен для user 'ansible'
  3. Firewall блокирует порт 22
Рекомендуемые действия: ssh -i /path/key ansible@10.0.1.5"
```

### Drift Detection для Terraform

- Фоновый cron job: `terraform plan -detailed-exitcode`
- exitcode=2 → изменения обнаружены → алерт в UI
- Dashboard "Drift Status" по всем Terraform-шаблонам
- Webhook/email уведомление при обнаружении дрейфа

### Настройки AI

```
Системные настройки → AI Integration
  Провайдер:    ○ Anthropic Claude  ○ OpenAI  ○ Ollama (local)
  API Key:      [••••••••••••••••••]
  Модель:       claude-3-haiku-20240307
  Автоанализ:   ☑ При падении задачи автоматически
```

### Статус: 🔵 Запланировано

---

## Фаза 5 — Notification Policies (v2.5)

### Каналы уведомлений

| Канал | Приоритет | Сложность |
|---|---|---|
| Slack | 🔴 Высокий | Низкая (webhook URL) |
| Microsoft Teams | 🔴 Высокий | Низкая (webhook) |
| Email | ✅ Реализовано | — |
| Telegram | ✅ Реализовано | — |
| PagerDuty | 🟠 Средний | Средняя (Events API v2) |
| OpsGenie | 🟠 Средний | Средняя |
| Custom Webhook | 🟡 Низкий | Низкая |

### Политики

```yaml
notification_policies:
  - name: "Prod failures → PagerDuty"
    events: [task_failed]
    filter:
      project: "Production"
      template_tags: [prod]
    channels: [pagerduty]

  - name: "All deploys → Slack #deployments"
    events: [task_started, task_finished]
    channels: [slack_deployments]

  - name: "Weekly report"
    events: [scheduled]
    schedule: "0 9 * * MON"
    channels: [email_team]
    format: analytics_summary
```

### Статус: 🔵 Запланировано

---

## Фаза 6 — LDAP Group Auto-Sync (v2.6)

### Описание

Автоматический маппинг LDAP-групп в команды проектов Velum.

### Конфигурация

```yaml
ldap_group_sync:
  enabled: true
  sync_interval: 300  # секунд
  mappings:
    - ldap_group: "CN=devops,OU=Groups,DC=company,DC=com"
      velum_project: "Infrastructure"
      velum_role: "deploy"
    - ldap_group: "CN=developers,OU=Groups,DC=company,DC=com"
      velum_project: "CI/CD"
      velum_role: "view"
```

### Статус: 🔵 Запланировано

---

## Фаза 7 — CLI Tool: `velum` (v2.7)

### Описание

Командная строка для разработчиков. Интеграция с CI/CD пайплайнами.

### Использование

```bash
# Аутентификация
velum login --url https://velum.company.com --token $VELUM_TOKEN

# Запуск задач
velum run --project "Backend" --template "Deploy" --env VERSION=2.3.1
velum run --project "Backend" --template "Deploy" --wait  # ожидать завершения
velum run --project "Backend" --template "Deploy" --watch # следить за логами

# Статус
velum status                              # все активные задачи
velum task 1234                           # детали задачи
velum logs 1234                           # вывод задачи
velum logs 1234 --follow                  # real-time логи

# Управление ресурсами
velum projects list
velum templates list --project "Backend"
velum schedules list --project "Backend"
velum runners list

# Подтверждение
velum approve 1234                        # подтвердить gated задачу
velum reject 1234 --reason "Not ready"

# GitOps
velum drift check --project "Terraform"  # проверить drift
velum rollback 1234                       # откатить к предыдущему run

# CI/CD интеграция
# .github/workflows/deploy.yml
- name: Deploy via Velum
  run: |
    velum run --project Backend --template Deploy \
      --extra-vars "version=${{ github.sha }}" \
      --wait --timeout 300
    echo "Exit code: $?"
```

### Реализация

Rust binary в `rust/src/cli/velum_cli.rs`, собирается как отдельный бинарник `velum` (отдельно от серверного `semaphore`).

### Статус: 🔵 Запланировано

---

## Фаза 8 — Rollback & Snapshots (v2.8)

### Описание

Каждый успешный запуск шаблона создаёт **snapshot параметров** — фиксирует:
- Git commit SHA (что было задеплоено)
- Инвентарь (на какие хосты)
- Переменные (extra_vars)
- Версия шаблона

Кнопка **"Откатить"** перезапускает задачу с теми же параметрами.

### UI

```
История задач
┌──────────┬──────────────────┬─────────┬────────────┬──────────┐
│ #        │ Шаблон           │ Статус  │ Запущено   │ Действия │
├──────────┼──────────────────┼─────────┼────────────┼──────────┤
│ 1234 ★   │ Deploy Backend   │ ✅ OK   │ 18 мар     │ 🔄 Повтор│
│ 1233     │ Deploy Backend   │ ❌ FAIL │ 17 мар     │ 🔄 Повтор│
│ 1230 ★   │ Deploy Backend   │ ✅ OK   │ 15 мар     │ ↩ Откат  │
└──────────┴──────────────────┴─────────┴────────────┴──────────┘
★ = успешный деплой (snapshot сохранён)
```

### Статус: 🔵 Запланировано

---

## Фаза 9 — Terraform Cost Tracking (v2.9)

### Описание

Интеграция с [Infracost](https://www.infracost.io/) — показывает стоимость изменений ПЕРЕД `terraform apply`.

```
📋 Предпросмотр Terraform Plan — Deploy VPC

  Изменения ресурсов:
  + aws_instance.web[3]     $45.60/мес  (новый)
  ~ aws_rds_cluster.main    $0.00       (изменение тегов)
  - aws_nat_gateway.old     -$32.00/мес (удаление)

  💰 Итого изменение: +$13.60/мес ($163.20/год)

  [❌ Отмена]  [✅ Применить]
```

### Статус: 🔵 Запланировано

---

## Сводная таблица фаз

| Фаза | Версия | Фича | Статус | Квартал |
|---|---|---|---|---|
| 1 | — | **MCP Server** (60 tools) | 🔴 В работе | Q1 2026 |
| 2 | v2.2 | **Workflow DAG Builder** | 🔵 Запланировано | Q2 2026 |
| 3 | v2.3 | **Survey Forms** | 🔵 Запланировано | Q2 2026 |
| 4 | v2.4 | **AI Analysis + Drift Detection** | 🔵 Запланировано | Q3 2026 |
| 5 | v2.5 | **Notification Policies** (Slack/Teams/PagerDuty) | 🔵 Запланировано | Q3 2026 |
| 6 | v2.6 | **LDAP Group Auto-Sync** | 🔵 Запланировано | Q3 2026 |
| 7 | v2.7 | **CLI Tool `velum`** | 🔵 Запланировано | Q4 2026 |
| 8 | v2.8 | **Rollback & Snapshots** | 🔵 Запланировано | Q4 2026 |
| 9 | v2.9 | **Terraform Cost Tracking** | 🔵 Запланировано | Q1 2027 |

---

## Текущее состояние (v2.1.0)

### Что реализовано ✅

- **Бэкенд**: 75+ API endpoints, все тесты зелёные, 0 Clippy warnings
- **Фронтенд**: 28+ HTML страниц, полный feature parity с Go-оригиналом
- **Auth**: JWT, bcrypt, TOTP 2FA, LDAP, OIDC, refresh tokens
- **Task Runner**: реальный запуск ansible/terraform/bash с WebSocket логами
- **Scheduler**: cron-расписания с автозапуском
- **Distributed Runners**: самостоятельная регистрация, health check, теги
- **Analytics**: Chart.js дашборд с трендами
- **Secret Storage**: HashiCorp Vault, DVLS, Fortanix
- **Webhooks**: матчеры, extract values, алиасы
- **Design**: Material Design, Roboto, teal #005057, Font Awesome 6.5
- **Deploy**: Docker (demo/dev/prod), DEB пакет, native binary

### Открытые задачи

- T-BE-15: `exporter_entities.rs` restore пользователей (⏸ dead code, низкий приоритет)

---

## Ссылки

| Репозиторий | URL |
|---|---|
| Velum (origin) | https://github.com/tnl-o/velum |
| Upstream (alexandervashurin) | https://github.com/alexandervashurin/semaphore |
| Go-оригинал (эталон) | https://github.com/velum/velum |
| Semaphore MCP (референс) | https://github.com/cloin/semaphore-mcp |
