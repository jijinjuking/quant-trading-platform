# 设置ClickHouse自动清理定时任务
# 每30分钟执行一次临时文件清理

param(
    [string]$ScriptPath = ".\scripts\cleanup-clickhouse-temp.ps1",
    [int]$IntervalMinutes = 10,  # Arc共享架构，10分钟清理一次
    [switch]$Remove = $false
)

$TaskName = "ClickHouse-TempFile-Cleanup"
$TaskDescription = "自动清理ClickHouse临时文件"

if ($Remove) {
    Write-Host "正在移除定时任务: $TaskName" -ForegroundColor Yellow
    try {
        Unregister-ScheduledTask -TaskName $TaskName -Confirm:$false -ErrorAction Stop
        Write-Host "定时任务已成功移除。" -ForegroundColor Green
    }
    catch {
        Write-Host "移除定时任务失败: $($_.Exception.Message)" -ForegroundColor Red
    }
    exit
}

Write-Host "=== 设置ClickHouse自动清理定时任务 ===" -ForegroundColor Green
Write-Host "任务名称: $TaskName" -ForegroundColor Cyan
Write-Host "脚本路径: $ScriptPath" -ForegroundColor Cyan
Write-Host "执行间隔: $IntervalMinutes 分钟" -ForegroundColor Cyan
Write-Host ""

# 检查脚本是否存在
$FullScriptPath = Resolve-Path $ScriptPath -ErrorAction SilentlyContinue
if (-not $FullScriptPath) {
    Write-Host "错误: 清理脚本不存在: $ScriptPath" -ForegroundColor Red
    exit 1
}

Write-Host "脚本完整路径: $FullScriptPath" -ForegroundColor White

# 检查是否已存在同名任务
$ExistingTask = Get-ScheduledTask -TaskName $TaskName -ErrorAction SilentlyContinue
if ($ExistingTask) {
    Write-Host "发现已存在的任务，正在移除..." -ForegroundColor Yellow
    Unregister-ScheduledTask -TaskName $TaskName -Confirm:$false
}

try {
    # 创建任务动作
    $Action = New-ScheduledTaskAction -Execute "PowerShell.exe" -Argument "-ExecutionPolicy Bypass -File `"$FullScriptPath`""
    
    # 创建触发器 - 每N分钟执行一次
    $Trigger = New-ScheduledTaskTrigger -Once -At (Get-Date) -RepetitionInterval (New-TimeSpan -Minutes $IntervalMinutes) -RepetitionDuration (New-TimeSpan -Days 365)
    
    # 创建任务设置
    $Settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -StartWhenAvailable -RunOnlyIfNetworkAvailable:$false
    
    # 创建任务主体（以系统权限运行）
    $Principal = New-ScheduledTaskPrincipal -UserId "SYSTEM" -LogonType ServiceAccount -RunLevel Highest
    
    # 注册任务
    Register-ScheduledTask -TaskName $TaskName -Action $Action -Trigger $Trigger -Settings $Settings -Principal $Principal -Description $TaskDescription
    
    Write-Host ""
    Write-Host "定时任务创建成功！" -ForegroundColor Green
    Write-Host ""
    Write-Host "任务详情:" -ForegroundColor Yellow
    Write-Host "  任务名称: $TaskName" -ForegroundColor White
    Write-Host "  执行间隔: 每 $IntervalMinutes 分钟" -ForegroundColor White
    Write-Host "  执行用户: SYSTEM" -ForegroundColor White
    Write-Host "  脚本路径: $FullScriptPath" -ForegroundColor White
    Write-Host ""
    Write-Host "管理命令:" -ForegroundColor Yellow
    Write-Host "  查看任务: Get-ScheduledTask -TaskName '$TaskName'" -ForegroundColor Gray
    Write-Host "  启动任务: Start-ScheduledTask -TaskName '$TaskName'" -ForegroundColor Gray
    Write-Host "  停止任务: Stop-ScheduledTask -TaskName '$TaskName'" -ForegroundColor Gray
    Write-Host "  移除任务: .\setup-clickhouse-cleanup-task.ps1 -Remove" -ForegroundColor Gray
    Write-Host ""
    
    # 立即执行一次测试
    Write-Host "正在执行首次清理测试..." -ForegroundColor Yellow
    Start-ScheduledTask -TaskName $TaskName
    
    Start-Sleep -Seconds 3
    
    # 检查任务状态
    $TaskInfo = Get-ScheduledTask -TaskName $TaskName
    Write-Host "任务状态: $($TaskInfo.State)" -ForegroundColor White
    
    Write-Host ""
    Write-Host "设置完成！ClickHouse临时文件将每 $IntervalMinutes 分钟自动清理一次。" -ForegroundColor Green
}
catch {
    Write-Host ""
    Write-Host "创建定时任务失败: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host ""
    Write-Host "可能的解决方案:" -ForegroundColor Yellow
    Write-Host "1. 以管理员身份运行PowerShell" -ForegroundColor Gray
    Write-Host "2. 检查PowerShell执行策略: Get-ExecutionPolicy" -ForegroundColor Gray
    Write-Host "3. 如需要，设置执行策略: Set-ExecutionPolicy RemoteSigned" -ForegroundColor Gray
    exit 1
}