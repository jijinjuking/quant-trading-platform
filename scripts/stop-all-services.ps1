# 停止所有微服务
param(
    [switch]$Force = $false
)

Write-Host "🛑 Stopping Quantitative Trading Platform Microservices" -ForegroundColor Red
Write-Host "=" * 60

$ports = @(8081, 8082, 8083, 8084, 8085, 8086)
$stoppedCount = 0

foreach ($port in $ports) {
    Write-Host "🔍 Checking port $port..." -ForegroundColor Yellow
    
    try {
        # 查找占用端口的进程
        $connections = Get-NetTCPConnection -LocalPort $port -ErrorAction SilentlyContinue
        
        if ($connections) {
            foreach ($connection in $connections) {
                $process = Get-Process -Id $connection.OwningProcess -ErrorAction SilentlyContinue
                
                if ($process) {
                    Write-Host "  📋 Found process: $($process.ProcessName) (PID: $($process.Id))" -ForegroundColor Cyan
                    
                    if ($Force) {
                        Write-Host "  🔥 Force stopping process..." -ForegroundColor Red
                        Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
                    } else {
                        Write-Host "  🛑 Gracefully stopping process..." -ForegroundColor Yellow
                        $process.CloseMainWindow() | Out-Null
                        Start-Sleep -Seconds 2
                        
                        if (-not $process.HasExited) {
                            Write-Host "  🔥 Process didn't exit gracefully, force stopping..." -ForegroundColor Red
                            Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
                        }
                    }
                    
                    Write-Host "  ✅ Process stopped" -ForegroundColor Green
                    $stoppedCount++
                } else {
                    Write-Host "  ⚠️  Could not find process for PID $($connection.OwningProcess)" -ForegroundColor Yellow
                }
            }
        } else {
            Write-Host "  ℹ️  No process found on port $port" -ForegroundColor Gray
        }
    } catch {
        Write-Host "  ❌ Error checking port $port`: $_" -ForegroundColor Red
    }
}

Write-Host ""

# 额外清理：查找可能的Rust/Cargo进程
Write-Host "🧹 Cleaning up any remaining Rust processes..." -ForegroundColor Yellow

$rustProcesses = Get-Process | Where-Object { 
    $_.ProcessName -like "*cargo*" -or 
    $_.ProcessName -like "*rust*" -or
    $_.ProcessName -like "*market-data*" -or
    $_.ProcessName -like "*trading-engine*" -or
    $_.ProcessName -like "*strategy-engine*"
} -ErrorAction SilentlyContinue

if ($rustProcesses) {
    foreach ($process in $rustProcesses) {
        Write-Host "  🔥 Stopping $($process.ProcessName) (PID: $($process.Id))" -ForegroundColor Red
        Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
        $stoppedCount++
    }
} else {
    Write-Host "  ✅ No additional Rust processes found" -ForegroundColor Green
}

Write-Host ""
Write-Host "📊 Summary:" -ForegroundColor Cyan
Write-Host "  • Stopped $stoppedCount processes" -ForegroundColor White
Write-Host "  • Checked ports: $($ports -join ', ')" -ForegroundColor Gray

# 验证所有端口都已释放
Write-Host ""
Write-Host "🔍 Verifying ports are free..." -ForegroundColor Yellow

$stillOccupied = @()
foreach ($port in $ports) {
    $connection = Test-NetConnection -ComputerName localhost -Port $port -WarningAction SilentlyContinue
    if ($connection.TcpTestSucceeded) {
        $stillOccupied += $port
        Write-Host "  ⚠️  Port $port is still occupied" -ForegroundColor Yellow
    } else {
        Write-Host "  ✅ Port $port is free" -ForegroundColor Green
    }
}

Write-Host ""

if ($stillOccupied.Count -eq 0) {
    Write-Host "🎉 All microservices stopped successfully!" -ForegroundColor Green
    Write-Host "✅ All ports are now available" -ForegroundColor Green
} else {
    Write-Host "⚠️  Some ports are still occupied: $($stillOccupied -join ', ')" -ForegroundColor Yellow
    Write-Host "💡 Try running with -Force flag: .\scripts\stop-all-services.ps1 -Force" -ForegroundColor Cyan
}

Write-Host ""
Write-Host "🔧 Next steps:" -ForegroundColor Cyan
Write-Host "  • Start services: .\scripts\start-all-services.ps1" -ForegroundColor Gray
Write-Host "  • Check infrastructure: .\scripts\health-check.ps1" -ForegroundColor Gray