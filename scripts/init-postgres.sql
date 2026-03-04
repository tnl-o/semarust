-- Таблица пользователей
CREATE TABLE "user" (
    id SERIAL PRIMARY KEY,
    created TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    username VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    admin BOOLEAN NOT NULL DEFAULT FALSE,
    external BOOLEAN NOT NULL DEFAULT FALSE,
    alert BOOLEAN NOT NULL DEFAULT FALSE,
    pro BOOLEAN NOT NULL DEFAULT FALSE,
    totp TEXT,
    email_otp TEXT
);

-- Таблица проектов
CREATE TABLE project (
    id SERIAL PRIMARY KEY,
    created TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    name VARCHAR(255) NOT NULL,
    alert BOOLEAN NOT NULL DEFAULT FALSE,
    alert_chat VARCHAR(255),
    max_parallel_tasks INTEGER NOT NULL DEFAULT 0,
    type VARCHAR(50) NOT NULL DEFAULT 'default',
    default_secret_storage_id INTEGER
);

-- Таблица связи пользователей и проектов
CREATE TABLE project_user (
    id SERIAL PRIMARY KEY,
    project_id INTEGER NOT NULL REFERENCES project(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL,
    created TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(project_id, user_id)
);

-- Таблица миграций
CREATE TABLE migration (
    version BIGINT PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

-- Индексы
CREATE INDEX idx_user_username ON "user"(username);
CREATE INDEX idx_user_email ON "user"(email);
CREATE INDEX idx_project_user_project ON project_user(project_id);
CREATE INDEX idx_project_user_user ON project_user(user_id);
