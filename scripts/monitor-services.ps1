# Microservice Real-time Monitoring Script
param([int]$RefreshInterval = 5)

function Get-ServiceHealth {
    param([string]$Url, [string]$ServiceName)
    try {
        $response = Invoke-WebRequest -Uri $Url -TimeoutSec 2 -UseBasicParsing
        if ($response.StatusCode -eq 200) {
            return @{Status="Healthy"; Color="Green"; Message="OK"}
        } else {
            return @{Status="Unhealthy"; Color="Yellow"; Message="Status: $($response.StatusCode)"}
        }
    }
    catch {
        return @{Status="Down"; Color="Red"; Message="Not responding"}
    }
}

function Show-ServiceStatus {
    Clear-Host
    Write-Host "=== Microservice Real-time Monitoring ===" -ForegroundColor Cyan
    Write-Host "Update Time: $(Get-Date -Format 'HH:mm:ss')" -ForegroundColor Gray
    Write-Host "Refresh Interval: $RefreshInterval seconds" -ForegroundColor Gray
    Write-Host ""
    
    $services = @(
        @{Name="Market Data"; Port=8081; Path="/health"},
        @{Name="Trading Engine"; Port=8082; Path="/health"},
        @{Name="Strategy Engine"; Port=8083; Path="/health"},
        @{Name="Risk Management"; Port=8085; Path="/health"},
        @{Name="User Management"; Port=8086; Path="/health"},
        @{Name="Notification"; Port=8087; Path="/health"}
    )
    
    Write-Host "Microservice Status:" -ForegroundColor Yellow
    foreach ($service in $services) {
        $url = "http://localhost:$($service.Port)$($service.Path)"
        $health = Get-ServiceHealth -Url $url -ServiceName $service.Name
        Write-Host "  $($service.Name) ($($service.Port)): " -NoNewline
        Write-Host $health.Status -ForegroundColor $health.Color
    }
    
    Write-Host ""
    Write-Host "Press Ctrl+C to exit" -ForegroundColor Gray
}

while ($true) {
    Show-ServiceStatus
    Start-Sleep -Seconds $RefreshInterval
}
