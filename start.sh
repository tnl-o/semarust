#!/bin/bash
# ============================================================================
# Semaphore UI - Универсальный скрипт запуска
# ============================================================================
# Поддерживаемые режимы:
#   1. docker-full    - Frontend + PostgreSQL в Docker + Backend на хосте
#   2. sqlite         - SQLite + Backend на хосте (минимальные зависимости)
#   3. docker-all     - Всё в Docker (Frontend + PostgreSQL + Backend)
# ============================================================================
# Использование: ./start.sh [РЕЖИМ] [ОПЦИИ]
#
# Режимы:
#   docker-full     Frontend + БД в Docker, Backend на хосте (по умолчанию)
#   sqlite          SQLite + Backend на хосте (для тестирования)
#   docker-all      Всё в Docker (продакшен)
#
# Опции:
#   --stop          Остановить все сервисы
#   --restart       Перезапустить сервисы
#   --clean         Очистить volumes/данные
#   --logs          Показать логи
#   --build         Пересобрать образы/бинарник
#   --backend       Запустить только backend (для docker-full)
#   --init          Инициализировать БД (создать админа)
#   --help, -h      Показать эту справку
# ============================================================================

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPOSE_FILE="$SCRIPT_DIR/docker-compose.yml"
COMPOSE_FULL_FILE="$SCRIPT_DIR/docker-compose.full.yml"
ENV_FILE="$SCRIPT_DIR/.env"
LOG_FILE="$SCRIPT_DIR/logs/start.log"

# Цвета для вывода
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m'

# Режим по умолчанию
MODE="${1:-docker-full}"
shift 2>/dev/null || true

# Флаги
STOP_MODE=false
RESTART_MODE=false
CLEAN_MODE=false
LOGS_MODE=false
BUILD_MODE=false
BACKEND_ONLY=false
INIT_MODE=false

# ============================================================================
# Функции вывода
# ============================================================================

info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

success() {
    echo -e "${GREEN}[OK]${NC} $1"
}

warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

step() {
    echo -e "${CYAN}➜${NC} $1"
}

# ============================================================================
# Проверка зависимостей
# ============================================================================

check_docker() {
    if ! command -v docker &> /dev/null; then
        error "Docker не найден. Установите Docker: https://docs.docker.com/get-docker/"
    fi
    
    if command -v docker-compose &> /dev/null; then
        COMPOSE_CMD="docker-compose"
    elif docker compose version &> /dev/null 2>&1; then
        COMPOSE_CMD="docker compose"
    else
        error "Docker Compose не найден"
    fi
    
    success "Docker и Docker Compose найдены ($COMPOSE_CMD)"
}

check_rust() {
    if ! command -v cargo &> /dev/null; then
        error "Rust/Cargo не найдены. Установите Rust: https://rustup.rs/"
    fi
    success "Rust найден ($(cargo --version))"
}

check_frontend() {
    if [ ! -f "$SCRIPT_DIR/web/public/app.js" ] || [ ! -s "$SCRIPT_DIR/web/public/app.js" ]; then
        return 1
    fi
    return 0
}

build_frontend() {
    step "Сборка frontend..."
    if [ -f "$SCRIPT_DIR/web/build.sh" ]; then
        "$SCRIPT_DIR/web/build.sh"
    else
        error "Скрипт web/build.sh не найден"
    fi
    success "Frontend собран"
}

build_backend() {
    step "Сборка backend..."
    cd "$SCRIPT_DIR/rust"
    cargo build --release
    success "Backend собран"
}

# ============================================================================
# Управление .env файлом
# ============================================================================

create_env_docker_full() {
    step "Создание .env для режима docker-full..."
    cat > "$ENV_FILE" << 'EOF'
# ============================================================================
# Semaphore UI - Конфигурация (Docker Full: Frontend+БД в Docker, Backend на хосте)
# ============================================================================

# База данных - PostgreSQL
SEMAPHORE_DB_DIALECT=postgres
SEMAPHORE_DB_URL=postgres://semaphore:semaphore_pass@localhost:5432/semaphore

# JWT Secret (генерируется автоматически при первом запуске)
SEMAPHORE_JWT_SECRET=

# Веб-интерфейс
SEMAPHORE_WEB_PATH=./web/public

# Временная директория
SEMAPHORE_TMP_PATH=/tmp/semaphore

# Порт сервера
SEMAPHORE_TCP_ADDRESS=0.0.0.0:3000

# Логирование
RUST_LOG=info

# Демо режим (опционально)
# SEMAPHORE_DEMO_MODE=true
EOF
    success ".env создан"
}

create_env_sqlite() {
    step "Создание .env для режима sqlite..."
    cat > "$ENV_FILE" << 'EOF'
# ============================================================================
# Semaphore UI - Конфигурация (SQLite)
# ============================================================================

# База данных - SQLite
SEMAPHORE_DB_DIALECT=sqlite
SEMAPHORE_DB_PATH=/tmp/semaphore.db

# JWT Secret (генерируется автоматически при первом запуске)
SEMAPHORE_JWT_SECRET=

# Веб-интерфейс
SEMAPHORE_WEB_PATH=./web/public

# Временная директория
SEMAPHORE_TMP_PATH=/tmp/semaphore

# Порт сервера
SEMAPHORE_TCP_ADDRESS=0.0.0.0:3000

# Логирование
RUST_LOG=info
EOF
    success ".env создан"
}

create_env_docker_all() {
    step "Создание .env для режима docker-all..."
    cat > "$ENV_FILE" << 'EOF'
# ============================================================================
# Semaphore UI - Конфигурация (Docker All: всё в Docker)
# ============================================================================

# База данных - PostgreSQL (внутри Docker сети)
SEMAPHORE_DB_DIALECT=postgres
SEMAPHORE_DB_URL=postgres://semaphore:semaphore_pass@db:5432/semaphore

# JWT Secret (генерируется автоматически при первом запуске)
SEMAPHORE_JWT_SECRET=

# Веб-интерфейс
SEMAPHORE_WEB_PATH=/app/web/public

# Временная директория
SEMAPHORE_TMP_PATH=/tmp/semaphore

# Порт сервера
SEMAPHORE_TCP_ADDRESS=0.0.0.0:3000

# Логирование
RUST_LOG=info
EOF
    success ".env создан"
}

# ============================================================================
# Режим 1: Docker Full (Frontend + БД в Docker, Backend на хосте)
# ============================================================================

mode_docker_full() {
    info "Режим: docker-full (Frontend + БД в Docker, Backend на хосте)"
    
    # Создание .env
    create_env_docker_full
    
    # Проверка зависимостей
    check_docker
    check_rust
    
    # Проверка и сборка frontend
    if ! check_frontend; then
        build_frontend
    else
        success "Frontend уже собран"
    fi
    
    # Запуск Docker сервисов
    if [ "$STOP_MODE" = true ]; then
        step "Остановка Docker сервисов..."
        $COMPOSE_CMD -f "$COMPOSE_FILE" down
        success "Сервисы остановлены"
        return
    fi
    
    if [ "$CLEAN_MODE" = true ]; then
        step "Очистка volumes..."
        $COMPOSE_CMD -f "$COMPOSE_FILE" down -v
        success "Volumes очищены"
        return
    fi
    
    if [ "$LOGS_MODE" = true ]; then
        $COMPOSE_CMD -f "$COMPOSE_FILE" logs -f
        return
    fi
    
    if [ "$RESTART_MODE" = true ]; then
        step "Перезапуск сервисов..."
        $COMPOSE_CMD -f "$COMPOSE_FILE" restart
        success "Сервисы перезапущены"
        return
    fi
    
    # Запуск сервисов
    step "Запуск PostgreSQL и Frontend..."
    $COMPOSE_CMD -f "$COMPOSE_FILE" down --remove-orphans 2>/dev/null || true
    $COMPOSE_CMD -f "$COMPOSE_FILE" up -d --remove-orphans
    
    # Ожидание готовности PostgreSQL
    wait_for_postgres
    
    success "Docker сервисы запущены"
    
    # Запуск backend
    if [ "$BACKEND_ONLY" = true ] || [ "$INIT_MODE" = false ]; then
        start_backend
    fi
    
    print_status_docker_full
}

# ============================================================================
# Режим 2: SQLite (минимальные зависимости)
# ============================================================================

mode_sqlite() {
    info "Режим: sqlite (SQLite + Backend на хосте)"
    
    # Создание .env
    create_env_sqlite
    
    # Проверка зависимостей
    check_rust
    
    # Проверка и сборка frontend
    if ! check_frontend; then
        warning "Frontend не собран"
        if [ "$BUILD_MODE" = true ]; then
            build_frontend
        else
            warning "Запуск без frontend (только API)"
        fi
    else
        success "Frontend уже собран"
    fi
    
    # Сборка backend если нужно
    if [ "$BUILD_MODE" = true ]; then
        build_backend
    fi
    
    # Остановка Docker если запущен
    if [ "$STOP_MODE" = true ]; then
        step "Остановка backend..."
        pkill -f "semaphore server" 2>/dev/null || true
        success "Backend остановлен"
        return
    fi
    
    # Инициализация БД
    if [ "$INIT_MODE" = true ]; then
        init_sqlite_db
        return
    fi
    
    # Запуск backend
    start_backend_sqlite
    
    print_status_sqlite
}

# ============================================================================
# Режим 3: Docker All (всё в Docker)
# ============================================================================

mode_docker_all() {
    info "Режим: docker-all (всё в Docker)"
    
    # Создание .env
    create_env_docker_all
    
    # Проверка зависимостей
    check_docker
    
    # Проверка и сборка frontend
    if ! check_frontend; then
        build_frontend
    else
        success "Frontend уже собран"
    fi
    
    # Сборка backend если нужно
    if [ "$BUILD_MODE" = true ]; then
        build_backend
    fi
    
    # Остановка Docker
    if [ "$STOP_MODE" = true ]; then
        step "Остановка Docker сервисов..."
        $COMPOSE_CMD -f "$COMPOSE_FULL_FILE" down
        success "Сервисы остановлены"
        return
    fi
    
    if [ "$CLEAN_MODE" = true ]; then
        step "Очистка volumes..."
        $COMPOSE_CMD -f "$COMPOSE_FULL_FILE" down -v
        success "Volumes очищены"
        return
    fi
    
    if [ "$LOGS_MODE" = true ]; then
        $COMPOSE_CMD -f "$COMPOSE_FULL_FILE" logs -f
        return
    fi
    
    if [ "$RESTART_MODE" = true ]; then
        step "Перезапуск сервисов..."
        $COMPOSE_CMD -f "$COMPOSE_FULL_FILE" restart
        success "Сервисы перезапущены"
        return
    fi
    
    # Запуск сервисов
    step "Запуск всех сервисов..."
    $COMPOSE_CMD -f "$COMPOSE_FULL_FILE" down --remove-orphans 2>/dev/null || true
    $COMPOSE_CMD -f "$COMPOSE_FULL_FILE" up -d --remove-orphans --build
    
    # Ожидание готовности
    wait_for_postgres
    sleep 5
    
    success "Все сервисы запущены"
    
    print_status_docker_all
}

# ============================================================================
# Вспомогательные функции
# ============================================================================

wait_for_postgres() {
    step "Ожидание готовности PostgreSQL..."
    local max_attempts=30
    local attempt=1
    
    while [ $attempt -le $max_attempts ]; do
        if docker exec semaphore-db pg_isready -U semaphore -d semaphore &> /dev/null 2>&1; then
            success "PostgreSQL готов"
            return 0
        fi
        sleep 1
        ((attempt++))
    done
    
    error "PostgreSQL не запустился за $max_attempts секунд"
}

start_backend() {
    step "Запуск backend..."
    cd "$SCRIPT_DIR/rust"
    
    # Остановка предыдущего backend
    pkill -f "semaphore server" 2>/dev/null || true
    sleep 1
    
    # Запуск в фоне
    nohup cargo run --release -- server --host 0.0.0.0 --port 3000 > "$SCRIPT_DIR/logs/backend.log" 2>&1 &
    BACKEND_PID=$!
    echo $BACKEND_PID > "$SCRIPT_DIR/logs/backend.pid"
    
    success "Backend запущен (PID: $BACKEND_PID)"
}

start_backend_sqlite() {
    step "Запуск backend с SQLite..."
    cd "$SCRIPT_DIR/rust"
    
    # Остановка предыдущего backend
    pkill -f "semaphore server" 2>/dev/null || true
    sleep 1
    
    # Запуск в фоне
    nohup cargo run --release -- server --host 0.0.0.0 --port 3000 > "$SCRIPT_DIR/logs/backend.log" 2>&1 &
    BACKEND_PID=$!
    echo $BACKEND_PID > "$SCRIPT_DIR/logs/backend.pid"
    
    success "Backend запущен (PID: $BACKEND_PID)"
}

init_sqlite_db() {
    step "Инициализация SQLite БД..."
    cd "$SCRIPT_DIR/rust"
    
    # Применение миграций
    cargo run --release -- migrate --upgrade
    success "Миграции применены"
    
    # Создание админа
    step "Создание пользователя admin..."
    cargo run --release -- user add \
        --username admin \
        --name "Administrator" \
        --email admin@localhost \
        --password admin123 \
        --admin
    
    success "Пользователь admin создан"
    echo ""
    info "Теперь запустите сервер: ./start.sh sqlite"
}

# ============================================================================
# Вывод статуса
# ============================================================================

print_status_docker_full() {
    echo ""
    echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║         Semaphore UI запущен!                          ║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${GREEN}🌐 Frontend:${NC} http://localhost"
    echo -e "${GREEN}💾 PostgreSQL:${NC} localhost:5432"
    echo -e "${GREEN}🔧 Backend:${NC} http://localhost:3000/api"
    echo ""
    echo -e "${YELLOW}Учётные данные (демо):${NC}"
    echo -e "   admin / demo123"
    echo ""
    echo -e "${YELLOW}Полезные команды:${NC}"
    echo -e "   ${CYAN}./start.sh --stop${NC}           - Остановить сервисы"
    echo -e "   ${CYAN}./start.sh --logs${NC}           - Просмотр логов"
    echo -e "   ${CYAN}./start.sh --clean${NC}          - Очистить данные"
    echo -e "   ${CYAN}docker logs semaphore-db${NC}    - Лог БД"
    echo -e "   ${CYAN}docker logs semaphore-frontend${NC} - Лог frontend"
    echo ""
}

print_status_sqlite() {
    echo ""
    echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║         Semaphore UI запущен!                          ║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${GREEN}🌐 Frontend + API:${NC} http://localhost:3000"
    echo -e "${GREEN}💾 SQLite:${NC} /tmp/semaphore.db"
    echo ""
    echo -e "${YELLOW}Учётные данные:${NC}"
    echo -e "   admin / admin123"
    echo ""
    echo -e "${YELLOW}Полезные команды:${NC}"
    echo -e "   ${CYAN}./start.sh sqlite --stop${NC}   - Остановить backend"
    echo -e "   ${CYAN}./start.sh sqlite --init${NC}   - Инициализировать БД"
    echo -e "   ${CYAN}./start.sh sqlite --build${NC}  - Пересобрать backend"
    echo ""
}

print_status_docker_all() {
    echo ""
    echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║         Semaphore UI запущен!                          ║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${GREEN}🌐 Frontend + API:${NC} http://localhost"
    echo -e "${GREEN}💾 PostgreSQL:${NC} внутри Docker"
    echo ""
    echo -e "${YELLOW}Учётные данные (демо):${NC}"
    echo -e "   admin / demo123"
    echo ""
    echo -e "${YELLOW}Полезные команды:${NC}"
    echo -e "   ${CYAN}./start.sh docker-all --stop${NC}   - Остановить сервисы"
    echo -e "   ${CYAN}./start.sh docker-all --logs${NC}   - Просмотр логов"
    echo -e "   ${CYAN}./start.sh docker-all --clean${NC}  - Очистить данные"
    echo ""
}

# ============================================================================
# Парсинг аргументов
# ============================================================================

# Дополнительные опции
while [[ $# -gt 0 ]]; do
    case $1 in
        --stop)
            STOP_MODE=true
            shift
            ;;
        --restart)
            RESTART_MODE=true
            shift
            ;;
        --clean)
            CLEAN_MODE=true
            shift
            ;;
        --logs)
            LOGS_MODE=true
            shift
            ;;
        --build)
            BUILD_MODE=true
            shift
            ;;
        --backend)
            BACKEND_ONLY=true
            shift
            ;;
        --init)
            INIT_MODE=true
            shift
            ;;
        --help|-h)
            head -25 "$0" | tail -22
            exit 0
            ;;
        *)
            error "Неизвестный параметр: $1"
            ;;
    esac
done

# ============================================================================
# Основной запуск
# ============================================================================

# Создание директории для логов
mkdir -p "$SCRIPT_DIR/logs"

case $MODE in
    docker-full|docker)
        mode_docker_full
        ;;
    sqlite)
        mode_sqlite
        ;;
    docker-all|all)
        mode_docker_all
        ;;
    --help|-h)
        head -25 "$0" | tail -22
        exit 0
        ;;
    *)
        error "Неизвестный режим: $MODE"
        echo "Доступные режимы: docker-full, sqlite, docker-all"
        exit 1
        ;;
esac
