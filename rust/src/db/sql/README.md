# Декомпозиция DB/SQL модуля

## 📊 Обзор

В ходе рефакторинга был декомпозирован монолитный файл `db/sql/mod.rs` (~4500 строк) на модули по типам БД.

## 🎯 Цели декомпозиции

1. **Улучшение читаемости** - файлы по 100-150 строк вместо 4500
2. **Разделение ответственности** - каждый файл отвечает за один диалект БД
3. **Упрощение тестирования** - можно тестировать каждый диалект отдельно
4. **Поддержка расширяемости** - легко добавить новый БД
5. **Параллельная разработка** - разные разработчики могут работать с разными БД

## 📁 Новая структура

```
src/db/sql/
├── mod.rs                  # Главный файл + SqlStore impl
├── types.rs                # SqlDialect, SqlDb типы
├── sqlite/
│   ├── mod.rs              # SQLite модули
│   ├── user.rs             # CRUD пользователей
│   ├── template.rs         # CRUD шаблонов
│   ├── project.rs          # CRUD проектов
│   ├── inventory.rs        # CRUD инвентарей
│   ├── repository.rs       # CRUD репозиториев
│   └── environment.rs      # CRUD окружений
├── postgres/
│   ├── mod.rs              # PostgreSQL модули
│   ├── user.rs             # CRUD пользователей
│   ├── template.rs         # CRUD шаблонов
│   ├── project.rs          # CRUD проектов
│   ├── inventory.rs        # CRUD инвентарей
│   ├── repository.rs       # CRUD репозиториев
│   └── environment.rs      # CRUD окружений
└── mysql/
    ├── mod.rs              # MySQL модули
    ├── user.rs             # CRUD пользователей
    ├── template.rs         # CRUD шаблонов
    ├── project.rs          # CRUD проектов
    ├── inventory.rs        # CRUD инвентарей
    ├── repository.rs       # CRUD репозиториев
    └── environment.rs      # CRUD окружений
```

## 🔧 Архитектура

### Уровень 1: Декомпозированные модули

Каждый модуль содержит чистые функции для работы с конкретным диалектом БД:

```rust
// sqlite/user.rs
pub async fn get_users(pool: &Pool<Sqlite>, params: &RetrieveQueryParams) -> Result<Vec<User>>
pub async fn get_user(pool: &Pool<Sqlite>, user_id: i32) -> Result<User>
pub async fn create_user(pool: &Pool<Sqlite>, user: User) -> Result<User>
pub async fn update_user(pool: &Pool<Sqlite>, user: User) -> Result<()>
pub async fn delete_user(pool: &Pool<Sqlite>, user_id: i32) -> Result<()>
```

### Уровень 2: Адаптеры SqlDb

Файлы-адаптеры (`user_crud.rs`, `template_crud.rs` и т.д.) делегируют вызовы в декомпозированные модули:

```rust
// user_crud.rs
impl SqlDb {
    pub async fn get_users(&self, params: &RetrieveQueryParams) -> Result<Vec<User>> {
        match self.get_dialect() {
            SqlDialect::SQLite => {
                let pool = self.get_sqlite_pool()?;
                sqlite::user::get_users(pool, params).await
            }
            SqlDialect::PostgreSQL => {
                let pool = self.get_postgres_pool()?;
                postgres::user::get_users(pool, params).await
            }
            SqlDialect::MySQL => {
                let pool = self.get_mysql_pool()?;
                mysql::user::get_users(pool, params).await
            }
        }
    }
}
```

### Уровень 3: SqlStore (Store trait)

`SqlStore` реализует трейт `Store` и использует `SqlDb`:

```rust
// mod.rs
impl ProjectStore for SqlStore {
    async fn get_projects(&self, user_id: Option<i32>) -> Result<Vec<Project>> {
        self.db.get_projects(user_id).await
    }
}
```

## 📈 Статистика

| Компонент | Было | Стало | Изменение |
|-----------|------|-------|-----------|
| **mod.rs** | ~4500 строк | ~2500 строк | -44% |
| **Адаптеры** | - | ~650 строк | Новые |
| **Декомпозированные модули** | - | ~2000 строк | Новые |
| **Всего** | ~4500 строк | ~5150 строк | +14% |

**Примечание:** Увеличение общего объёма кода связано с:
- Явным разделением диалектов (раньше был один код для SQLite)
- Улучшенной читаемостью и поддерживаемостью
- Возможностью независимого тестирования

## 🎯 Преимущества

### 1. Читаемость
- Файлы по 100-150 строк вместо 4500
- Чёткая структура по диалектам
- Легко найти нужный код

### 2. Поддержка
- Изменения в одном диалекте не влияют на другие
- Легко добавить новый диалект (просто скопировать модуль)
- Понятная структура для новых разработчиков

### 3. Тестирование
- Можно тестировать каждый диалект отдельно
- Легко моковать pool для тестов
- Изолированные тесты для каждого CRUD

### 4. Расширяемость
```rust
// Добавить новый диалект (например, MariaDB) очень просто:
src/db/sql/
├── mariadb/
│   ├── mod.rs
│   ├── user.rs
│   ├── template.rs
│   └── ...
```

## 📝 Примеры использования

### Получение пользователей (SQLite)
```rust
let pool = sql_db.get_sqlite_pool()?;
let users = sqlite::user::get_users(pool, &params).await?;
```

### Получение пользователей (через SqlDb)
```rust
let users = sql_db.get_users(&params).await?;
```

### Получение пользователей (через Store trait)
```rust
let users = store.get_users(&params).await?;
```

## 🔄 Миграция старого кода

### До декомпозиции
```rust
// mod.rs - огромный файл с SQL запросами
impl SqlDb {
    pub async fn get_users(&self, params: &RetrieveQueryParams) -> Result<Vec<User>> {
        match self.get_dialect() {
            SqlDialect::SQLite => {
                // 50 строк SQL кода
            }
            SqlDialect::PostgreSQL => {
                // 50 строк SQL кода
            }
            // ...
        }
    }
}
```

### После декомпозиции
```rust
// user_crud.rs - тонкий адаптер (~15 строк)
impl SqlDb {
    pub async fn get_users(&self, params: &RetrieveQueryParams) -> Result<Vec<User>> {
        match self.get_dialect() {
            SqlDialect::SQLite => {
                let pool = self.get_sqlite_pool()?;
                sqlite::user::get_users(pool, params).await
            }
            // ...
        }
    }
}

// sqlite/user.rs - чистая реализация (~80 строк)
pub async fn get_users(pool: &Pool<Sqlite>, params: &RetrieveQueryParams) -> Result<Vec<User>> {
    // SQL код
}
```

## ✅ Статус завершения

| Модуль | SQLite | PostgreSQL | MySQL | Статус |
|--------|--------|------------|-------|--------|
| **user** | ✅ | ✅ | ✅ | 100% |
| **template** | ✅ | ✅ | ✅ | 100% |
| **project** | ✅ | ✅ | ✅ | 100% |
| **inventory** | ✅ | ✅ | ✅ | 100% |
| **repository** | ✅ | ✅ | ✅ | 100% |
| **environment** | ✅ | ✅ | ✅ | 100% |

**Общий статус:** ✅ 100% завершено

## 🚀 Следующие шаги

1. **Добавить тесты** для каждого диалекта
2. **Документировать** публичные API модулей
3. **Оптимизировать** повторяющийся код (если возможно)
4. **Рассмотреть** возможность генерации кода для уменьшения дублирования

## 📚 Дополнительные ресурсы

- [SQLx документация](https://docs.rs/sqlx/)
- [Rust Book - Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Clean Architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
