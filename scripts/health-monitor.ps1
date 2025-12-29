# 微服务健康监控脚本
param(
    [switch]$Continuous = $false,
    [int]$Interval = 30
)

$services = @(
    @{Name="Market Data"; Port=8081; Url="http://localhost:8081/health"},
    @{Name="Trading Engine"; Port=8082; Url="http://localhost:8082/health"},
    @{Name="Strategy Engine"; Port=8083; Url="http://localhost:8083/health"},
    @{Name="User Management"; Port=8084; Url="http://localhost:8084/health"},
    @{Name="Risk Management"; Port=8085; Url="http://localhost:8085/health"},
    @{Name="Notification"; Port=8086; Url="http://localhost:8086/health"}
)

function Test-ServiceHealth {
    Write-Host "🔍 Microservices Health Check Report" -ForegroundColor Cyan
    Write-Host "Time: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')" -ForegroundColor Gray
    Write-Host "=" * 60

    $healthyCount = 0
    $totalCount = $services.Count
    $results = @()

    foreach ($service in $services) {
        try {
            $response = Invoke-RestMethod -Uri $service.Url -TimeoutSec 5 -ErrorAction Stop
            if ($response.success -eq $true -or $response.status -eq "healthy" -or $response.data.status -eq "healthy") {
                Write-Host "✅ $($service.Name) (Port $($service.Port)): HEALTHY" -ForegroundColor Green
                $healthyCount++
                $results += @{Service=$service.Name; Status="HEALTHY"; Port=$service.Port}
            } else {
                Write-Host "⚠️  $($service.Name) (Port $($service.Port)): UNHEALTHY" -ForegroundColor Yellow
                $results += @{Service=$service.Name; Status="UNHEALTHY"; Port=$service.Port}
            }
        } catch {
            Write-Host "❌ $($service.Name) (Port $($service.Port)): OFFLINE" -ForegroundColor Red
            $results += @{Service=$service.Name; Status="OFFLINE"; Port=$service.Port}
        }
    }

    Write-Host ""
    Write-Host "📊 Summary: $healthyCount/$totalCount services healthy" -ForegroundColor $(if ($healthyCount -eq $totalCount) { "Green" } elseif ($healthyCount -gt 0) { "Yellow" } else { "Red" })

    if ($healthyCount -eq $totalCount) {
        Write-Host "🎉 All microservices are running perfectly!" -ForegroundColor Green
    } elseif ($healthyCount -gt 0) {
        Write-Host "⚠️  Some services need attention" -ForegroundColor Yellow
    } else {
        Write-Host "🚨 All services are offline!" -ForegroundColor Red
    }

    Write-Host ""
    Write-Host "🌐 Service URLs:" -ForegroundColor Cyan
    foreach ($service in $services) {
        Write-Host "  • $($service.Name): $($service.Url)" -ForegroundColor Gray
    }

    return @{
        HealthyCount = $healthyCount
        TotalCount = $totalCount
        Results = $results
        Timestamp = Get-Date
    }
}

# 主执行逻辑
if ($Continuous) {
    Write-Host "🔄 Starting continuous monitoring (every $Interval seconds)" -ForegroundColor Yellow
    Write-Host "Press Ctrl+C to stop" -ForegroundColor Gray
    Write-Host ""
    
    while ($true) {
        $result = Test-ServiceHealth
        
        if ($result.HealthyCount -lt $result.TotalCount) {
            # 发送告警 (可以扩展为邮件、Slack等)
            Write-Host "🚨 ALERT: $($result.TotalCount - $result.HealthyCount) services are down!" -ForegroundColor Red
        }
        
        Start-Sleep -Seconds $Interval
        Clear-Host
    }
} else {
    # 单次检查
    $result = Test-ServiceHealth
    
    # 返回退出码
    if ($result.HealthyCount -eq $result.TotalCount) {
        exit 0  # 全部健康
    } elseif ($result.HealthyCount -gt 0) {
        exit 1  # 部分健康
    } else {
        exit 2  # 全部离线
    }
}