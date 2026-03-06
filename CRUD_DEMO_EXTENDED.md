# 🎯 CRUD Демо - Расширенная версия

## Что нового?

Добавлена поддержка **полного CRUD** для всех сущностей Semaphore UI:

### ✅ Реализованные сущности

| Сущность | Create | Read | Update | Delete | Формы |
|----------|--------|------|--------|--------|-------|
| 📁 Проекты | ✅ | ✅ | ✅ | ✅ | ✅ |
| 📋 Шаблоны | ✅ | ✅ | ✅ | ✅ | ✅ |
| ⚡ Задачи | ✅ | ✅ | - | ✅ | - |
| 🖥️ Инвентарь | ✅ | ✅ | ✅ | ✅ | ✅ |
| 📦 Репозитории | ✅ | ✅ | ✅ | ✅ | ✅ |
| ⚙️ Окружения | ✅ | ✅ | ✅ | ✅ | ✅ |
| 🔑 Ключи доступа | ✅ | ✅ | ✅ | ✅ | ✅ |
| 🕐 Расписания | ✅ | ✅ | ✅ | ✅ | ✅ |
| 📝 События | - | ✅ | - | ✅ | - |

### 🎨 Новые формы

1. **Инвентарь**
   - Название
   - Тип (Static YAML/JSON/File)
   - Данные инвентаря (многострочное поле)
   - SSH login/port

2. **Репозитории**
   - Название
   - Git URL
   - Тип (Git/HTTPS/HTTP/File)
   - Ветвь

3. **Окружения**
   - Название
   - JSON с переменными (многострочное поле с подсветкой)

4. **Ключи доступа**
   - Название
   - Тип (SSH/Login-Password/None)
   - Динамические поля в зависимости от типа
   - SSH ключ или Login/Password

5. **Шаблоны**
   - Название
   - Playbook
   - Описание
   - Выбор инвентаря, репозитория, окружения
   - Опции (allow_override_args_in_task)

6. **Расписания**
   - Название
   - Выбор шаблона
   - Cron выражение
   - Флаг активности

## 🚀 Быстрый старт

```bash
# 1. Запуск
./demo-start.sh

# 2. Запуск backend
./demo-start.sh --backend

# 3. Открыть браузер
http://localhost/demo-crud.html
```

## 📋 Пример рабочего процесса

### Создание инфраструктуры "под ключ"

```
1. Создаём проект
   → "My Web Application"

2. Добавляем SSH ключ
   → Тип: SSH Key
   → Вставляем приватный ключ

3. Создаём инвентарь
   → Тип: Static YAML
   → Добавляем серверы:
      all:
        children:
          webservers:
            hosts:
              web1:
              web2:

4. Добавляем репозиторий
   → URL: https://github.com/myorg/playbooks.git
   → Ветвь: main

5. Создаём окружение
   → JSON: {"env": "production", "domain": "example.com"}

6. Создаём шаблон
   → Playbook: deploy.yml
   → Выбираем: Инвентарь + Репозиторий + Окружение

7. Запускаем задачу
   → Кнопка "Запустить"

8. (Опционально) Добавляем расписание
   → Cron: 0 2 * * * (ежедневно в 2:00)
```

## 🎨 Особенности UI

### Умные формы

- **Динамические поля** - показываются только нужные поля
- **Валидация** - обязательные поля помечены *
- **Подсветка JSON** - для окружений и инвентаря
- **Выпадающие списки** - выбор связанных сущностей

### Уведомления

- ✅ **Success** - зелёные, при успешных операциях
- ❌ **Error** - красные, при ошибках
- ⚠️ **Warning** - жёлтые, предупреждения
- ℹ️ **Info** - синие, информация

### Фильтры

- По проектам (для всех сущностей)
- По статусам (для задач)

## 📊 Статистика

На дашборде отображается:
- Количество проектов
- Количество шаблонов
- Количество задач
- Количество инвентарей
- Количество репозиториев
- Количество окружений
- Количество ключей
- Количество расписаний

## 🔗 Связи между сущностями

```
Project
├── Inventory (N)
├── Repository (N)
├── Environment (N)
├── Access Key (N)
├── Template (N)
│   ├── inventory_id → Inventory
│   ├── repository_id → Repository
│   └── environment_id → Environment
├── Task (N)
│   └── template_id → Template
└── Schedule (N)
    └── template_id → Template
```

## 💡 Примеры данных

### Инвентарь (YAML)

```yaml
all:
  children:
    webservers:
      hosts:
        web1.example.com:
          ansible_user: ansible
        web2.example.com:
          ansible_user: ansible
    databases:
      hosts:
        db1.example.com:
          ansible_user: postgres
```

### Окружение (JSON)

```json
{
  "env": "production",
  "domain": "example.com",
  "ssl_enabled": true,
  "backup_enabled": true,
  "log_level": "warn",
  "workers": 4,
  "cache_enabled": true
}
```

### Расписание (Cron)

```
0 2 * * *      # Ежедневно в 2:00
0 3 * * 0      # Еженедельно в воскресенье в 3:00
*/15 * * * *   # Каждые 15 минут
0 9-17 * * 1-5 # Каждый час с 9 до 17 в будни
0 0 1 * *      # Ежемесячно 1 числа в 0:00
```

## 🧪 Тестирование

```bash
# Проверка демо окружения
./test-demo.sh

# Полный тест CRUD
./test-full-crud.sh
```

## 📖 API Endpoints

### Инвентарь
```
POST   /api/project/{id}/inventory
GET    /api/project/{id}/inventory
PUT    /api/project/{id}/inventory/{iid}
DELETE /api/project/{id}/inventory/{iid}
```

### Репозитории
```
POST   /api/project/{id}/repository
GET    /api/project/{id}/repository
PUT    /api/project/{id}/repository/{rid}
DELETE /api/project/{id}/repository/{rid}
```

### Окружения
```
POST   /api/project/{id}/environment
GET    /api/project/{id}/environment
PUT    /api/project/{id}/environment/{eid}
DELETE /api/project/{id}/environment/{eid}
```

### Ключи
```
POST   /api/project/{id}/keys
GET    /api/project/{id}/keys
PUT    /api/project/{id}/keys/{kid}
DELETE /api/project/{id}/keys/{kid}
```

### Шаблоны
```
POST   /api/project/{id}/templates
GET    /api/project/{id}/templates
PUT    /api/project/{id}/templates/{tid}
DELETE /api/project/{id}/templates/{tid}
```

### Расписания
```
POST   /api/project/{id}/schedule
GET    /api/project/{id}/schedule
PUT    /api/project/{id}/schedule/{sid}
DELETE /api/project/{id}/schedule/{sid}
```

## 🎯 Следующие шаги

### В разработке
- [ ] Редактирование сущностей (полные формы update)
- [ ] Запуск задач из UI
- [ ] Просмотр вывода задач (task output)
- [ ] Управление пользователями проектов
- [ ] Экспорт/импорт конфигураций
- [ ] Массовые операции

### Планы
- [ ] Графический редактор инвентаря
- [ ] Предпросмотр playbook
- [ ] Валидация JSON/YAML
- [ ] История изменений
- [ ] Сравнение версий

## 📚 Документация

- [CRUD_DEMO.md](CRUD_DEMO.md) - Основное руководство
- [CRUD_ENTITIES.md](CRUD_ENTITIES.md) - Структура сущностей
- [API.md](API.md) - Документация API
- [ЗАПУСК_ДЕМО.md](ЗАПУСК_ДЕМО.md) - Инструкция по запуску

## 🐛 Известные ограничения

1. **Редактирование** - кнопки edit показывают уведомление "в разработке"
2. **Запуск задач** - требуется дополнительная реализация
3. **Task Output** - не реализован просмотр
4. **События** - только чтение

## 📞 Поддержка

Возникли вопросы?
- Проверьте документацию
- Посмотрите логи: `./demo-start.sh --logs`
- Запустите тесты: `./test-demo.sh`

**Приятной работы с Semaphore UI! 🚀**
