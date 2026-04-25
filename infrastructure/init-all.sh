#!/bin/bash
# 一键初始化基础设施脚本
# 用于快速设置 Kafka Topics 和数据库 Schema

set -e

echo "=========================================="
echo "量化交易平台 - 基础设施初始化"
echo "=========================================="
echo ""

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 配置
KAFKA_BROKER="${KAFKA_BROKER:-localhost:9092}"
POSTGRES_HOST="${POSTGRES_HOST:-localhost}"
POSTGRES_PORT="${POSTGRES_PORT:-5432}"
POSTGRES_USER="${POSTGRES_USER:-postgres}"
POSTGRES_DB="${POSTGRES_DB:-trading_platform}"
CLICKHOUSE_HOST="${CLICKHOUSE_HOST:-localhost}"
CLICKHOUSE_PORT="${CLICKHOUSE_PORT:-9000}"

# 检查依赖
check_dependencies() {
    echo "检查依赖..."

    local missing_deps=()

    if ! command -v kafka-topics.sh &> /dev/null; then
        missing_deps+=("kafka-topics.sh (Kafka)")
    fi

    if ! command -v psql &> /dev/null; then
        missing_deps+=("psql (PostgreSQL)")
    fi

    if ! command -v clickhouse-client &> /dev/null; then
        echo -e "${YELLOW}警告: clickhouse-client 未找到，将跳过 ClickHouse 初始化${NC}"
    fi

    if [ ${#missing_deps[@]} -gt 0 ]; then
        echo -e "${RED}错误: 缺少以下依赖:${NC}"
        for dep in "${missing_deps[@]}"; do
            echo "  - $dep"
        done
        echo ""
        echo "请先安装缺失的依赖，或使用 Docker 环境"
        exit 1
    fi

    echo -e "${GREEN}✓ 依赖检查通过${NC}"
    echo ""
}

# 检查服务连接
check_services() {
    echo "检查服务连接..."

    # 检查 Kafka
    if kafka-broker-api-versions.sh --bootstrap-server "$KAFKA_BROKER" &> /dev/null; then
        echo -e "${GREEN}✓ Kafka 连接成功 ($KAFKA_BROKER)${NC}"
    else
        echo -e "${RED}✗ Kafka 连接失败 ($KAFKA_BROKER)${NC}"
        echo "  请确保 Kafka 正在运行"
        exit 1
    fi

    # 检查 PostgreSQL
    if PGPASSWORD="$POSTGRES_PASSWORD" psql -h "$POSTGRES_HOST" -p "$POSTGRES_PORT" -U "$POSTGRES_USER" -c "SELECT 1" &> /dev/null; then
        echo -e "${GREEN}✓ PostgreSQL 连接成功 ($POSTGRES_HOST:$POSTGRES_PORT)${NC}"
    else
        echo -e "${RED}✗ PostgreSQL 连接失败 ($POSTGRES_HOST:$POSTGRES_PORT)${NC}"
        echo "  请确保 PostgreSQL 正在运行，并设置 POSTGRES_PASSWORD 环境变量"
        exit 1
    fi

    # 检查 ClickHouse (可选)
    if command -v clickhouse-client &> /dev/null; then
        if clickhouse-client --host "$CLICKHOUSE_HOST" --port "$CLICKHOUSE_PORT" --query "SELECT 1" &> /dev/null; then
            echo -e "${GREEN}✓ ClickHouse 连接成功 ($CLICKHOUSE_HOST:$CLICKHOUSE_PORT)${NC}"
        else
            echo -e "${YELLOW}⚠ ClickHouse 连接失败，将跳过 ClickHouse 初始化${NC}"
        fi
    fi

    echo ""
}

# 初始化 Kafka Topics
init_kafka() {
    echo "=========================================="
    echo "1. 初始化 Kafka Topics"
    echo "=========================================="

    cd "$(dirname "$0")"

    if [ -f "kafka-topics.sh" ]; then
        chmod +x kafka-topics.sh
        ./kafka-topics.sh
        echo -e "${GREEN}✓ Kafka Topics 初始化完成${NC}"
    else
        echo -e "${RED}✗ kafka-topics.sh 文件不存在${NC}"
        exit 1
    fi

    echo ""
}

# 初始化数据库
init_database() {
    echo "=========================================="
    echo "2. 初始化 PostgreSQL 数据库"
    echo "=========================================="

    cd "$(dirname "$0")"

    if [ -f "database-schema.sql" ]; then
        PGPASSWORD="$POSTGRES_PASSWORD" psql -h "$POSTGRES_HOST" -p "$POSTGRES_PORT" -U "$POSTGRES_USER" -f database-schema.sql
        echo -e "${GREEN}✓ PostgreSQL 数据库初始化完成${NC}"
    else
        echo -e "${RED}✗ database-schema.sql 文件不存在${NC}"
        exit 1
    fi

    echo ""
}

# 初始化 ClickHouse (可选)
init_clickhouse() {
    if ! command -v clickhouse-client &> /dev/null; then
        return
    fi

    if ! clickhouse-client --host "$CLICKHOUSE_HOST" --port "$CLICKHOUSE_PORT" --query "SELECT 1" &> /dev/null; then
        return
    fi

    echo "=========================================="
    echo "3. 初始化 ClickHouse"
    echo "=========================================="

    # 创建数据库
    clickhouse-client --host "$CLICKHOUSE_HOST" --port "$CLICKHOUSE_PORT" --query "CREATE DATABASE IF NOT EXISTS market_data"

    # 创建行情数据表
    clickhouse-client --host "$CLICKHOUSE_HOST" --port "$CLICKHOUSE_PORT" --query "
    CREATE TABLE IF NOT EXISTS market_data.trades (
        exchange String,
        symbol String,
        trade_id String,
        price Float64,
        quantity Float64,
        is_buyer_maker UInt8,
        event_time DateTime64(3),
        insert_time DateTime64(3)
    ) ENGINE = MergeTree()
    PARTITION BY toYYYYMMDD(event_time)
    ORDER BY (symbol, event_time, trade_id)
    "

    echo -e "${GREEN}✓ ClickHouse 初始化完成${NC}"
    echo ""
}

# 验证初始化结果
verify_setup() {
    echo "=========================================="
    echo "验证初始化结果"
    echo "=========================================="

    # 验证 Kafka Topics
    echo "Kafka Topics:"
    kafka-topics.sh --bootstrap-server "$KAFKA_BROKER" --list | grep -E "(market-events|strategy-signals|execution)" || true
    echo ""

    # 验证数据库表
    echo "PostgreSQL Tables:"
    PGPASSWORD="$POSTGRES_PASSWORD" psql -h "$POSTGRES_HOST" -p "$POSTGRES_PORT" -U "$POSTGRES_USER" -d "$POSTGRES_DB" -c "\dt" | head -20
    echo ""

    # 验证 ClickHouse (可选)
    if command -v clickhouse-client &> /dev/null && clickhouse-client --host "$CLICKHOUSE_HOST" --port "$CLICKHOUSE_PORT" --query "SELECT 1" &> /dev/null; then
        echo "ClickHouse Tables:"
        clickhouse-client --host "$CLICKHOUSE_HOST" --port "$CLICKHOUSE_PORT" --query "SHOW TABLES FROM market_data"
        echo ""
    fi
}

# 主函数
main() {
    check_dependencies
    check_services
    init_kafka
    init_database
    init_clickhouse
    verify_setup

    echo "=========================================="
    echo -e "${GREEN}✓ 基础设施初始化完成！${NC}"
    echo "=========================================="
    echo ""
    echo "下一步："
    echo "  1. 启动 Market Data Service"
    echo "  2. 启动 Strategy Engine"
    echo "  3. 启动 Trading Engine"
    echo ""
    echo "查看文档: infrastructure/README.md"
}

# 执行主函数
main
