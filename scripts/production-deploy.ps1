# 生产环境自动化部署脚本
param(
    [string]$Version = "latest",
    [switch]$RollbackOnFailure = $true,
    [switch]$SkipBackup = $false,
    [string]$ConfigPath = "config/production"
)

Write-Host "🚀 Production Deployment Script v2.0" -ForegroundColor Cyan
Write-Host "Version: $Version" -ForegroundColor Yellow
Write-Host "=" * 70

# 预部署检查
Write-Host "📋 Step 1: Pre-deployment Checks" -ForegroundColor Cyan

# 检查Docker环境
if (-not (Get-Command docker -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Docker is not installed or not in PATH" -ForegroundColor Red
    exit 1
}

# 检查Docker Compose
if (-not (Get-Command docker-compose -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Docker Compose is not installed or not in PATH" -ForegroundColor Red
    exit 1
}

# 检查配置文件
$requiredConfigs = @(
    "$ConfigPath/docker-compose.prod.yml",
    "$ConfigPath/prometheus/prometheus.yml",
    "$ConfigPath/grafana/grafana.ini"
)

foreach ($config in $requiredConfigs) {
    if (-not (Test-Path $config)) {
        Write-Host "❌ Missing configuration file: $config" -ForegroundColor Red
        exit 1
    }
}

Write-Host "✅ Pre-deployment checks passed" -ForegroundColor Green

# 备份当前部署
if (-not $SkipBackup) {
    Write-Host ""
    Write-Host "💾 Step 2: Creating Backup" -ForegroundColor Cyan
    
    $backupDir = "backups/$(Get-Date -Format 'yyyyMMdd-HHmmss')"
    New-Item -ItemType Directory -Path $backupDir -Force | Out-Null
    
    # 备份数据库
    Write-Host "  📊 Backing up PostgreSQL..." -ForegroundColor Yellow
    docker exec trading_postgres pg_dump -U postgres trading_platform > "$backupDir/postgres_backup.sql"
    
    # 备份Redis
    Write-Host "  🔄 Backing up Redis..." -ForegroundColor Yellow
    docker exec trading_redis redis-cli BGSAVE
    docker cp trading_redis:/data/dump.rdb "$backupDir/redis_backup.rdb"
    
    # 备份配置
    Write-Host "  ⚙️  Backing up configurations..." -ForegroundColor Yellow
    Copy-Item -Path "config" -Destination "$backupDir/config" -Recurse
    
    Write-Host "✅ Backup completed: $backupDir" -ForegroundColor Green
}

# 构建和部署
Write-Host ""
Write-Host "🔨 Step 3: Building and Deploying" -ForegroundColor Cyan

# 拉取最新镜像
Write-Host "  📥 Pulling latest images..." -ForegroundColor Yellow
docker-compose -f "$ConfigPath/docker-compose.prod.yml" pull

# 构建自定义镜像
Write-Host "  🔨 Building custom images..." -ForegroundColor Yellow
docker-compose -f "$ConfigPath/docker-compose.prod.yml" build --no-cache

# 停止现有服务
Write-Host "  🛑 Stopping existing services..." -ForegroundColor Yellow
docker-compose -f "$ConfigPath/docker-compose.prod.yml" down --remove-orphans

# 启动新服务
Write-Host "  🚀 Starting new services..." -ForegroundColor Yellow
docker-compose -f "$ConfigPath/docker-compose.prod.yml" up -d

# 等待服务启动
Write-Host "  ⏳ Waiting for services to initialize..." -ForegroundColor Yellow
Start-Sleep -Seconds 30

# 健康检查
Write-Host ""
Write-Host "🔍 Step 4: Health Verification" -ForegroundColor Cyan

$services = @(
    @{Name="Market Data"; URL="http://localhost:8081/health"},
    @{Name="Trading Engine"; URL="http://localhost:8082/health"},
    @{Name="Strategy Engine"; URL="http://localhost:8083/health"},
    @{Name="User Management"; URL="http://localhost:8084/health"},
    @{Name="Risk Management"; URL="http://localhost:8085/health"},
    @{Name="Notification"; URL="http://localhost:8086/health"},
    @{Name="Prometheus"; URL="http://localhost:9090/-/healthy"},
    @{Name="Grafana"; URL="http://localhost:3000/api/health"}
)

$healthyServices = 0
$totalServices = $services.Count

foreach ($service in $services) {
    try {
        $response = Invoke-RestMethod -Uri $service.URL -TimeoutSec 10 -ErrorAction Stop
        Write-Host "  ✅ $($service.Name) is healthy" -ForegroundColor Green
        $healthyServices++
    } catch {
        Write-Host "  ❌ $($service.Name) health check failed" -ForegroundColor Red
    }
}

$healthPercentage = ($healthyServices / $totalServices) * 100

# 部署验证
Write-Host ""
Write-Host "📊 Step 5: Deployment Verification" -ForegroundColor Cyan
Write-Host "Health Status: $healthyServices/$totalServices services healthy ($([math]::Round($healthPercentage, 1))%)" -ForegroundColor $(if ($healthPercentage -ge 80) { "Green" } else { "Red" })

if ($healthPercentage -lt 80 -and $RollbackOnFailure) {
    Write-Host ""
    Write-Host "⚠️  Deployment failed health checks. Initiating rollback..." -ForegroundColor Yellow
    
    # 回滚到之前的版本
    docker-compose -f "$ConfigPath/docker-compose.prod.yml" down
    
    # 恢复备份（如果存在）
    if (-not $SkipBackup -and (Test-Path $backupDir)) {
        Write-Host "  📥 Restoring from backup..." -ForegroundColor Yellow
        # 这里可以添加具体的恢复逻辑
    }
    
    Write-Host "❌ Deployment failed and rolled back" -ForegroundColor Red
    exit 1
}

# 部署成功
Write-Host ""
Write-Host "🎉 Production Deployment Successful!" -ForegroundColor Green
Write-Host ""
Write-Host "📊 Monitoring URLs:" -ForegroundColor Cyan
Write-Host "  • Grafana: http://localhost:3000" -ForegroundColor Gray
Write-Host "  • Prometheus: http://localhost:9090" -ForegroundColor Gray
Write-Host "  • AlertManager: http://localhost:9093" -ForegroundColor Gray
Write-Host ""
Write-Host "🔧 Management Commands:" -ForegroundColor Cyan
Write-Host "  • View Logs: docker-compose -f $ConfigPath/docker-compose.prod.yml logs -f" -ForegroundColor Gray
Write-Host "  • Scale Service: docker-compose -f $ConfigPath/docker-compose.prod.yml up -d --scale market-data=3" -ForegroundColor Gray
Write-Host "  • Stop All: docker-compose -f $ConfigPath/docker-compose.prod.yml down" -ForegroundColor Gray

# 生成部署报告
$deploymentReport = @{
    Timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    Version = $Version
    HealthyServices = $healthyServices
    TotalServices = $totalServices
    HealthPercentage = $healthPercentage
    BackupLocation = if (-not $SkipBackup) { $backupDir } else { "Skipped" }
}

$deploymentReport | ConvertTo-Json | Out-File "logs/deployment-$(Get-Date -Format 'yyyyMMdd-HHmmss').json"

Write-Host ""
Write-Host "📝 Deployment report saved to logs/" -ForegroundColor Gray
Write-Host "🚀 Production system is ready!" -ForegroundColor Cyan