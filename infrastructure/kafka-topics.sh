#!/bin/bash
# Kafka Topics 创建脚本
# 用于初始化量化交易平台所需的所有 Kafka Topics

set -e

# Kafka 配置
KAFKA_BROKER="${KAFKA_BROKER:-localhost:9092}"
PARTITIONS="${PARTITIONS:-3}"
REPLICATION_FACTOR="${REPLICATION_FACTOR:-1}"

echo "=========================================="
echo "Kafka Topics 初始化脚本"
echo "=========================================="
echo "Broker: $KAFKA_BROKER"
echo "Partitions: $PARTITIONS"
echo "Replication Factor: $REPLICATION_FACTOR"
echo ""

# 定义所有需要的 Topics
declare -a TOPICS=(
    # 行情数据
    "market-events:行情事件 - Market Data Service 产生"

    # 策略信号
    "strategy-signals:策略信号 - Strategy Engine 产生"

    # 交易执行
    "execution-drafts:执行草稿 - CopyTrading Service 产生（跟单后）"
    "execution-results:执行结果 - Trading Engine 产生"

    # 订单事件
    "order-events:订单事件 - Trading Engine 产生"

    # 分佣记录
    "commission-records:分佣记录 - Commission Service 产生"

    # 风控告警
    "risk-alerts:风控告警 - Risk Management Service 产生"

    # 通知消息
    "notifications:通知消息 - Notification Service 消费"

    # 用户事件
    "user-events:用户事件 - User Management Service 产生"
)

# 创建 Topics
echo "开始创建 Kafka Topics..."
echo ""

for topic_info in "${TOPICS[@]}"; do
    # 分割 topic 名称和描述
    IFS=':' read -r topic_name description <<< "$topic_info"

    echo "创建 Topic: $topic_name"
    echo "  描述: $description"

    # 检查 Topic 是否已存在
    if kafka-topics.sh --bootstrap-server "$KAFKA_BROKER" --list | grep -q "^${topic_name}$"; then
        echo "  状态: ✓ 已存在，跳过"
    else
        # 创建 Topic
        kafka-topics.sh --bootstrap-server "$KAFKA_BROKER" \
            --create \
            --topic "$topic_name" \
            --partitions "$PARTITIONS" \
            --replication-factor "$REPLICATION_FACTOR" \
            --config retention.ms=604800000 \
            --config compression.type=lz4

        echo "  状态: ✓ 创建成功"
    fi
    echo ""
done

echo "=========================================="
echo "所有 Topics 创建完成！"
echo "=========================================="
echo ""

# 列出所有 Topics
echo "当前所有 Topics:"
kafka-topics.sh --bootstrap-server "$KAFKA_BROKER" --list

echo ""
echo "查看 Topic 详情:"
echo "  kafka-topics.sh --bootstrap-server $KAFKA_BROKER --describe --topic <topic-name>"
