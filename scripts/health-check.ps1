# Infrastructure Health Check Script

Write-Host "=== Trading Platform Infrastructure Health Check ===" -ForegroundColor Cyan
Write-Host "Check Time: $(Get-Date)" -ForegroundColor Gray
Write-Host ""

$healthStatus = @{}

# Check Docker service status
Write-Host "1. Docker Service Status Check" -ForegroundColor Yellow
$services = @(
    @{Name="PostgreSQL"; Container="trading_postgres_23"; Port=5432},
    @{Name="Redis"; Container="market_data_redis"; Port=6379},
    @{Name="ClickHouse"; Container="market_data_clickhouse"; Port=8123},
    @{Name="Kafka"; Container="market_data_kafka"; Port=9092},
    @{Name="Zookeeper"; Container="market_data_zookeeper"; Port=2181},
    @{Name="Prometheus"; Container="trading_prometheus"; Port=9090},
    @{Name="Grafana"; Container="trading_grafana"; Port=3000}
)

foreach ($service in $services) {
    try {
        $status = docker inspect --format='{{.State.Status}}' $service.Container 2>$null
        if ($status -eq "running") {
            Write-Host "  OK $($service.Name) - Running" -ForegroundColor Green
            $healthStatus[$service.Name] = "Healthy"
        } else {
            Write-Host "  ERROR $($service.Name) - Status: $status" -ForegroundColor Red
            $healthStatus[$service.Name] = "Unhealthy"
        }
    }
    catch {
        Write-Host "  ERROR $($service.Name) - Container not found" -ForegroundColor Red
        $healthStatus[$service.Name] = "Missing"
    }
}

Write-Host ""

# Check port connectivity
Write-Host "2. Port Connectivity Check" -ForegroundColor Yellow
foreach ($service in $services) {
    try {
        $connection = Test-NetConnection -ComputerName localhost -Port $service.Port -WarningAction SilentlyContinue
        if ($connection.TcpTestSucceeded) {
            Write-Host "  OK $($service.Name) Port $($service.Port) - Accessible" -ForegroundColor Green
        } else {
            Write-Host "  ERROR $($service.Name) Port $($service.Port) - Not accessible" -ForegroundColor Red
        }
    }
    catch {
        Write-Host "  ERROR $($service.Name) Port $($service.Port) - Check failed" -ForegroundColor Red
    }
}

Write-Host ""

# Check database connections
Write-Host "3. Database Connection Check" -ForegroundColor Yellow
try {
    $result = docker exec trading_postgres_23 psql -U postgres -d trading_db -c "SELECT 1;" 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Host "  OK PostgreSQL database connection" -ForegroundColor Green
    } else {
        Write-Host "  ERROR PostgreSQL database connection failed" -ForegroundColor Red
    }
}
catch {
    Write-Host "  ERROR PostgreSQL database connection check failed" -ForegroundColor Red
}

try {
    $result = docker exec market_data_redis redis-cli ping 2>$null
    if ($result -eq "PONG") {
        Write-Host "  OK Redis connection" -ForegroundColor Green
    } else {
        Write-Host "  ERROR Redis connection failed" -ForegroundColor Red
    }
}
catch {
    Write-Host "  ERROR Redis connection check failed" -ForegroundColor Red
}

Write-Host ""

# Summary
Write-Host "=== Health Check Summary ===" -ForegroundColor Cyan
$healthyCount = ($healthStatus.Values | Where-Object { $_ -eq "Healthy" }).Count
$totalCount = $healthStatus.Count
Write-Host "Service Status: $healthyCount/$totalCount Healthy" -ForegroundColor $(if ($healthyCount -eq $totalCount) { "Green" } else { "Yellow" })

Write-Host ""
Write-Host "Monitoring Dashboard URLs:" -ForegroundColor Cyan
Write-Host "  • Grafana: http://localhost:3000 (admin/admin123)" -ForegroundColor Gray
Write-Host "  • Prometheus: http://localhost:9090" -ForegroundColor Gray
Write-Host "  • Kafka UI: http://localhost:8080" -ForegroundColor Gray
Write-Host "  • Redis Commander: http://localhost:8082" -ForegroundColor Gray