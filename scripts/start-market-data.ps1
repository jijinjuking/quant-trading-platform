# 市场数据服务启动脚本
param(
    [switch]$Build = $false,
    [switch]$Watch = $false,
    [string]$LogLevel = "info"
)

Write-Host "=== 市场数据服务启动脚本 ===" -ForegroundColor Cyan
Write-Host "启动时间: $(Get-Date)" -ForegroundColor Gray

# 检查基础设施状态
Write-Host "1. 检查基础设施状态..." -ForegroundColor Yellow

$requiredServices = @(
    @{Name="PostgreSQL"; Container="trading_postgres_23"},
    @{Name="Redis"; Container="market_data_redis"},
    @{Name="ClickHouse"; Container="market_data_clickhouse"},
    @{Name="Kafka"; Container="market_data_kafka"}
)

$allHealthy = $true
foreach ($service in $requiredServices) {
    try {
        $status = docker inspect --format='{{.State.Status}}' $service.Container 2>$null
        if ($status -eq "running") {
            Write-Host "  ✓ $($service.Name) - 运行正常" -ForegroundColor Green
        } else {
            Write-Host "  ✗ $($service.Name) - 状态异常: $status" -ForegroundColor Red
            $allHealthy = $false
        }
    }
    catch {
        Write-Host "  ✗ $($service.Name) - 服务不可用" -ForegroundColor Red
        $allHealthy = $false
    }
}

if (-not $allHealthy) {
    Write-Host "❌ 基础设施检查失败，请先启动所需服务" -ForegroundColor Red
    Write-Host "运行: docker-compose -f docker-compose.dev.yml up -d" -ForegroundColor Yellow
    exit 1
}

# 设置环境变量
Write-Host "2. 设置环境变量..." -ForegroundColor Yellow
$env:RUST_LOG = $LogLevel
$env:MARKET_DATA__SERVER__HOST = "0.0.0.0"
$env:MARKET_DATA__SERVER__PORT = "8081"

# 从.env.database文件加载环境变量
if (Test-Path ".env.database") {
    Get-Content ".env.database" | ForEach-Object {
        if ($_ -match "^([^#][^=]+)=(.*)$") {
            [Environment]::SetEnvironmentVariable($matches[1], $matches[2], "Process")
        }
    }
    Write-Host "  ✓ 环境变量已加载" -ForegroundColor Green
}

# 构建服务（如果需要）
if ($Build) {
    Write-Host "3. 构建市场数据服务..." -ForegroundColor Yellow
    Set-Location "services/market-data"
    
    try {
        cargo build --release
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  ✓ 构建成功" -ForegroundColor Green
        } else {
            Write-Host "  ✗ 构建失败" -ForegroundColor Red
            Set-Location "../.."
            exit 1
        }
    }
    catch {
        Write-Host "  ✗ 构建异常: $_" -ForegroundColor Red
        Set-Location "../.."
        exit 1
    }
    
    Set-Location "../.."
}

# 启动服务
Write-Host "4. 启动市场数据服务..." -ForegroundColor Yellow
Set-Location "services/market-data"

try {
    if ($Watch) {
        Write-Host "  🔄 监视模式启动 (cargo watch)" -ForegroundColor Cyan
        cargo watch -x run
    } else {
        Write-Host "  🚀 正常模式启动" -ForegroundColor Cyan
        cargo run
    }
}
catch {
    Write-Host "  ✗ 启动失败: $_" -ForegroundColor Red
    Set-Location "../.."
    exit 1
}

Set-Location "../.."