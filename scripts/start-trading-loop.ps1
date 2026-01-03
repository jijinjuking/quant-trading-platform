# 交易闭环启动脚本 (PowerShell)
# 用于启动最小交易闭环的三个服务

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  量化交易平台 - 最小闭环启动脚本" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# 检查 Kafka 是否运行
Write-Host "[1/4] 检查 Kafka..." -ForegroundColor Yellow
$kafka = docker ps 2>$null | Select-String "kafka"
if (-not $kafka) {
    Write-Host "  ❌ Kafka 未运行，请先启动: docker-compose up -d kafka zookeeper" -ForegroundColor Red
    exit 1
}
Write-Host "  ✅ Kafka 运行中" -ForegroundColor Green

# 启动 strategy-engine
Write-Host ""
Write-Host "[2/4] 启动 strategy-engine (8083)..." -ForegroundColor Yellow
Start-Process -FilePath "cargo" -ArgumentList "run -p strategy-engine" -WorkingDirectory $PSScriptRoot\.. -WindowStyle Normal
Start-Sleep -Seconds 3
Write-Host "  ✅ strategy-engine 启动中" -ForegroundColor Green

# 启动 trading-engine
Write-Host ""
Write-Host "[3/4] 启动 trading-engine (8081)..." -ForegroundColor Yellow
Start-Process -FilePath "cargo" -ArgumentList "run -p trading-engine" -WorkingDirectory $PSScriptRoot\.. -WindowStyle Normal
Start-Sleep -Seconds 3
Write-Host "  ✅ trading-engine 启动中" -ForegroundColor Green

# 启动 market-data
Write-Host ""
Write-Host "[4/4] 启动 market-data (8082)..." -ForegroundColor Yellow
Start-Process -FilePath "cargo" -ArgumentList "run -p market-data" -WorkingDirectory $PSScriptRoot\.. -WindowStyle Normal
Start-Sleep -Seconds 3
Write-Host "  ✅ market-data 启动中" -ForegroundColor Green

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  所有服务已启动！" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "服务地址:" -ForegroundColor White
Write-Host "  - strategy-engine: http://localhost:8083" -ForegroundColor Gray
Write-Host "  - trading-engine:  http://localhost:8081" -ForegroundColor Gray
Write-Host "  - market-data:     (无 HTTP API)" -ForegroundColor Gray
Write-Host ""
Write-Host "健康检查:" -ForegroundColor White
Write-Host "  curl http://localhost:8083/health" -ForegroundColor Gray
Write-Host "  curl http://localhost:8081/health" -ForegroundColor Gray
Write-Host ""
Write-Host "查看日志: 请查看各服务的终端窗口" -ForegroundColor Yellow
