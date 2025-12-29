# 高级服务监控脚本 v2.0
param(
    [switch]$Continuous = $false,
    [int]$Interval = 30,
    [switch]$Detailed = $false,
    [switch]$ExportMetrics = $false
)

Write-Host "🔍 Advanced Service Monitor v2.0" -ForegroundColor Cyan
Write-Host "=" * 50

# 服务配置
$services = @(
    @{Name="Market Data"; Port=8081; HealthPath="/health"; MetricsPath="/metrics"},
    @{Name="Trading Engine"; Port=8082; HealthPath="/health"; MetricsPath="/metrics"},
    @{Name="Strategy Engine"; Port=8083; HealthPath="/health"; MetricsPath="/metrics"},
    @{Name="User Management"; Port=8084; HealthPath="/health"; MetricsPath="/metrics"},
    @{Name="Risk Management"; Port=8085; HealthPath="/health"; MetricsPath="/metrics"},
    @{Name="Notification"; Port=8086; HealthPath="/health"; MetricsPath="/metrics"}
)

$infrastructure = @(
    @{Name="PostgreSQL"; Port=5432; Type="Database"},
    @{Name="Redis"; Port=6379; Type="Cache"},
    @{Name="ClickHouse"; Port=8123; Type="Analytics"},
    @{Name="Prometheus"; Port=9090; Type="Monitoring"},
    @{Name="Grafana"; Port=3000; Type="Visualization"}
)

function Test-ServiceHealth {
    param($Service)
    
    try {
        $url = "http://localhost:$($Service.Port)$($Service.HealthPath)"
        $response = Invoke-RestMethod -Uri $url -TimeoutSec 5 -ErrorAction Stop
        
        return @{
            Status = "Healthy"
            ResponseTime = (Measure-Command { Invoke-RestMethod -Uri $url -TimeoutSec 5 }).TotalMilliseconds
            Details = $response
        }
    } catch {
        return @{
            Status = "Unhealthy"
            ResponseTime = -1
            Error = $_.Exception.Message
        }
    }
}

function Get-ServiceMetrics {
    param($Service)
    
    try {
        $url = "http://localhost:$($Service.Port)$($Service.MetricsPath)"
        $metrics = Invoke-RestMethod -Uri $url -TimeoutSec 5 -ErrorAction Stop
        
        # 解析关键指标
        $parsedMetrics = @{}
        $metrics -split "`n" | ForEach-Object {
            if ($_ -match '^(\w+)(?:\{[^}]*\})?\s+(.+)$') {
                $parsedMetrics[$matches[1]] = $matches[2]
            }
        }
        
        return $parsedMetrics
    } catch {
        return @{}
    }
}

function Test-InfrastructureHealth {
    param($Infrastructure)
    
    $connection = Test-NetConnection -ComputerName localhost -Port $Infrastructure.Port -WarningAction SilentlyContinue
    return @{
        Status = if ($connection.TcpTestSucceeded) { "Running" } else { "Down" }
        Port = $Infrastructure.Port
        Type = $Infrastructure.Type
    }
}

function Show-MonitoringResults {
    param($Results, $InfraResults)
    
    Clear-Host
    Write-Host "🔍 Service Health Monitor - $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')" -ForegroundColor Cyan
    Write-Host "=" * 70
    
    # 微服务状态
    Write-Host ""
    Write-Host "🚀 Microservices Status:" -ForegroundColor Yellow
    Write-Host "Service Name".PadRight(20) + "Status".PadRight(12) + "Response Time".PadRight(15) + "Details" -ForegroundColor Gray
    Write-Host "-" * 70 -ForegroundColor Gray
    
    $healthyCount = 0
    foreach ($result in $Results) {
        $statusColor = if ($result.Health.Status -eq "Healthy") { "Green"; $healthyCount++ } else { "Red" }
        $responseTime = if ($result.Health.ResponseTime -gt 0) { "$([math]::Round($result.Health.ResponseTime, 2))ms" } else { "N/A" }
        
        Write-Host $result.Service.Name.PadRight(20) -NoNewline
        Write-Host $result.Health.Status.PadRight(12) -ForegroundColor $statusColor -NoNewline
        Write-Host $responseTime.PadRight(15) -NoNewline
        
        if ($result.Health.Status -eq "Healthy" -and $Detailed) {
            Write-Host "OK" -ForegroundColor Green
        } elseif ($result.Health.Status -eq "Unhealthy") {
            Write-Host $result.Health.Error -ForegroundColor Red
        } else {
            Write-Host "OK" -ForegroundColor Green
        }
    }
    
    # 基础设施状态
    Write-Host ""
    Write-Host "🏗️  Infrastructure Status:" -ForegroundColor Yellow
    Write-Host "Service Name".PadRight(20) + "Type".PadRight(15) + "Status".PadRight(12) + "Port" -ForegroundColor Gray
    Write-Host "-" * 70 -ForegroundColor Gray
    
    $infraHealthyCount = 0
    foreach ($result in $InfraResults) {
        $statusColor = if ($result.Health.Status -eq "Running") { "Green"; $infraHealthyCount++ } else { "Red" }
        
        Write-Host $result.Infrastructure.Name.PadRight(20) -NoNewline
        Write-Host $result.Infrastructure.Type.PadRight(15) -NoNewline
        Write-Host $result.Health.Status.PadRight(12) -ForegroundColor $statusColor -NoNewline
        Write-Host $result.Health.Port
    }
    
    # 总体状态
    Write-Host ""
    Write-Host "📊 Overall Status:" -ForegroundColor Yellow
    $totalServices = $Results.Count
    $totalInfra = $InfraResults.Count
    $overallHealth = ($healthyCount + $infraHealthyCount) / ($totalServices + $totalInfra) * 100
    
    Write-Host "  Microservices: $healthyCount/$totalServices healthy" -ForegroundColor $(if ($healthyCount -eq $totalServices) { "Green" } else { "Yellow" })
    Write-Host "  Infrastructure: $infraHealthyCount/$totalInfra running" -ForegroundColor $(if ($infraHealthyCount -eq $totalInfra) { "Green" } else { "Yellow" })
    Write-Host "  Overall Health: $([math]::Round($overallHealth, 1))%" -ForegroundColor $(if ($overallHealth -ge 90) { "Green" } elseif ($overallHealth -ge 70) { "Yellow" } else { "Red" })
    
    # 详细指标
    if ($Detailed) {
        Write-Host ""
        Write-Host "📈 Service Metrics:" -ForegroundColor Yellow
        foreach ($result in $Results) {
            if ($result.Metrics.Count -gt 0) {
                Write-Host "  $($result.Service.Name):" -ForegroundColor Cyan
                $result.Metrics.GetEnumerator() | Sort-Object Name | ForEach-Object {
                    Write-Host "    $($_.Key): $($_.Value)" -ForegroundColor Gray
                }
            }
        }
    }
    
    Write-Host ""
    Write-Host "🔄 Last Updated: $(Get-Date -Format 'HH:mm:ss')" -ForegroundColor Gray
    if ($Continuous) {
        Write-Host "⏱️  Next update in $Interval seconds (Press Ctrl+C to stop)" -ForegroundColor Gray
    }
}

# 主监控循环
do {
    # 检查微服务
    $results = @()
    foreach ($service in $services) {
        $health = Test-ServiceHealth -Service $service
        $metrics = if ($Detailed) { Get-ServiceMetrics -Service $service } else { @{} }
        
        $results += @{
            Service = $service
            Health = $health
            Metrics = $metrics
        }
    }
    
    # 检查基础设施
    $infraResults = @()
    foreach ($infra in $infrastructure) {
        $health = Test-InfrastructureHealth -Infrastructure $infra
        
        $infraResults += @{
            Infrastructure = $infra
            Health = $health
        }
    }
    
    # 显示结果
    Show-MonitoringResults -Results $results -InfraResults $infraResults
    
    # 导出指标
    if ($ExportMetrics) {
        $timestamp = Get-Date -Format "yyyyMMdd-HHmmss"
        $exportData = @{
            Timestamp = Get-Date
            Services = $results
            Infrastructure = $infraResults
        }
        $exportData | ConvertTo-Json -Depth 10 | Out-File "logs/metrics-$timestamp.json"
    }
    
    if ($Continuous) {
        Start-Sleep -Seconds $Interval
    }
} while ($Continuous)

# 返回健康状态
$healthyServices = ($results | Where-Object { $_.Health.Status -eq "Healthy" }).Count
$totalServices = $results.Count
$healthPercentage = ($healthyServices / $totalServices) * 100

if ($healthPercentage -ge 80) {
    exit 0
} else {
    exit 1
}