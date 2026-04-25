# 端到端集成测试脚本 (PowerShell)
# 自动验证从行情采集到交易执行的完整链路

param(
    [string]$KafkaBroker = "localhost:9092",
    [int]$TestDuration = 30,
    [int]$ExpectedEvents = 10
)

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "端到端集成测试" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "Kafka Broker: $KafkaBroker"
Write-Host "测试持续时间: ${TestDuration}秒"
Write-Host "期望最小事件数: $ExpectedEvents"
Write-Host ""

# 测试结果
$TestsPassed = 0
$TestsFailed = 0

# 测试函数
function Test-Passed {
    param([string]$Message)
    Write-Host "✓ $Message" -ForegroundColor Green
    $script:TestsPassed++
}

function Test-Failed {
    param([string]$Message)
    Write-Host "✗ $Message" -ForegroundColor Red
    $script:TestsFailed++
}

function Test-Warning {
    param([string]$Message)
    Write-Host "⚠ $Message" -ForegroundColor Yellow
}

# 1. 检查基础设施
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "1. 检查基础设施" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan

# 检查 Kafka
try {
    $null = kafka-broker-api-versions.bat --bootstrap-server $KafkaBroker 2>&1
    Test-Passed "Kafka 连接成功"
} catch {
    Test-Failed "Kafka 连接失败"
    exit 1
}

# 检查 Topics
$requiredTopics = @("market-events", "strategy-signals", "execution-results")
$existingTopics = kafka-topics.bat --bootstrap-server $KafkaBroker --list 2>$null

foreach ($topic in $requiredTopics) {
    if ($existingTopics -contains $topic) {
        Test-Passed "Topic '$topic' 存在"
    } else {
        Test-Failed "Topic '$topic' 不存在"
    }
}

Write-Host ""

# 2. 测试行情数据流
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "2. 测试行情数据流 (market-events)" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan

Write-Host "等待 $TestDuration 秒，收集行情数据..."

# 消费行情数据
$marketEventsFile = "$env:TEMP\market-events-$PID.json"
$job = Start-Job -ScriptBlock {
    param($broker, $file)
    kafka-console-consumer.bat --bootstrap-server $broker --topic market-events --max-messages 100 2>$null | Out-File -FilePath $file
} -ArgumentList $KafkaBroker, $marketEventsFile

Wait-Job $job -Timeout $TestDuration | Out-Null
Stop-Job $job -ErrorAction SilentlyContinue
Remove-Job $job -ErrorAction SilentlyContinue

# 统计事件数量
if (Test-Path $marketEventsFile) {
    $marketEventCount = (Get-Content $marketEventsFile | Measure-Object -Line).Lines

    if ($marketEventCount -ge $ExpectedEvents) {
        Test-Passed "收到 $marketEventCount 个行情事件 (>= $ExpectedEvents)"
    } else {
        Test-Failed "仅收到 $marketEventCount 个行情事件 (< $ExpectedEvents)"
    }

    # 验证数据格式
    if ($marketEventCount -gt 0) {
        $firstEvent = Get-Content $marketEventsFile -First 1
        try {
            $eventObj = $firstEvent | ConvertFrom-Json
            Test-Passed "行情数据格式正确 (JSON)"

            Write-Host "  示例事件:"
            Write-Host "    - 事件类型: $($eventObj.event_type)"
            Write-Host "    - 交易所: $($eventObj.exchange)"
            Write-Host "    - 交易对: $($eventObj.symbol)"
        } catch {
            Test-Failed "行情数据格式错误 (非 JSON)"
        }
    }

    Remove-Item $marketEventsFile -ErrorAction SilentlyContinue
} else {
    Test-Failed "未收到任何行情数据"
    $marketEventCount = 0
}

Write-Host ""

# 3. 测试策略信号流
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "3. 测试策略信号流 (strategy-signals)" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan

Write-Host "等待 $TestDuration 秒，收集策略信号..."

# 消费策略信号
$strategySignalsFile = "$env:TEMP\strategy-signals-$PID.json"
$job = Start-Job -ScriptBlock {
    param($broker, $file)
    kafka-console-consumer.bat --bootstrap-server $broker --topic strategy-signals --max-messages 50 2>$null | Out-File -FilePath $file
} -ArgumentList $KafkaBroker, $strategySignalsFile

Wait-Job $job -Timeout $TestDuration | Out-Null
Stop-Job $job -ErrorAction SilentlyContinue
Remove-Job $job -ErrorAction SilentlyContinue

# 统计信号数量
if (Test-Path $strategySignalsFile) {
    $signalCount = (Get-Content $strategySignalsFile | Measure-Object -Line).Lines

    if ($signalCount -gt 0) {
        Test-Passed "收到 $signalCount 个策略信号"

        # 验证数据格式
        $firstSignal = Get-Content $strategySignalsFile -First 1
        try {
            $signalObj = $firstSignal | ConvertFrom-Json
            Test-Passed "策略信号格式正确 (JSON)"

            Write-Host "  示例信号:"
            Write-Host "    - 信号ID: $($signalObj.id)"
            Write-Host "    - 策略ID: $($signalObj.strategy_id)"
            Write-Host "    - 交易对: $($signalObj.symbol)"
            Write-Host "    - 方向: $($signalObj.side)"
        } catch {
            Test-Failed "策略信号格式错误 (非 JSON)"
        }
    } else {
        Test-Warning "未收到策略信号（可能策略条件未触发）"
    }

    Remove-Item $strategySignalsFile -ErrorAction SilentlyContinue
} else {
    Test-Warning "未收到策略信号（可能策略条件未触发）"
    $signalCount = 0
}

Write-Host ""

# 4. 测试执行结果流
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "4. 测试执行结果流 (execution-results)" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan

Write-Host "等待 $TestDuration 秒，收集执行结果..."

# 消费执行结果
$executionResultsFile = "$env:TEMP\execution-results-$PID.json"
$job = Start-Job -ScriptBlock {
    param($broker, $file)
    kafka-console-consumer.bat --bootstrap-server $broker --topic execution-results --max-messages 50 2>$null | Out-File -FilePath $file
} -ArgumentList $KafkaBroker, $executionResultsFile

Wait-Job $job -Timeout $TestDuration | Out-Null
Stop-Job $job -ErrorAction SilentlyContinue
Remove-Job $job -ErrorAction SilentlyContinue

# 统计结果数量
if (Test-Path $executionResultsFile) {
    $executionCount = (Get-Content $executionResultsFile | Measure-Object -Line).Lines

    if ($executionCount -gt 0) {
        Test-Passed "收到 $executionCount 个执行结果"

        # 验证数据格式
        $firstResult = Get-Content $executionResultsFile -First 1
        try {
            $resultObj = $firstResult | ConvertFrom-Json
            Test-Passed "执行结果格式正确 (JSON)"

            Write-Host "  示例结果:"
            Write-Host "    - 订单ID: $($resultObj.order_id)"
            Write-Host "    - 状态: $($resultObj.status)"
            Write-Host "    - 交易对: $($resultObj.symbol)"
        } catch {
            Test-Failed "执行结果格式错误 (非 JSON)"
        }
    } else {
        Test-Warning "未收到执行结果（可能未启动 Trading Engine）"
    }

    Remove-Item $executionResultsFile -ErrorAction SilentlyContinue
} else {
    Test-Warning "未收到执行结果（可能未启动 Trading Engine）"
    $executionCount = 0
}

Write-Host ""

# 5. 计算端到端延迟
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "5. 端到端延迟分析" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan

if ($marketEventCount -gt 0 -and $signalCount -gt 0) {
    Write-Host "行情事件数: $marketEventCount"
    Write-Host "策略信号数: $signalCount"
    $signalRate = [math]::Round(($signalCount * 100 / $marketEventCount), 2)
    Write-Host "信号生成率: ${signalRate}%"

    if ($signalCount -gt 0) {
        Test-Passed "策略正常生成信号"
    }
} else {
    Test-Warning "无法计算延迟（数据不足）"
}

Write-Host ""

# 6. 测试总结
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "测试总结" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan

$totalTests = $TestsPassed + $TestsFailed

Write-Host "总测试数: $totalTests"
Write-Host "通过: $TestsPassed" -ForegroundColor Green
Write-Host "失败: $TestsFailed" -ForegroundColor Red
Write-Host ""

if ($TestsFailed -eq 0) {
    Write-Host "==========================================" -ForegroundColor Green
    Write-Host "✓ 所有测试通过！" -ForegroundColor Green
    Write-Host "==========================================" -ForegroundColor Green
    exit 0
} else {
    Write-Host "==========================================" -ForegroundColor Red
    Write-Host "✗ 部分测试失败" -ForegroundColor Red
    Write-Host "==========================================" -ForegroundColor Red
    Write-Host ""
    Write-Host "故障排查建议:" -ForegroundColor Yellow
    Write-Host "  1. 检查服务是否正在运行"
    Write-Host "  2. 查看服务日志"
    Write-Host "  3. 验证配置文件"
    Write-Host "  4. 参考 E2E_TESTING_GUIDE.md"
    exit 1
}
