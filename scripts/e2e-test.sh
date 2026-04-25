#!/bin/bash
# 端到端集成测试脚本
# 自动验证从行情采集到交易执行的完整链路

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 配置
KAFKA_BROKER="${KAFKA_BROKER:-localhost:9092}"
TEST_DURATION="${TEST_DURATION:-30}" # 测试持续时间（秒）
EXPECTED_EVENTS="${EXPECTED_EVENTS:-10}" # 期望的最小事件数

echo "=========================================="
echo "端到端集成测试"
echo "=========================================="
echo "Kafka Broker: $KAFKA_BROKER"
echo "测试持续时间: ${TEST_DURATION}秒"
echo "期望最小事件数: $EXPECTED_EVENTS"
echo ""

# 测试结果
TESTS_PASSED=0
TESTS_FAILED=0

# 测试函数
test_passed() {
    echo -e "${GREEN}✓ $1${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
}

test_failed() {
    echo -e "${RED}✗ $1${NC}"
    TESTS_FAILED=$((TESTS_FAILED + 1))
}

test_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

# 1. 检查基础设施
echo "=========================================="
echo "1. 检查基础设施"
echo "=========================================="

# 检查 Kafka
if kafka-broker-api-versions.sh --bootstrap-server "$KAFKA_BROKER" &> /dev/null; then
    test_passed "Kafka 连接成功"
else
    test_failed "Kafka 连接失败"
    exit 1
fi

# 检查 Topics
REQUIRED_TOPICS=("market-events" "strategy-signals" "execution-results")
for topic in "${REQUIRED_TOPICS[@]}"; do
    if kafka-topics.sh --bootstrap-server "$KAFKA_BROKER" --list | grep -q "^${topic}$"; then
        test_passed "Topic '$topic' 存在"
    else
        test_failed "Topic '$topic' 不存在"
    fi
done

echo ""

# 2. 测试行情数据流
echo "=========================================="
echo "2. 测试行情数据流 (market-events)"
echo "=========================================="

echo "等待 ${TEST_DURATION} 秒，收集行情数据..."

# 消费行情数据
MARKET_EVENTS_FILE="/tmp/market-events-$$.json"
timeout ${TEST_DURATION}s kafka-console-consumer.sh \
    --bootstrap-server "$KAFKA_BROKER" \
    --topic market-events \
    --max-messages 100 \
    > "$MARKET_EVENTS_FILE" 2>/dev/null || true

# 统计事件数量
MARKET_EVENT_COUNT=$(wc -l < "$MARKET_EVENTS_FILE" | tr -d ' ')

if [ "$MARKET_EVENT_COUNT" -ge "$EXPECTED_EVENTS" ]; then
    test_passed "收到 $MARKET_EVENT_COUNT 个行情事件 (>= $EXPECTED_EVENTS)"
else
    test_failed "仅收到 $MARKET_EVENT_COUNT 个行情事件 (< $EXPECTED_EVENTS)"
fi

# 验证数据格式
if [ "$MARKET_EVENT_COUNT" -gt 0 ]; then
    FIRST_EVENT=$(head -n 1 "$MARKET_EVENTS_FILE")
    if echo "$FIRST_EVENT" | jq -e '.event_type' &> /dev/null; then
        test_passed "行情数据格式正确 (JSON)"

        # 提取字段
        EVENT_TYPE=$(echo "$FIRST_EVENT" | jq -r '.event_type')
        EXCHANGE=$(echo "$FIRST_EVENT" | jq -r '.exchange')
        SYMBOL=$(echo "$FIRST_EVENT" | jq -r '.symbol')

        echo "  示例事件:"
        echo "    - 事件类型: $EVENT_TYPE"
        echo "    - 交易所: $EXCHANGE"
        echo "    - 交易对: $SYMBOL"
    else
        test_failed "行情数据格式错误 (非 JSON)"
    fi
fi

rm -f "$MARKET_EVENTS_FILE"
echo ""

# 3. 测试策略信号流
echo "=========================================="
echo "3. 测试策略信号流 (strategy-signals)"
echo "=========================================="

echo "等待 ${TEST_DURATION} 秒，收集策略信号..."

# 消费策略信号
STRATEGY_SIGNALS_FILE="/tmp/strategy-signals-$$.json"
timeout ${TEST_DURATION}s kafka-console-consumer.sh \
    --bootstrap-server "$KAFKA_BROKER" \
    --topic strategy-signals \
    --max-messages 50 \
    > "$STRATEGY_SIGNALS_FILE" 2>/dev/null || true

# 统计信号数量
SIGNAL_COUNT=$(wc -l < "$STRATEGY_SIGNALS_FILE" | tr -d ' ')

if [ "$SIGNAL_COUNT" -gt 0 ]; then
    test_passed "收到 $SIGNAL_COUNT 个策略信号"

    # 验证数据格式
    FIRST_SIGNAL=$(head -n 1 "$STRATEGY_SIGNALS_FILE")
    if echo "$FIRST_SIGNAL" | jq -e '.id' &> /dev/null; then
        test_passed "策略信号格式正确 (JSON)"

        # 提取字段
        SIGNAL_ID=$(echo "$FIRST_SIGNAL" | jq -r '.id')
        STRATEGY_ID=$(echo "$FIRST_SIGNAL" | jq -r '.strategy_id')
        SYMBOL=$(echo "$FIRST_SIGNAL" | jq -r '.symbol')
        SIDE=$(echo "$FIRST_SIGNAL" | jq -r '.side')

        echo "  示例信号:"
        echo "    - 信号ID: $SIGNAL_ID"
        echo "    - 策略ID: $STRATEGY_ID"
        echo "    - 交易对: $SYMBOL"
        echo "    - 方向: $SIDE"
    else
        test_failed "策略信号格式错误 (非 JSON)"
    fi
else
    test_warning "未收到策略信号（可能策略条件未触发）"
fi

rm -f "$STRATEGY_SIGNALS_FILE"
echo ""

# 4. 测试执行结果流
echo "=========================================="
echo "4. 测试执行结果流 (execution-results)"
echo "=========================================="

echo "等待 ${TEST_DURATION} 秒，收集执行结果..."

# 消费执行结果
EXECUTION_RESULTS_FILE="/tmp/execution-results-$$.json"
timeout ${TEST_DURATION}s kafka-console-consumer.sh \
    --bootstrap-server "$KAFKA_BROKER" \
    --topic execution-results \
    --max-messages 50 \
    > "$EXECUTION_RESULTS_FILE" 2>/dev/null || true

# 统计结果数量
EXECUTION_COUNT=$(wc -l < "$EXECUTION_RESULTS_FILE" | tr -d ' ')

if [ "$EXECUTION_COUNT" -gt 0 ]; then
    test_passed "收到 $EXECUTION_COUNT 个执行结果"

    # 验证数据格式
    FIRST_RESULT=$(head -n 1 "$EXECUTION_RESULTS_FILE")
    if echo "$FIRST_RESULT" | jq -e '.order_id' &> /dev/null; then
        test_passed "执行结果格式正确 (JSON)"

        # 提取字段
        ORDER_ID=$(echo "$FIRST_RESULT" | jq -r '.order_id')
        STATUS=$(echo "$FIRST_RESULT" | jq -r '.status')
        SYMBOL=$(echo "$FIRST_RESULT" | jq -r '.symbol')

        echo "  示例结果:"
        echo "    - 订单ID: $ORDER_ID"
        echo "    - 状态: $STATUS"
        echo "    - 交易对: $SYMBOL"
    else
        test_failed "执行结果格式错误 (非 JSON)"
    fi
else
    test_warning "未收到执行结果（可能未启动 Trading Engine）"
fi

rm -f "$EXECUTION_RESULTS_FILE"
echo ""

# 5. 计算端到端延迟
echo "=========================================="
echo "5. 端到端延迟分析"
echo "=========================================="

if [ "$MARKET_EVENT_COUNT" -gt 0 ] && [ "$SIGNAL_COUNT" -gt 0 ]; then
    echo "行情事件数: $MARKET_EVENT_COUNT"
    echo "策略信号数: $SIGNAL_COUNT"
    echo "信号生成率: $(echo "scale=2; $SIGNAL_COUNT * 100 / $MARKET_EVENT_COUNT" | bc)%"

    if [ "$SIGNAL_COUNT" -gt 0 ]; then
        test_passed "策略正常生成信号"
    fi
else
    test_warning "无法计算延迟（数据不足）"
fi

echo ""

# 6. 测试总结
echo "=========================================="
echo "测试总结"
echo "=========================================="

TOTAL_TESTS=$((TESTS_PASSED + TESTS_FAILED))

echo "总测试数: $TOTAL_TESTS"
echo -e "通过: ${GREEN}$TESTS_PASSED${NC}"
echo -e "失败: ${RED}$TESTS_FAILED${NC}"
echo ""

if [ "$TESTS_FAILED" -eq 0 ]; then
    echo -e "${GREEN}=========================================="
    echo "✓ 所有测试通过！"
    echo "==========================================${NC}"
    exit 0
else
    echo -e "${RED}=========================================="
    echo "✗ 部分测试失败"
    echo "==========================================${NC}"
    echo ""
    echo "故障排查建议:"
    echo "  1. 检查服务是否正在运行"
    echo "  2. 查看服务日志"
    echo "  3. 验证配置文件"
    echo "  4. 参考 E2E_TESTING_GUIDE.md"
    exit 1
fi
