# 8GB内存+4核CPU系统监控脚本
# 实时监控系统资源使用情况

param(
    [int]$IntervalSeconds = 30,
    [switch]$Continuous = $false,
    [string]$LogFile = "system-monitor.log"
)

Write-Host "=== 8GB系统资源监控 ===" -ForegroundColor Green
Write-Host "监控间隔: $IntervalSeconds 秒" -ForegroundColor Cyan
Write-Host "持续监控: $Continuous" -ForegroundColor Cyan
Write-Host ""

function Get-SystemResources {
    # 获取内存使用情况
    $memory = Get-WmiObject -Class Win32_OperatingSystem
    $totalMemoryGB = [math]::Round($memory.TotalVisibleMemorySize / 1MB, 2)
    $freeMemoryGB = [math]::Round($memory.FreePhysicalMemory / 1MB, 2)
    $usedMemoryGB = [math]::Round($totalMemoryGB - $freeMemoryGB, 2)
    $memoryUsagePercent = [math]::Round(($usedMemoryGB / $totalMemoryGB) * 100, 1)
    
    # 获取CPU使用情况
    $cpu = Get-WmiObject -Class Win32_Processor
    $cpuUsage = (Get-Counter "\Processor(_Total)\% Processor Time").CounterSamples.CookedValue
    $cpuUsagePercent = [math]::Round(100 - $cpuUsage, 1)
    
    # 获取磁盘使用情况
    $disk = Get-WmiObject -Class Win32_LogicalDisk | Where-Object { $_.DriveType -eq 3 }
    $totalDiskGB = [math]::Round(($disk | Measure-Object -Property Size -Sum).Sum / 1GB, 2)
    $freeDiskGB = [math]::Round(($disk | Measure-Object -Property FreeSpace -Sum).Sum / 1GB, 2)
    $usedDiskGB = [math]::Round($totalDiskGB - $freeDiskGB, 2)
    $diskUsagePercent = [math]::Round(($usedDiskGB / $totalDiskGB) * 100, 1)
    
    return @{
        Memory = @{
            Total = $totalMemoryGB
            Used = $usedMemoryGB
            Free = $freeMemoryGB
            UsagePercent = $memoryUsagePercent
        }
        CPU = @{
            Cores = $cpu.NumberOfCores
            UsagePercent = $cpuUsagePercent
        }
        Disk = @{
            Total = $totalDiskGB
            Used = $usedDiskGB
            Free = $freeDiskGB
            UsagePercent = $diskUsagePercent
        }
        Timestamp = Get-Date
    }
}

function Show-ResourceStatus {
    param($Resources)
    
    Clear-Host
    Write-Host "=== 系统资源监控 - $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss') ===" -ForegroundColor Green
    Write-Host ""
    
    # 内存状态
    $memColor = if ($Resources.Memory.UsagePercent -gt 85) { "Red" } 
                elseif ($Resources.Memory.UsagePercent -gt 70) { "Yellow" } 
                else { "Green" }
    
    Write-Host "💾 内存使用情况:" -ForegroundColor Cyan
    Write-Host "   总内存: $($Resources.Memory.Total) GB" -ForegroundColor White
    Write-Host "   已使用: $($Resources.Memory.Used) GB ($($Resources.Memory.UsagePercent)%)" -ForegroundColor $memColor
    Write-Host "   可用: $($Resources.Memory.Free) GB" -ForegroundColor White
    Write-Host ""
    
    # CPU状态
    $cpuColor = if ($Resources.CPU.UsagePercent -gt 80) { "Red" } 
                elseif ($Resources.CPU.UsagePercent -gt 60) { "Yellow" } 
                else { "Green" }
    
    Write-Host "🖥️  CPU使用情况:" -ForegroundColor Cyan
    Write-Host "   核心数: $($Resources.CPU.Cores)" -ForegroundColor White
    Write-Host "   使用率: $($Resources.CPU.UsagePercent)%" -ForegroundColor $cpuColor
    Write-Host ""
    
    # 磁盘状态
    $diskColor = if ($Resources.Disk.UsagePercent -gt 90) { "Red" } 
                 elseif ($Resources.Disk.UsagePercent -gt 80) { "Yellow" } 
                 else { "Green" }
    
    Write-Host "💿 磁盘使用情况:" -ForegroundColor Cyan
    Write-Host "   总容量: $($Resources.Disk.Total) GB" -ForegroundColor White
    Write-Host "   已使用: $($Resources.Disk.Used) GB ($($Resources.Disk.UsagePercent)%)" -ForegroundColor $diskColor
    Write-Host "   可用: $($Resources.Disk.Free) GB" -ForegroundColor White
    Write-Host ""
    
    # Docker容器状态（如果Docker正在运行）
    try {
        $dockerContainers = docker ps --format "table {{.Names}}\t{{.Status}}" 2>$null
        if ($dockerContainers) {
            Write-Host "🐳 Docker容器状态:" -ForegroundColor Cyan
            $dockerContainers | ForEach-Object {
                if ($_ -notmatch "NAMES") {
                    Write-Host "   $_" -ForegroundColor Gray
                }
            }
            Write-Host ""
        }
    }
    catch {
        # Docker未运行或未安装
    }
    
    # 警告和建议
    $warnings = @()
    
    if ($Resources.Memory.UsagePercent -gt 85) {
        $warnings += "⚠️  内存使用率过高 ($($Resources.Memory.UsagePercent)%)"
    }
    
    if ($Resources.CPU.UsagePercent -gt 80) {
        $warnings += "⚠️  CPU使用率过高 ($($Resources.CPU.UsagePercent)%)"
    }
    
    if ($Resources.Disk.UsagePercent -gt 90) {
        $warnings += "🚨 磁盘空间严重不足 ($($Resources.Disk.UsagePercent)%)"
    }
    elseif ($Resources.Disk.UsagePercent -gt 80) {
        $warnings += "⚠️  磁盘空间不足 ($($Resources.Disk.UsagePercent)%)"
    }
    
    if ($warnings.Count -gt 0) {
        Write-Host "警告:" -ForegroundColor Red
        $warnings | ForEach-Object {
            Write-Host "   $_" -ForegroundColor Yellow
        }
        Write-Host ""
        
        Write-Host "建议操作:" -ForegroundColor Cyan
        if ($Resources.Memory.UsagePercent -gt 85) {
            Write-Host "   - 重启高内存使用的服务" -ForegroundColor Gray
            Write-Host "   - 检查内存泄漏" -ForegroundColor Gray
        }
        if ($Resources.Disk.UsagePercent -gt 80) {
            Write-Host "   - 运行磁盘清理: .\disk-space-monitor.ps1 -AutoClean" -ForegroundColor Gray
            Write-Host "   - 清理Docker: docker system prune -a" -ForegroundColor Gray
        }
        Write-Host ""
    }
    else {
        Write-Host "✅ 系统运行正常" -ForegroundColor Green
        Write-Host ""
    }
    
    if ($Continuous) {
        Write-Host "按 Ctrl+C 停止监控..." -ForegroundColor Gray
    }
}

function Write-LogEntry {
    param($Resources, $LogFile)
    
    $logEntry = "$(Get-Date -Format 'yyyy-MM-dd HH:mm:ss'),Memory:$($Resources.Memory.UsagePercent)%,CPU:$($Resources.CPU.UsagePercent)%,Disk:$($Resources.Disk.UsagePercent)%"
    Add-Content -Path $LogFile -Value $logEntry
}

# 主监控循环
try {
    do {
        $resources = Get-SystemResources
        Show-ResourceStatus -Resources $resources
        
        if ($LogFile) {
            Write-LogEntry -Resources $resources -LogFile $LogFile
        }
        
        if ($Continuous) {
            Start-Sleep -Seconds $IntervalSeconds
        }
    } while ($Continuous)
}
catch {
    Write-Host "监控中断: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host "监控结束。" -ForegroundColor Green