# DevOps Status Monitor
function Show-DevOpsStatus {
    Clear-Host
    Write-Host "=== DevOps Infrastructure Status ===" -ForegroundColor Cyan
    Write-Host "Time: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')" -ForegroundColor Gray
    Write-Host ""
    
    # Infrastructure Services
    Write-Host "Infrastructure Services:" -ForegroundColor Yellow
    $infraServices = @("trading_postgres_23", "market_data_redis", "market_data_clickhouse", "market_data_kafka")
    foreach ($service in $infraServices) {
        try {
            $status = docker inspect --format='{{.State.Status}}' $service 2>$null
            $color = if ($status -eq "running") { "Green" } else { "Red" }
            Write-Host "  $($service): " -NoNewline
            Write-Host $status -ForegroundColor $color
        }
        catch {
            Write-Host "  $($service): Not Found" -ForegroundColor Red
        }
    }
    
    Write-Host ""
    Write-Host "Monitoring Services:" -ForegroundColor Yellow
    $monitoringServices = @("trading_prometheus", "trading_grafana")
    foreach ($service in $monitoringServices) {
        try {
            $status = docker inspect --format='{{.State.Status}}' $service 2>$null
            $color = if ($status -eq "running") { "Green" } else { "Red" }
            Write-Host "  $($service): " -NoNewline
            Write-Host $status -ForegroundColor $color
        }
        catch {
            Write-Host "  $($service): Not Found" -ForegroundColor Red
        }
    }
}

Show-DevOpsStatus
