# K线数据架构设计 - 专业级量化交易系统
## Kline Data Architecture Design - Professional Quantitative Trading System

**设计时间**: 2024-12-20  
**架构师**: 窗口1  
**问题发现**: 用户提出的专业架构问题  

---

## 🎯 **核心设计原理**

### **用户的问题完全正确！**
当前系统缺少专业的K线数据架构设计：
- ❌ 直接存储15分钟K线
- ❌ 没有基础数据合成机制
- ❌ 缺乏数据一致性保证

### **正确的架构应该是**:
```
Tick数据 → 1分钟K线 → 合成其他周期 (5m, 15m, 1h, 4h, 1d)
```

---

## 📊 **专业K线数据架构**

### **1. 数据存储层次**
```rust
// 基础数据单位
struct BaseKline {
    symbol: String,
    interval: KlineInterval,  // 固定为1分钟
    open_time: i64,
    close_time: i64,
    open: Decimal,
    high: Decimal,
    low: Decimal,
    close: Decimal,
    volume: Decimal,
    trades_count: u64,
}

// 支持的时间周期
enum KlineInterval {
    OneMinute,    // 基础存储单位
    FiveMinutes,  // 合成
    FifteenMinutes, // 合成
    OneHour,      // 合成
    FourHours,    // 合成
    OneDay,       // 合成
}
```

### **2. 数据合成算法**
```rust
impl KlineAggregator {
    /// 将1分钟K线合成为指定周期
    pub fn aggregate_klines(
        &self,
        base_klines: Vec<BaseKline>,
        target_interval: KlineInterval
    ) -> Result<Kline> {
        let aggregated = Kline {
            symbol: base_klines[0].symbol.clone(),
            interval: target_interval,
            open_time: base_klines[0].open_time,
            close_time: base_klines.last().unwrap().close_time,
            open: base_klines[0].open,
            high: base_klines.iter().map(|k| k.high).max().unwrap(),
            low: base_klines.iter().map(|k| k.low).min().unwrap(),
            close: base_klines.last().unwrap().close,
            volume: base_klines.iter().map(|k| k.volume).sum(),
            trades_count: base_klines.iter().map(|k| k.trades_count).sum(),
        };
        Ok(aggregated)
    }
}
```

---

## 🏗️ **改进的系统架构**

### **数据流设计**:
```
Binance WebSocket
    ↓
Tick数据处理器
    ↓
1分钟K线生成器 (基础数据)
    ↓
┌─────────────────────────────────┐
│     K线合成引擎                  │
├─────────────────────────────────┤
│ • 5分钟K线合成器                │
│ • 15分钟K线合成器               │
│ • 1小时K线合成器                │
│ • 4小时K线合成器                │
│ • 日K线合成器                   │
└─────────────────────────────────┘
    ↓
存储层 (ClickHouse + Redis缓存)
    ↓
API层 (按需提供不同周期数据)
```

### **存储策略**:
```sql
-- 基础表：只存储1分钟K线
CREATE TABLE klines_1m (
    symbol String,
    open_time DateTime64(3),
    close_time DateTime64(3),
    open Decimal64(8),
    high Decimal64(8),
    low Decimal64(8),
    close Decimal64(8),
    volume Decimal64(8),
    trades_count UInt64
) ENGINE = MergeTree()
ORDER BY (symbol, open_time);

-- 视图：实时合成其他周期
CREATE MATERIALIZED VIEW klines_15m AS
SELECT 
    symbol,
    toStartOfInterval(open_time, INTERVAL 15 MINUTE) as open_time,
    toStartOfInterval(open_time, INTERVAL 15 MINUTE) + INTERVAL 15 MINUTE - INTERVAL 1 SECOND as close_time,
    argMin(open, open_time) as open,
    max(high) as high,
    min(low) as low,
    argMax(close, open_time) as close,
    sum(volume) as volume,
    sum(trades_count) as trades_count
FROM klines_1m
GROUP BY symbol, toStartOfInterval(open_time, INTERVAL 15 MINUTE);
```

---

## 🔧 **立即实施方案**

### **Phase 1: 修复当前架构 (今天)**
1. **修改Binance连接器** - 只订阅1分钟K线
2. **实现K线合成器** - 基于1分钟数据合成其他周期
3. **更新API端点** - 支持动态周期查询

### **Phase 2: 优化存储 (明天)**
1. **ClickHouse表结构优化** - 基于1分钟数据的存储设计
2. **Redis缓存策略** - 缓存常用周期的最新数据
3. **数据压缩和分区** - 优化历史数据存储

### **Phase 3: 高级功能 (下周)**
1. **实时K线更新** - 当前未完成K线的实时更新
2. **数据回填机制** - 历史数据的批量处理
3. **数据质量监控** - 确保合成数据的准确性

---

## 💡 **技术实现细节**

### **1. K线合成器实现**
```rust
pub struct KlineAggregator {
    cache: Arc<RwLock<HashMap<String, Vec<BaseKline>>>>,
}

impl KlineAggregator {
    /// 处理新的1分钟K线
    pub async fn process_base_kline(&self, kline: BaseKline) -> Result<()> {
        // 1. 存储基础数据
        self.store_base_kline(&kline).await?;
        
        // 2. 检查是否需要合成其他周期
        self.check_and_aggregate(&kline).await?;
        
        Ok(())
    }
    
    /// 检查并合成其他周期
    async fn check_and_aggregate(&self, base_kline: &BaseKline) -> Result<()> {
        let intervals = vec![
            KlineInterval::FiveMinutes,
            KlineInterval::FifteenMinutes,
            KlineInterval::OneHour,
            KlineInterval::FourHours,
            KlineInterval::OneDay,
        ];
        
        for interval in intervals {
            if self.should_aggregate(base_kline, &interval) {
                let aggregated = self.aggregate_for_interval(base_kline, &interval).await?;
                self.store_aggregated_kline(aggregated).await?;
            }
        }
        
        Ok(())
    }
}
```

### **2. API端点设计**
```rust
// GET /api/v1/klines?symbol=BTCUSDT&interval=15m&limit=200
pub async fn get_klines(
    Query(params): Query<KlineQuery>
) -> Result<Json<Vec<Kline>>, ApiError> {
    match params.interval {
        KlineInterval::OneMinute => {
            // 直接从基础表查询
            get_base_klines(&params).await
        },
        _ => {
            // 实时合成或从缓存获取
            get_or_aggregate_klines(&params).await
        }
    }
}
```

---

## 🎯 **立即行动建议**

### **对窗口2的建议**:
1. **当前优先级** - 先让基本服务跑起来 (15分钟计划继续)
2. **下一步改进** - 服务启动后立即实施K线合成架构
3. **Binance配置** - 修改为只订阅1分钟K线数据

### **配置修改**:
```rust
// 在 binance.rs 中
let kline_streams = vec![
    "btcusdt@kline_1m",  // 只订阅1分钟
    "ethusdt@kline_1m",
    "solusdt@kline_1m",
];

// 移除其他周期的直接订阅
// "btcusdt@kline_15m", // 删除
// "btcusdt@kline_1h",  // 删除
```

---

## 🏆 **架构优势**

### **这种设计的好处**:
1. **数据一致性** - 所有周期基于同一份基础数据
2. **存储效率** - 只存储最小粒度数据
3. **查询灵活性** - 可以生成任意周期的K线
4. **实时性** - 支持实时K线更新
5. **扩展性** - 易于添加新的时间周期

### **符合行业标准**:
- **Binance** - 内部也是基于1分钟数据合成
- **OKX** - 同样的架构设计
- **专业量化平台** - 标准做法

---

**你的问题非常专业！这正是区分业余和专业量化系统的关键架构设计。**

**建议**: 窗口2先完成15分钟的基本服务启动，然后我们立即实施这个专业的K线架构改进！

---

**文档创建时间**: 2024-12-20 16:00  
**优先级**: 高 (服务启动后立即实施)  
**负责人**: 架构师 + 窗口2后端团队