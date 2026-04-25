# Kafka Topics 创建脚本 (PowerShell)
# 用于初始化量化交易平台所需的所有 Kafka Topics

param(
    [string]$KafkaBroker = "localhost:9092",
    [int]$Partitions = 3,
    [int]$ReplicationFactor = 1
)

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "Kafka Topics 初始化脚本" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "Broker: $KafkaBroker"
Write-Host "Partitions: $Partitions"
Write-Host "Replication Factor: $ReplicationFactor"
Write-Host ""

# 定义所有需要的 Topics
$topics = @(
    @{Name="market-events"; Description="行情事件 - Market Data Service 产生"},
    @{Name="strategy-signals"; Description="策略信号 - Strategy Engine 产生"},
    @{Name="execution-drafts"; Description="执行草稿 - CopyTrading Service 产生（跟单后）"},
    @{Name="execution-results"; Description="执行结果 - Trading Engine 产生"},
    @{Name="order-events"; Description="订单事件 - Trading Engine 产生"},
    @{Name="commission-records"; Description="分佣记录 - Commission Service 产生"},
    @{Name="risk-alerts"; Description="风控告警 - Risk Management Service 产生"},
    @{Name="notifications"; Description="通知消息 - Notification Service 消费"},
    @{Name="user-events"; Description="用户事件 - User Management Service 产生"}
)

Write-Host "开始创建 Kafka Topics..." -ForegroundColor Green
Write-Host ""

foreach ($topic in $topics) {
    $topicName = $topic.Name
    $description = $topic.Description

    Write-Host "创建 Topic: $topicName" -ForegroundColor Yellow
    Write-Host "  描述: $description"

    # 检查 Topic 是否已存在
    $existingTopics = kafka-topics.bat --bootstrap-server $KafkaBroker --list 2>$null
    if ($existingTopics -contains $topicName) {
        Write-Host "  状态: ✓ 已存在，跳过" -ForegroundColor Gray
    } else {
        # 创建 Topic
        try {
            kafka-topics.bat --bootstrap-server $KafkaBroker `
                --create `
                --topic $topicName `
                --partitions $Partitions `
                --replication-factor $ReplicationFactor `
                --config retention.ms=604800000 `
                --config compression.type=lz4 2>&1 | Out-Null

            Write-Host "  状态: ✓ 创建成功" -ForegroundColor Green
        } catch {
            Write-Host "  状态: ✗ 创建失败: $_" -ForegroundColor Red
        }
    }
    Write-Host ""
}

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "所有 Topics 创建完成！" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host ""

# 列出所有 Topics
Write-Host "当前所有 Topics:" -ForegroundColor Green
kafka-topics.bat --bootstrap-server $KafkaBroker --list

Write-Host ""
Write-Host "查看 Topic 详情:" -ForegroundColor Yellow
Write-Host "  kafka-topics.bat --bootstrap-server $KafkaBroker --describe --topic <topic-name>"
