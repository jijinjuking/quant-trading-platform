# 日志收集脚本
# 收集所有服务的日志到统一目录

param(
    [string]$OutputDir = "logs/$(Get-Date -Format 'yyyy-MM-dd')",
    [int]$Lines = 1000
)

# 创建日志目录
New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null

Write-Host "开始收集服务日志到: $OutputDir" -ForegroundColor Green

# 基础设施服务日志
$services = @(
    "trading_postgres_23",
    "market_data_redis", 
    "market_data_clickhouse",
    "market_data_kafka",
    "market_data_zookeeper",
    "trading_prometheus",
    "trading_grafana"
)

foreach ($service in $services) {
    Write-Host "收集 $service 日志..." -ForegroundColor Yellow
    try {
        docker logs --tail $Lines $service > "$OutputDir/$service.log" 2>&1
        Write-Host "✓ $service 日志收集完成" -ForegroundColor Green
    }
    catch {
        Write-Host "✗ $service 日志收集失败: $_" -ForegroundColor Red
    }
}

# 收集系统信息
Write-Host "收集系统信息..." -ForegroundColor Yellow
docker ps > "$OutputDir/docker-ps.log"
docker stats --no-stream > "$OutputDir/docker-stats.log"
docker system df > "$OutputDir/docker-df.log"

Write-Host "日志收集完成！" -ForegroundColor Green
Write-Host "日志位置: $OutputDir" -ForegroundColor Cyan