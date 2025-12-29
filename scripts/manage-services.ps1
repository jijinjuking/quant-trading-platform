# 微服务管理脚本
param(
    [Parameter(Mandatory=$true)]
    [ValidateSet("start", "stop", "restart", "status", "logs")]
    [string]$Action,
    [string]$Service = "all",
    [switch]$Build = $false
)

Write-Host "=== 微服务管理系统 ===" -ForegroundColor Cyan
Write-Host "操作: $Action | 服务: $Service" -ForegroundColor Gray

# 服务配置
$services = @{
    "market-data" = @{
        Name = "Market Data Service"
        Path = "services/market-data"
        Port = 8081
        Binary = "market-data"
    }
    "trading-engine" = @{
        Name = "Trading Engine"
        Path = "services/trading-engine"
        Port = 8082
        Binary = "trading-engine"
    }
    "strategy-engine" = @{
        Name = "Strategy Engine"
        Path = "services/strategy-engine"
        Port = 8083
        Binary = "strategy-engine"
    }
    "risk-management" = @{
        Name = "Risk Management"
        Path = "services/risk-management"
        Port = 8085
        Binary = "risk-management"
    }
    "user-management" = @{
        Name = "User Management"
        Path = "services/user-management"
        Port = 8086
        Binary = "user-management"
    }
    "notification" = @{
        Name = "Notification Service"
        Path = "services/notification"
        Port = 8087
        Binary = "notification"
    }
}

function Start-MicroService($ServiceConfig) {
    Write-Host "🚀 Starting $($ServiceConfig.Name)..." -ForegroundColor Green
    
    # 检查端口
    $connection = Test-NetConnection -ComputerName localhost -Port $ServiceConfig.Port -WarningAction SilentlyContinue
    if ($connection.TcpTestSucceeded) {
        Write-Host "⚠️ Port $($ServiceConfig.Port) already in use" -ForegroundColor Yellow
        return
    }
    
    # 构建服务
    if ($Build) {
        Write-Host "🔨 Building $($ServiceConfig.Name)..." -ForegroundColor Yellow
        Set-Location $ServiceConfig.Path
        cargo build --release
        if ($LASTEXITCODE -ne 0) {
            Write-Host "❌ Build failed" -ForegroundColor Red
            Set-Location "../.."
            return
        }
        Set-Location "../.."
    }
    
    # 启动服务
    Set-Location $ServiceConfig.Path
    Start-Process -FilePath "cargo" -ArgumentList "run" -NoNewWindow
    Set-Location "../.."
    
    Write-Host "✅ $($ServiceConfig.Name) started on port $($ServiceConfig.Port)" -ForegroundColor Green
}

function Stop-MicroService($ServiceConfig) {
    Write-Host "🛑 Stopping $($ServiceConfig.Name)..." -ForegroundColor Yellow
    
    $processes = Get-Process -Name $ServiceConfig.Binary -ErrorAction SilentlyContinue
    if ($processes) {
        $processes | Stop-Process -Force
        Write-Host "✅ $($ServiceConfig.Name) stopped" -ForegroundColor Green
    } else {
        Write-Host "ℹ️ $($ServiceConfig.Name) not running" -ForegroundColor Gray
    }
}

function Get-MicroServiceStatus($ServiceConfig) {
    $connection = Test-NetConnection -ComputerName localhost -Port $ServiceConfig.Port -WarningAction SilentlyContinue
    $status = if ($connection.TcpTestSucceeded) { "🟢 Running" } else { "🔴 Stopped" }
    Write-Host "  $($ServiceConfig.Name): $status (Port: $($ServiceConfig.Port))" -ForegroundColor White
}

# 设置环境变量
$env:RUST_LOG = "info"
$env:DATABASE_URL = "postgresql://postgres:password@localhost:5432/trading_db"
$env:REDIS_URL = "redis://localhost:6379"
$env:CLICKHOUSE_URL = "http://localhost:8123"
$env:KAFKA_BROKERS = "localhost:9092"

# 执行操作
switch ($Action) {
    "start" {
        if ($Service -eq "all") {
            foreach ($svc in $services.Values) {
                Start-MicroService $svc
            }
        } else {
            Start-MicroService $services[$Service]
        }
    }
    "stop" {
        if ($Service -eq "all") {
            foreach ($svc in $services.Values) {
                Stop-MicroService $svc
            }
        } else {
            Stop-MicroService $services[$Service]
        }
    }
    "status" {
        Write-Host "📊 微服务状态:" -ForegroundColor Cyan
        foreach ($svc in $services.Values) {
            Get-MicroServiceStatus $svc
        }
    }
}
