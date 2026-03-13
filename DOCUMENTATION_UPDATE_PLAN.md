# 📋 План обновления документации

## ✅ Обновлённые файлы (актуальные)

| Файл | Статус | Примечание |
|------|--------|------------|
| `README.md` | ✅ Актуален | Обновлены команды запуска |
| `SEMAPHORE_SH.md` | ✅ Актуален | Новая документация для semaphore.sh |
| `SETUP_ENV.md` | ✅ Актуален | Полностью переработан |
| `TROUBLESHOOTING.md` | ✅ Актуален | Обновлены команды |
| `CRUD_DEMO.md` | ✅ Актуален | Обновлён быстрый старт |
| `ЗАПУСК_ДЕМО.md` | ⚠️ Частично | Требует полного обновления |
| `VANILLA_JS_STATUS.md` | ✅ Актуален | 100% миграция |
| `PLAYBOOK_DB_STATUS.md` | ✅ Актуален | Новая документация |

---

## ⚠️ Файлы, требующие обновления

### Критичные (много устаревших ссылок)

1. **DOCKER_DEMO.md** (22 устаревших ссылки)
   - Заменить `./start.sh` → `./semaphore.sh start docker`
   - Заменить `./stop.sh` → `./semaphore.sh stop`
   - Заменить `./start.sh --backend` → удалить (не нужно)

2. **ЗАПУСК_ДЕМО.md** (15 устаревших ссылок)
   - Заменить все команды на `semaphore.sh`

3. **SCRIPTS.md** (30+ устаревших ссылок)
   - Полностью переработать под semaphore.sh
   - Или удалить как устаревший

### Менее критичные

4. **DEMO_DATA.md** (5 ссылок)
   - Заменить `./start.sh` → `./semaphore.sh start`
   - Заменить `./cleanup.sh` → `./semaphore.sh clean`

5. **db/postgres/DEMO_MODE.md** (1 ссылка)
   - Заменить `./start.sh hybrid` → `./semaphore.sh start hybrid`

6. **CRUD_TESTS.md** (2 ссылки)
   - Заменить `./start.sh` → `./semaphore.sh start`

---

## 🗑️ Файлы на удаление

### Удалённые скрипты (документация устарела)

| Удалённый скрипт | Файлы документации |
|------------------|-------------------|
| `stop.sh` | SCRIPTS.md, DOCKER_DEMO.md |
| `cleanup.sh` | SCRIPTS.md, DEMO_DATA.md |
| `setup-env.sh` | SETUP_ENV.md (переработан) |
| `init-demo-db.sh` | - |
| `scripts/start-demo-mode.sh` | README.md (обновлён) |
| `scripts/postgres-cleanup.sh` | - |

---

## 📊 Статистика

### Обновлённые файлы (6)
- README.md
- SEMAPHORE_SH.md (новый)
- SETUP_ENV.md
- TROUBLESHOOTING.md
- CRUD_DEMO.md
- ЗАПУСК_ДЕМО.md (частично)

### Требуют обновления (6)
- DOCKER_DEMO.md (22 правки)
- ЗАПУСК_ДЕМО.md (15 правок)
- SCRIPTS.md (30+ правок)
- DEMO_DATA.md (5 правок)
- db/postgres/DEMO_MODE.md (1 правка)
- CRUD_TESTS.md (2 правки)

### Удалённые файлы (6)
- stop.sh
- cleanup.sh
- setup-env.sh
- init-demo-db.sh
- scripts/start-demo-mode.sh
- scripts/postgres-cleanup.sh

---

## 🎯 Приоритеты обновления

### Высокий приоритет
1. ✅ README.md - главный файл документации
2. ✅ SEMAPHORE_SH.md - новая документация
3. ⚠️ DOCKER_DEMO.md - популярный файл
4. ⚠️ ЗАПУСК_ДЕМО.md - важный для пользователей

### Средний приоритет
5. SCRIPTS.md - можно удалить или переработать
6. DEMO_DATA.md - вспомогательный
7. db/postgres/DEMO_MODE.md - специфичный

### Низкий приоритет
8. CRUD_TESTS.md - для разработчиков
9. Остальные файлы - по мере необходимости

---

## 🔧 Автоматическое обновление

Для массового обновления можно использовать:

```bash
# Поиск устаревших ссылок
grep -r "\./start\.sh" --include="*.md" .

# Поиск устаревших скриптов
grep -r "\./stop\.sh\|\./cleanup\.sh\|\./setup-env\.sh" --include="*.md" .
```

---

## 📝 Рекомендации

1. **Обновить критичные файлы** в первую очередь
2. **Удалить SCRIPTS.md** или полностью переработать
3. **Добавить миграционный гид** для пользователей старых скриптов
4. **Создать changelog** с информацией об изменениях

---

*Последнее обновление: 13 марта 2026 г.*
