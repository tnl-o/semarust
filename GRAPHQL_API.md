# 📊 GraphQL API

> **GraphQL альтернатива REST API для Semaphore UI**

---

## 🚀 Быстрый старт

### Запуск сервера

```bash
cd rust
cargo run -- server --host 0.0.0.0 --port 3000
```

### GraphiQL Playground

Откройте в браузере: **http://localhost:3000/graphql**

GraphiQL предоставляет интерактивную среду для:
- Изучения схемы
- Выполнения запросов
- Тестирования мутаций

---

## 📚 Схема API

### Query (Чтение данных)

#### `users` - Получить всех пользователей

```graphql
query {
  users {
    id
    username
    name
    email
    admin
  }
}
```

#### `projects` - Получить все проекты

```graphql
query {
  projects {
    id
    name
  }
}
```

#### `templates` - Получить шаблоны проекта

```graphql
query {
  templates(projectId: 1) {
    id
    projectId
    name
    playbook
  }
}
```

#### `tasks` - Получить задачи проекта

```graphql
query {
  tasks(projectId: 1) {
    id
    templateId
    status
  }
}
```

#### `ping` - Проверка доступности

```graphql
query {
  ping
}
```

---

### Mutation (Изменение данных)

#### `ping` - Тестовая мутация

```graphql
mutation {
  ping
}
```

---

### Subscription (Real-time обновления)

#### `taskCreated` - Подписка на создание задач

```graphql
subscription {
  taskCreated {
    # Заглушка - будет реализовано в будущем
  }
}
```

---

## 🔧 Примеры запросов

### Получить всех пользователей и их проекты

```graphql
query {
  users {
    id
    username
    name
  }
  projects {
    id
    name
  }
}
```

### Получить информацию о проекте с шаблонами

```graphql
query GetProjectWithTemplates($projectId: Int!) {
  projects {
    id
    name
  }
  templates(projectId: $projectId) {
    id
    name
    playbook
  }
}
```

**Переменные:**
```json
{
  "projectId": 1
}
```

### Мониторинг задач

```graphql
query GetProjectTasks($projectId: Int!) {
  tasks(projectId: $projectId) {
    id
    templateId
    status
  }
}
```

---

## 🛠️ Расширение API

### Добавление нового Query

1. Откройте `src/api/graphql/query.rs`
2. Добавьте метод в `QueryRoot`:

```rust
#[Object]
impl QueryRoot {
    async fn my_new_query(&self, ctx: &Context<'_>, id: i32) -> Result<MyType> {
        let state = ctx.data::<AppState>()?;
        let store = &state.store;
        
        // Ваша логика
        Ok(...)
    }
}
```

### Добавление нового типа

1. Откройте `src/api/graphql/types.rs`
2. Добавьте тип:

```rust
#[derive(SimpleObject, Debug, Clone)]
pub struct MyType {
    pub id: i32,
    pub name: String,
}
```

---

## 📝 Отличия от REST API

| Характеристика | REST API | GraphQL |
|---------------|----------|---------|
| **Endpoint** | `/api/*` | `/graphql` |
| **Метод** | GET/POST/PUT/DELETE | POST |
| **Получение данных** | Фиксированная структура | Гибкая структура |
| **Over-fetching** | Возможно | Нет |
| **Under-fetching** | Возможно | Нет |
| **Версионирование** | `/api/v2/` | Через схему |
| **Документация** | Swagger/OpenAPI | Introspection |

---

## 🔐 Безопасность

**В текущей версии:**
- ❌ Нет аутентификации
- ❌ Нет авторизации
- ❌ Нет rate limiting

**План реализации:**
1. Добавить JWT аутентификацию через middleware
2. Проверка прав доступа в resolver'ах
3. Rate limiting на уровне GraphQL

---

## 📊 Метрики

| Метрика | Значение |
|---------|----------|
| **Query типов** | 5 |
| **Mutation типов** | 1 |
| **Subscription типов** | 1 |
| **Пользовательских типов** | 4 |

---

## 🐛 Известные ограничения

1. **Только чтение для основных сущностей** - мутации в разработке
2. **Нет pagination** - будет добавлено
3. **Нет фильтрации** - будет добавлено
4. **Нет сортировки** - будет добавлено
5. **Subscription заглушки** - будут реализованы через WebSocket

---

## 🔮 Будущие улучшения

### Q4 2026

- [ ] Полные CRUD мутации
- [ ] Pagination для списков
- [ ] Фильтрация и сортировка
- [ ] Аутентификация и авторизация
- [ ] Rate limiting

### Q1 2027

- [ ] Real-time subscriptions через WebSocket
- [ ] Кэширование запросов
- [ ] Сложные агрегации
- [ ] Batch запросы

---

## 📖 Дополнительные ресурсы

- [GraphQL Specification](https://spec.graphql.org/)
- [async-graphql Documentation](https://async-graphql.github.io/async-graphql/en/index.html)
- [GraphiQL](https://github.com/graphql/graphiql)

---

*Последнее обновление: 10 марта 2026 г.*
