# 一键初始化基础设施脚本 (PowerShell)
# 用于快速设置 Kafka Topics 和数据库 Schema

param(
    [string]$KafkaBroker = "localhost:9092",
    [string]$PostgresHost = "localhost",
    [int]$PostgresPort = 5432,
    [string]$PostgresUser = "postgres",
    [string]$PostgresPassword = "",
    [string]$PostgresDb = "trading_platform",
    [string]$ClickHouseHost = "localhost",
    [int]$ClickHousePort = 9000
)

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "量化交易平台 - 基础设施初始化" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host ""

# 检查依赖
function Check-Dependencies {
    Write-Host "检查依赖..." -ForegroundColor Yellow

    $missingDeps = @()

    if (-not (Get-Command kafka-topics.bat -ErrorAction SilentlyContinue)) {
        $missingDeps += "kafka-topics.bat (Kafka)"
    }

    if (-not (Get-Command psql -ErrorAction SilentlyContinue)) {
        $missingDeps += "psql (PostgreSQL)"
    }

    if (-not (Get-Command clickhouse-client -ErrorAction SilentlyContinue)) {
        Write-Host "警告: clickhouse-client 未找到，将跳过 ClickHouse 初始化" -ForegroundColor Yellow
    }

    if ($missingDeps.Count -gt 0) {
        Write-Host "错误: 缺少以下依赖:" -ForegroundColor Red
        foreach ($dep in $missingDeps) {
            Write-Host "  - $dep" -ForegroundColor Red
        }
        Write-Host ""
        Write-Host "请先安装缺失的依赖，或使用 Docker 环境"
        exit 1
    }

    Write-Host "✓ 依赖检查通过" -ForegroundColor Green
    Write-Host ""
}

# 检查服务连接
function Check-Services {
    Write-Host "检查服务连接..." -ForegroundColor Yellow

    # 检查 Kafka
    try {
        $null = kafka-broker-api-versions.bat --bootstrap-server $KafkaBroker 2>&1
        Write-Host "✓ Kafka 连接成功 ($KafkaBroker)" -ForegroundColor Green
    } catch {
        Write-Host "✗ Kafka 连接失败 ($KafkaBroker)" -ForegroundColor Red
        Write-Host "  请确保 Kafka 正在运行"
        exit 1
    }

    # 检查 PostgreSQL
    $env:PGPASSWORD = $PostgresPassword
    try {
        $null = psql -h $PostgresHost -p $PostgresPort -U $PostgresUser -c "SELECT 1" 2>&1
        Write-Host "✓ PostgreSQL 连接成功 (${PostgresHost}:${PostgresPort})" -ForegroundColor Green
    } catch {
        Write-Host "✗ PostgreSQL 连接失败 (${PostgresHost}:${PostgresPort})" -ForegroundColor Red
        Write-Host "  请确保 PostgreSQL 正在运行，并设置正确的密码"
        exit 1
    }

    # 检查 ClickHouse (可选)
    if (Get-Command clickhouse-client -ErrorAction SilentlyContinue) {
        try {
            $null = clickhouse-client --host $ClickHouseHost --port $ClickHousePort --query "SELECT 1" 2>&1
            Write-Host "✓ ClickHouse 连接成功 (${ClickHouseHost}:${ClickHousePort})" -ForegroundColor Green
        } catch {
            Write-Host "⚠ ClickHouse 连接失败，将跳过 ClickHouse 初始化" -ForegroundColor Yellow
        }
    }

    Write-Host ""
}

# 初始化 Kafka Topics
function Init-Kafka {
    Write-Host "==========================================" -ForegroundColor Cyan
    Write-Host "1. 初始化 Kafka Topics" -ForegroundColor Cyan
    Write-Host "==========================================" -ForegroundColor Cyan

    $scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
    $kafkaScript = Join-Path $scriptDir "kafka-topics.ps1"

    if (Test-Path $kafkaScript) {
        & $kafkaScript -KafkaBroker $KafkaBroker
        Write-Host "✓ Kafka Topics 初始化完成" -ForegroundColor Green
    } else {
        Write-Host "✗ kafka-topics.ps1 文件不存在" -ForegroundColor Red
        exit 1
    }

    Write-Host ""
}

# 初始化数据库
function Init-Database {
    Write-Host "==========================================" -ForegroundColor Cyan
    Write-Host "2. 初始化 PostgreSQL 数据库" -ForegroundColor Cyan
    Write-Host "==========================================" -ForegroundColor Cyan

    $scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
    $sqlScript = Join-Path $scriptDir "database-schema.sql"

    if (Test-Path $sqlScript) {
        $env:PGPASSWORD = $PostgresPassword
        psql -h $PostgresHost -p $PostgresPort -U $PostgresUser -f $sqlScript
        Write-Host "✓ PostgreSQL 数据库初始化完成" -ForegroundColor Green
    } else {
        Write-Host "✗ database-schema.sql 文件不存在" -ForegroundColor Red
        exit 1
    }

    Write-Host ""
}

# 初始化 ClickHouse (可选)
function Init-ClickHouse {
    if (-not (Get-Command clickhouse-client -ErrorAction SilentlyContinue)) {
        return
    }

    try {
        $null = clickhouse-client --host $ClickHouseHost --port $ClickHousePort --query "SELECT 1" 2>&1
    } catch {
        return
    }

    Write-Host "==========================================" -ForegroundColor Cyan
    Write-Host "3. 初始化 ClickHouse" -ForegroundColor Cyan
    Write-Host "==========================================" -ForegroundColor Cyan

    # 创建数据库
    clickhouse-client --host $ClickHouseHost --port $ClickHousePort --query "CREATE DATABASE IF NOT EXISTS market_data"

    # 创建行情数据表
    $createTableQuery = @"
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
"@

    clickhouse-client --host $ClickHouseHost --port $ClickHousePort --query $createTableQuery

    Write-Host "✓ ClickHouse 初始化完成" -ForegroundColor Green
    Write-Host ""
}

# 验证初始化结果
function Verify-Setup {
    Write-Host "==========================================" -ForegroundColor Cyan
    Write-Host "验证初始化结果" -ForegroundColor Cyan
    Write-Host "==========================================" -ForegroundColor Cyan

    # 验证 Kafka Topics
    Write-Host "Kafka Topics:" -ForegroundColor Yellow
    kafka-topics.bat --bootstrap-server $KafkaBroker --list | Select-String -Pattern "(market-events|strategy-signals|execution)"
    Write-Host ""

    # 验证数据库表
    Write-Host "PostgreSQL Tables:" -ForegroundColor Yellow
    $env:PGPASSWORD = $PostgresPassword
    psql -h $PostgresHost -p $PostgresPort -U $PostgresUser -d $PostgresDb -c "\dt" | Select-Object -First 20
    Write-Host ""

    # 验证 ClickHouse (可选)
    if (Get-Command clickhouse-client -ErrorAction SilentlyContinue) {
        try {
            $null = clickhouse-client --host $ClickHouseHost --port $ClickHousePort --query "SELECT 1" 2>&1
            Write-Host "ClickHouse Tables:" -ForegroundColor Yellow
            clickhouse-client --host $ClickHouseHost --port $ClickHousePort --query "SHOW TABLES FROM market_data"
            Write-Host ""
        } catch {
            # Silently skip if ClickHouse is not available
        }
    }
}

# 主函数
function Main {
    Check-Dependencies
    Check-Services
    Init-Kafka
    Init-Database
    Init-ClickHouse
    Verify-Setup

    Write-Host "==========================================" -ForegroundColor Cyan
    Write-Host "✓ 基础设施初始化完成！" -ForegroundColor Green
    Write-Host "==========================================" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "下一步:" -ForegroundColor Yellow
    Write-Host "  1. 启动 Market Data Service"
    Write-Host "  2. 启动 Strategy Engine"
    Write-Host "  3. 启动 Trading Engine"
    Write-Host ""
    Write-Host "查看文档: infrastructure\README.md"
}

# 执行主函数
Main
