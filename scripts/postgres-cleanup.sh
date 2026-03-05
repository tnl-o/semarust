#!/bin/bash
# ==============================================================================
# Скрипт остановки и удаления контейнера PostgreSQL для Semaphore
# ==============================================================================
# Этот скрипт:
#   1. Останавливает контейнер semaphore_postgres
#   2. Удаляет контейнер
#   3. Удаляет volume с данными
# ==============================================================================

set -e

COMPOSE_FILE="docker-compose.postgres.yml"

# Цвета для вывода
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}==============================================================================${NC}"
echo -e "${BLUE}Semaphore - Остановка и удаление PostgreSQL${NC}"
echo -e "${BLUE}==============================================================================${NC}"
echo ""

# Проверка наличия Docker
if ! command -v docker &> /dev/null; then
    echo -e "${RED}Ошибка: Docker не найден. Пожалуйста, установите Docker.${NC}"
    exit 1
fi

# Функция для определения команды docker-compose
get_compose_cmd() {
    if command -v docker-compose &> /dev/null; then
        echo "docker-compose"
    else
        echo "docker compose"
    fi
}

COMPOSE_CMD=$(get_compose_cmd)

echo -e "${YELLOW}[1/3] Остановка контейнера semaphore_postgres...${NC}"
if docker ps -q -f name=semaphore_postgres 2>/dev/null | grep -q .; then
    docker stop semaphore_postgres
    echo -e "${GREEN}✓ Контейнер остановлен${NC}"
else
    echo -e "${YELLOW}⚠ Контейнер semaphore_postgres не запущен${NC}"
fi

echo -e "${YELLOW}[2/3] Удаление контейнера...${NC}"
if docker ps -aq -f name=semaphore_postgres 2>/dev/null | grep -q .; then
    docker rm semaphore_postgres
    echo -e "${GREEN}✓ Контейнер удален${NC}"
else
    echo -e "${YELLOW}⚠ Контейнер semaphore_postgres не существует${NC}"
fi

echo -e "${YELLOW}[3/3] Удаление volume с данными...${NC}"
if docker volume ls -q -f name=semaphore_postgres_data 2>/dev/null | grep -q .; then
    docker volume rm semaphore_postgres_data
    echo -e "${GREEN}✓ Volume удален${NC}"
else
    echo -e "${YELLOW}⚠ Volume semaphore_postgres_data не существует${NC}"
fi

echo ""
echo -e "${GREEN}==============================================================================${NC}"
echo -e "${GREEN}Готово!${NC}"
echo -e "${GREEN}==============================================================================${NC}"
echo ""
echo -e "${BLUE}Для запуска нового экземпляра выполните:${NC}"
echo -e "  ${GREEN}./scripts/postgres-demo-start.sh${NC}"
echo ""
echo -e "${BLUE}Или просто запустите PostgreSQL без демонстрационных данных:${NC}"
echo -e "  ${GREEN}docker-compose -f docker-compose.postgres.yml up -d${NC}"
echo ""
