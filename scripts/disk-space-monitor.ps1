# 50GB硬盘空间监控和清理脚本
# 适用于小型服务器的磁盘空间管理

param(
    [string]$RootPath = ".",
    [int]$WarningThresholdGB = 40,  # 40GB使用量警告
    [int]$CriticalThresholdGB = 45, # 45GB使用量严重警告
    [switch]$AutoClean = $false,
    [switch]$DryRun = $false
)

Write-Host "=== 50GB硬盘空间监控 ===" -ForegroundColor Green
Write-Host "根路径: $RootPath" -ForegroundColor Cyan
Write-Host "警告阈值: $WarningThresholdGB GB" -ForegroundColor Cyan
Write-Host "严重阈值: $CriticalThresholdGB GB" -ForegroundColor Cyan
Write-Host ""

# 获取磁盘使用情况
function Get-DiskUsage {
    param([string]$Path)
    
    $totalSize = 0
    $directories = @()
    
    # 扫描主要目录
    $mainDirs = @("data", "target", "logs", "23")
    
    foreach ($dir in $mainDirs) {
        $dirPath = Join-Path $Path $dir
        if (Test-Path $dirPath) {
            try {
                $size = (Get-ChildItem $dirPath -Recurse -File -ErrorAction SilentlyContinue | 
                        Measure-Object -Property Length -Sum).Sum
                if ($size -gt 0) {
                    $sizeGB = [math]::Round($size / 1GB, 2)
                    $directories += [PSCustomObject]@{
                        Name = $dir
                        SizeGB = $sizeGB
                        Path = $dirPath
                    }
                    $totalSize += $size
                }
            }
            catch {
                Write-Host "警告: 无法扫描目录 $dirPath" -ForegroundColor Yellow
            }
        }
    }
    
    return @{
        TotalSizeGB = [math]::Round($totalSize / 1GB, 2)
        Directories = $directories
    }
}

# 清理函数
function Start-DiskCleanup {
    param([bool]$DryRun)
    
    Write-Host "开始磁盘清理..." -ForegroundColor Yellow
    $cleanedSpace = 0
    
    # 1. 清理Rust编译缓存
    $targetDirs = Get-ChildItem -Path $RootPath -Recurse -Directory -Name "target" -ErrorAction SilentlyContinue
    foreach ($targetDir in $targetDirs) {
        $fullPath = Join-Path $RootPath $targetDir
        if (Test-Path $fullPath) {
            $size = (Get-ChildItem $fullPath -Recurse -File -ErrorAction SilentlyContinue | 
                    Measure-Object -Property Length -Sum).Sum
            
            if ($size -gt 100MB) {
                Write-Host "  清理编译缓存: $fullPath ($([math]::Round($size/1MB, 2)) MB)" -ForegroundColor Gray
                if (-not $DryRun) {
                    Remove-Item $fullPath -Recurse -Force -ErrorAction SilentlyContinue
                }
                $cleanedSpace += $size
            }
        }
    }
    
    # 2. 清理ClickHouse临时文件
    $clickhouseData = Join-Path $RootPath "data\clickhouse"
    if (Test-Path $clickhouseData) {
        $tmpDirs = Get-ChildItem $clickhouseData -Recurse -Directory | 
                   Where-Object { $_.Name -match "tmp_" }
        
        foreach ($tmpDir in $tmpDirs) {
            $size = (Get-ChildItem $tmpDir.FullName -Recurse -File -ErrorAction SilentlyContinue | 
                    Measure-Object -Property Length -Sum).Sum
            
            Write-Host "  清理ClickHouse临时文件: $($tmpDir.Name) ($([math]::Round($size/1MB, 2)) MB)" -ForegroundColor Gray
            if (-not $DryRun) {
                Remove-Item $tmpDir.FullName -Recurse -Force -ErrorAction SilentlyContinue
            }
            $cleanedSpace += $size
        }
    }
    
    # 3. 清理日志文件（保留最近7天）
    $logsDir = Join-Path $RootPath "logs"
    if (Test-Path $logsDir) {
        $oldLogs = Get-ChildItem $logsDir -Recurse -File | 
                   Where-Object { $_.LastWriteTime -lt (Get-Date).AddDays(-7) }
        
        foreach ($log in $oldLogs) {
            Write-Host "  清理旧日志: $($log.Name) ($([math]::Round($log.Length/1MB, 2)) MB)" -ForegroundColor Gray
            if (-not $DryRun) {
                Remove-Item $log.FullName -Force -ErrorAction SilentlyContinue
            }
            $cleanedSpace += $log.Length
        }
    }
    
    # 4. 压缩ClickHouse数据
    Write-Host "  建议: 手动执行ClickHouse OPTIMIZE TABLE命令压缩数据" -ForegroundColor Cyan
    
    return [math]::Round($cleanedSpace / 1GB, 2)
}

# 主逻辑
try {
    $usage = Get-DiskUsage -Path $RootPath
    
    Write-Host "磁盘使用情况:" -ForegroundColor Green
    Write-Host "总使用量: $($usage.TotalSizeGB) GB / 50 GB" -ForegroundColor White
    Write-Host "剩余空间: $([math]::Round(50 - $usage.TotalSizeGB, 2)) GB" -ForegroundColor White
    Write-Host ""
    
    Write-Host "目录详情:" -ForegroundColor Green
    $usage.Directories | Sort-Object SizeGB -Descending | ForEach-Object {
        $color = if ($_.SizeGB -gt 5) { "Red" } elseif ($_.SizeGB -gt 2) { "Yellow" } else { "White" }
        Write-Host "  $($_.Name): $($_.SizeGB) GB" -ForegroundColor $color
    }
    Write-Host ""
    
    # 检查阈值
    if ($usage.TotalSizeGB -gt $CriticalThresholdGB) {
        Write-Host "🚨 严重警告: 磁盘使用量超过 $CriticalThresholdGB GB！" -ForegroundColor Red
        Write-Host "建议立即清理磁盘空间。" -ForegroundColor Red
        
        if ($AutoClean) {
            Write-Host "自动清理已启用..." -ForegroundColor Yellow
            $cleaned = Start-DiskCleanup -DryRun $DryRun
            Write-Host "清理完成，释放空间: $cleaned GB" -ForegroundColor Green
        }
    }
    elseif ($usage.TotalSizeGB -gt $WarningThresholdGB) {
        Write-Host "⚠️  警告: 磁盘使用量超过 $WarningThresholdGB GB" -ForegroundColor Yellow
        Write-Host "建议考虑清理磁盘空间。" -ForegroundColor Yellow
    }
    else {
        Write-Host "✅ 磁盘空间充足" -ForegroundColor Green
    }
    
    # 提供清理建议
    Write-Host ""
    Write-Host "清理建议:" -ForegroundColor Cyan
    Write-Host "1. 运行清理脚本: .\disk-space-monitor.ps1 -AutoClean" -ForegroundColor Gray
    Write-Host "2. 清理编译缓存: cargo clean (在各服务目录)" -ForegroundColor Gray
    Write-Host "3. 压缩ClickHouse: OPTIMIZE TABLE <table_name>" -ForegroundColor Gray
    Write-Host "4. 清理Docker: docker system prune -a" -ForegroundColor Gray
}
catch {
    Write-Host "错误: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}