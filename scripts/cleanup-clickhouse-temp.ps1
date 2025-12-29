# ClickHouse临时文件自动清理脚本
# 用于定期清理ClickHouse产生的临时文件

param(
    [string]$DataPath = "data\clickhouse",
    [int]$MaxAgeHours = 0.25,  # 15分钟 (Arc共享架构)
    [switch]$DryRun = $false,
    [switch]$Verbose = $false
)

Write-Host "=== ClickHouse临时文件清理脚本 ===" -ForegroundColor Green
Write-Host "数据路径: $DataPath" -ForegroundColor Cyan
Write-Host "最大保留时间: $MaxAgeHours 小时" -ForegroundColor Cyan
Write-Host "模拟运行: $DryRun" -ForegroundColor Cyan
Write-Host ""

# 检查数据路径是否存在
if (-not (Test-Path $DataPath)) {
    Write-Host "错误: 数据路径不存在: $DataPath" -ForegroundColor Red
    exit 1
}

# 计算截止时间
$CutoffTime = (Get-Date).AddHours(-$MaxAgeHours)
Write-Host "清理截止时间: $CutoffTime" -ForegroundColor Yellow

# 查找临时文件和目录
Write-Host "正在扫描临时文件..." -ForegroundColor Yellow

# 查找tmp_insert目录
$TmpInsertDirs = Get-ChildItem -Path $DataPath -Recurse -Directory -ErrorAction SilentlyContinue | 
    Where-Object { $_.Name -match "tmp_insert_\d+" -and $_.CreationTime -lt $CutoffTime }

# 查找其他临时文件
$TmpFiles = Get-ChildItem -Path $DataPath -Recurse -File -ErrorAction SilentlyContinue | 
    Where-Object { 
        ($_.Name -match "\.tmp$" -or $_.Name -match "\.temp$" -or $_.Name -match "^tmp_") -and 
        $_.CreationTime -lt $CutoffTime 
    }

# 统计信息
$TotalDirs = $TmpInsertDirs.Count
$TotalFiles = $TmpFiles.Count
$TotalSize = 0

if ($TmpInsertDirs) {
    $TotalSize += ($TmpInsertDirs | ForEach-Object { 
        (Get-ChildItem $_.FullName -Recurse -File -ErrorAction SilentlyContinue | 
         Measure-Object -Property Length -Sum).Sum 
    } | Measure-Object -Sum).Sum
}

if ($TmpFiles) {
    $TotalSize += ($TmpFiles | Measure-Object -Property Length -Sum).Sum
}

Write-Host ""
Write-Host "扫描结果:" -ForegroundColor Green
Write-Host "  临时目录数量: $TotalDirs" -ForegroundColor White
Write-Host "  临时文件数量: $TotalFiles" -ForegroundColor White
Write-Host "  总大小: $([math]::Round($TotalSize/1MB, 2)) MB" -ForegroundColor White

if ($TotalDirs -eq 0 -and $TotalFiles -eq 0) {
    Write-Host "没有找到需要清理的临时文件。" -ForegroundColor Green
    exit 0
}

if ($DryRun) {
    Write-Host ""
    Write-Host "=== 模拟运行模式 - 不会实际删除文件 ===" -ForegroundColor Yellow
    
    if ($TmpInsertDirs) {
        Write-Host ""
        Write-Host "将要删除的临时目录:" -ForegroundColor Yellow
        $TmpInsertDirs | ForEach-Object {
            Write-Host "  $($_.FullName)" -ForegroundColor Gray
        }
    }
    
    if ($TmpFiles) {
        Write-Host ""
        Write-Host "将要删除的临时文件:" -ForegroundColor Yellow
        $TmpFiles | ForEach-Object {
            Write-Host "  $($_.FullName)" -ForegroundColor Gray
        }
    }
    
    Write-Host ""
    Write-Host "要执行实际清理，请运行: .\cleanup-clickhouse-temp.ps1 -DataPath '$DataPath'" -ForegroundColor Cyan
    exit 0
}

# 执行清理
Write-Host ""
Write-Host "开始清理..." -ForegroundColor Yellow

$DeletedDirs = 0
$DeletedFiles = 0
$Errors = 0

# 清理临时目录
if ($TmpInsertDirs) {
    Write-Host "清理临时目录..." -ForegroundColor Yellow
    foreach ($dir in $TmpInsertDirs) {
        try {
            if ($Verbose) {
                Write-Host "  删除目录: $($dir.FullName)" -ForegroundColor Gray
            }
            Remove-Item -Path $dir.FullName -Recurse -Force -ErrorAction Stop
            $DeletedDirs++
        }
        catch {
            Write-Host "  错误: 无法删除目录 $($dir.FullName) - $($_.Exception.Message)" -ForegroundColor Red
            $Errors++
        }
    }
}

# 清理临时文件
if ($TmpFiles) {
    Write-Host "清理临时文件..." -ForegroundColor Yellow
    foreach ($file in $TmpFiles) {
        try {
            if ($Verbose) {
                Write-Host "  删除文件: $($file.FullName)" -ForegroundColor Gray
            }
            Remove-Item -Path $file.FullName -Force -ErrorAction Stop
            $DeletedFiles++
        }
        catch {
            Write-Host "  错误: 无法删除文件 $($file.FullName) - $($_.Exception.Message)" -ForegroundColor Red
            $Errors++
        }
    }
}

# 清理结果
Write-Host ""
Write-Host "=== 清理完成 ===" -ForegroundColor Green
Write-Host "已删除目录: $DeletedDirs" -ForegroundColor White
Write-Host "已删除文件: $DeletedFiles" -ForegroundColor White
Write-Host "错误数量: $Errors" -ForegroundColor $(if ($Errors -gt 0) { "Red" } else { "White" })
Write-Host "释放空间: $([math]::Round($TotalSize/1MB, 2)) MB" -ForegroundColor White

if ($Errors -gt 0) {
    Write-Host ""
    Write-Host "警告: 清理过程中遇到 $Errors 个错误。" -ForegroundColor Yellow
    Write-Host "这可能是由于文件被占用或权限不足导致的。" -ForegroundColor Yellow
    exit 1
}

Write-Host ""
Write-Host "清理成功完成！" -ForegroundColor Green