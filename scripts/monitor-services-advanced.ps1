# 企业级服务监控脚本
param(
    [int]$Interval = 30,
    [switch]$Continuous = $false,
    [switch]$Alerts = $false,
    [string]$LogFile = "logs/service-monitor.log"
)

$services = @(
    @{Name="Market Data"; Port=8081; Url="http://localhost:8081/health"; Critical=$true},
    @{Name="Trading Engine"; Port=8082; Url="http://localhost:8082/health"; Critical=$true},
    @{Name="Strategy Engine"; Port=8083; Url="http://localhost:8083/health"; Critical=$true},
    @{Name="User Management"; Port=8084; Url="http://localhost:8084/health"; Critical=$true},
    @{Name="Risk Management"; Port=8085; Url="http://localhost:8085/health"; Critical=$true},
    @{Name="Notification"; Port=8086; Url="http://localhost:8086/health"; Critical=$false}
)

function Write-Log {
    param($Message, $Level = "INFO")
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $logEntry = "[$timestamp] [$Level] $Message"
    Write-Host $logEntry
    if ($LogFile) {
        Add-Content -Path $LogFile -Value $logEntry
    }
}

function Test-ServiceHealth {
    param($Service)
    
    try {
        $response = Invoke-RestMethod -Uri $Service.Url -TimeoutSec 5 -ErrorAction Stop
        
        if ($response.status -eq "healthy" -or $response.success -eq $true) {
            return @{
                Status = "HEALTHY"
                ResponseTime = (Measure-Command { Invoke-RestMethod -Uri $Service.Url -TimeoutSec 5 }).TotalMilliseconds
                Details = $response
            }
        } else {
            return @{
                Status = "UNHEALTHY"
                ResponseTime = 0
                Details = $response
            }
        }
    } catch {
        return @{
            Status = "OFFLINE"
            ResponseTime = 0
            Error = $_.Exception.Message
        }
    }
}

function Send-Alert {
    param($Service, $Status, $Message)
    
    if ($Alerts) {
        Write-Log "🚨 ALERT: $($Service.Name) - $Status - $Message" "ALERT"
        
        # 这里可以添加邮件、Slack、Teams等通知
        # Send-MailMessage -To "admin@company.com" -Subject "Service Alert" -Body $Message
    }
}

function Show-ServiceStatus {
    Write-Host ""
    Write-Host "🔍 Enterprise Service Monitoring Report" -ForegroundColor Cyan
    Write-Host "Time: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')" -ForegroundColor Gray
    Write-Host "=" * 70

    $healthyCount = 0
    $totalCount = $services.Count
    $criticalDown = 0
    $results = @()

    foreach ($service in $services) {
        $health = Test-ServiceHealth -Service $service
        
        $statusColor = switch ($health.Status) {
            "HEALTHY" { "Green"; $healthyCount++ }
            "UNHEALTHY" { "Yellow" }
            "OFFLINE" { "Red"; if ($service.Critical) { $criticalDown++ } }
        }
        
        $icon = switch ($health.Status) {
            "HEALTHY" { "[OK]" }
            "UNHEALTHY" { "[WARN]" }
            "OFFLINE" { "[ERROR]" }
        }
        
        $responseTime = if ($health.ResponseTime -gt 0) { " ($([math]::Round($health.ResponseTime, 2))ms)" } else { "" }
        
        Write-Host "$icon $($service.Name) (Port $($service.Port)): $($health.Status)$responseTime" -ForegroundColor $statusColor
        
        # 发送告警
        if ($health.Status -ne "HEALTHY" -and $service.Critical) {
            Send-Alert -Service $service -Status $health.Status -Message "Critical service $($service.Name) is $($health.Status)"
        }
        
        $results += @{
            Service = $service.Name
            Port = $service.Port
            Status = $health.Status
            ResponseTime = $health.ResponseTime
            Critical = $service.Critical
        }
    }

    Write-Host ""
    
    # 总体状态
    if ($criticalDown -gt 0) {
        Write-Host "🚨 CRITICAL: $criticalDown critical services are down!" -ForegroundColor Red
        Write-Log "CRITICAL: $criticalDown critical services are down!" "CRITICAL"
    } elseif ($healthyCount -eq $totalCount) {
        Write-Host "🎉 All microservices are running perfectly!" -ForegroundColor Green
        Write-Log "All services healthy" "INFO"
    } else {
        Write-Host "⚠️  Some services need attention" -ForegroundColor Yellow
        Write-Log "Some services need attention" "WARNING"
    }
    
    Write-Host "📊 Summary: $healthyCount/$totalCount services healthy" -ForegroundColor $(
        if ($healthyCount -eq $totalCount) { "Green" } 
        elseif ($criticalDown -gt 0) { "Red" } 
        else { "Yellow" }
    )

function Compare-ExchangePerformance {
    Write-Host ""
    Write-Host "📊 Multi-Exchange Performance Comparison" -ForegroundColor Cyan
    Write-Host "=" * 60
    
    $exchanges = @("binance", "okx", "huobi")
    $performanceData = @()
    
    foreach ($exchange in $exchanges) {
        try {
            # 模拟获取交易所性能数据
            $latency = Get-Random -Minimum 10 -Maximum 100
            $throughput = Get-Random -Minimum 1000 -Maximum 10000
            $errorRate = Get-Random -Minimum 0 -Maximum 5
            $dataQuality = Get-Random -Minimum 85 -Maximum 100
            
            $performanceData += @{
                Exchange = $exchange.ToUpper()
                Latency = $latency
                Throughput = $throughput
                ErrorRate = $errorRate
                DataQuality = $dataQuality
            }
            
            $latencyColor = if ($latency -lt 50) { "Green" } elseif ($latency -lt 100) { "Yellow" } else { "Red" }
            $throughputColor = if ($throughput -gt 5000) { "Green" } elseif ($throughput -gt 2000) { "Yellow" } else { "Red" }
            $errorColor = if ($errorRate -lt 1) { "Green" } elseif ($errorRate -lt 3) { "Yellow" } else { "Red" }
            $qualityColor = if ($dataQuality -gt 95) { "Green" } elseif ($dataQuality -gt 85) { "Yellow" } else { "Red" }
            
            Write-Host "$($exchange.ToUpper().PadRight(8)) - " -NoNewline
            Write-Host "Latency: $($latency)ms".PadRight(15) -ForegroundColor $latencyColor -NoNewline
            Write-Host "Throughput: $($throughput)/s".PadRight(20) -ForegroundColor $throughputColor -NoNewline
            Write-Host "Errors: $($errorRate)%".PadRight(12) -ForegroundColor $errorColor -NoNewline
            Write-Host "Quality: $($dataQuality)%" -ForegroundColor $qualityColor
            
        } catch {
            Write-Host "$($exchange.ToUpper().PadRight(8)) - ERROR: Unable to fetch performance data" -ForegroundColor Red
        }
    }
    
    # 显示最佳性能交易所
    Write-Host ""
    $bestLatency = ($performanceData | Sort-Object Latency | Select-Object -First 1).Exchange
    $bestThroughput = ($performanceData | Sort-Object Throughput -Descending | Select-Object -First 1).Exchange
    $bestQuality = ($performanceData | Sort-Object DataQuality -Descending | Select-Object -First 1).Exchange
    
    Write-Host "🏆 Performance Leaders:" -ForegroundColor Green
    Write-Host "  • Best Latency: $bestLatency" -ForegroundColor Gray
    Write-Host "  • Best Throughput: $bestThroughput" -ForegroundColor Gray
    Write-Host "  • Best Data Quality: $bestQuality" -ForegroundColor Gray
    
    return $performanceData
}

    Write-Host ""
    Write-Host "🌐 Service URLs:" -ForegroundColor Cyan
    foreach ($service in $services) {
        Write-Host "  • $($service.Name): $($service.Url)" -ForegroundColor Gray
    }

    # 多交易所性能对比
    $exchangePerformance = Compare-ExchangePerformance

    return @{
        HealthyCount = $healthyCount
        TotalCount = $totalCount
        CriticalDown = $criticalDown
        Results = $results
        ExchangePerformance = $exchangePerformance
        Timestamp = Get-Date
    }
}

# 创建日志目录
if ($LogFile -and -not (Test-Path (Split-Path $LogFile))) {
    New-Item -ItemType Directory -Path (Split-Path $LogFile) -Force | Out-Null
}

Write-Log "Service monitoring started" "INFO"

# 主监控循环
if ($Continuous) {
    Write-Host "🔄 Starting continuous monitoring (every $Interval seconds)" -ForegroundColor Yellow
    Write-Host "Press Ctrl+C to stop" -ForegroundColor Gray
    Write-Host ""
    
    while ($true) {
        $result = Show-ServiceStatus
        
        # 记录状态
        Write-Log "Health check completed: $($result.HealthyCount)/$($result.TotalCount) healthy, $($result.CriticalDown) critical down" "INFO"
        
        Start-Sleep -Seconds $Interval
        Clear-Host
    }
} else {
    # 单次检查
    $result = Show-ServiceStatus
    
    # 返回退出码
    if ($result.CriticalDown -gt 0) {
        exit 2  # 关键服务离线
    } elseif ($result.HealthyCount -eq $result.TotalCount) {
        exit 0  # 全部健康
    } else {
        exit 1  # 部分健康
    }
}