# Arc在量化交易系统中的应用示例

## 🎯 实际使用场景

### 1. 共享配置数据
```rust
use std::sync::Arc;
use std::thread;

// 交易配置
#[derive(Debug)]
struct TradingConfig {
    max_position_size: f64,
    risk_limit: f64,
    symbols: Vec<String>,
}

fn main() {
    // 创建共享配置
    let config = Arc::new(TradingConfig {
        max_position_size: 10000.0,
        risk_limit: 0.02,
        symbols: vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()],
    });

    // 多个服务共享同一配置
    let config_for_strategy = Arc::clone(&config);
    let config_for_risk = Arc::clone(&config);
    let config_for_trading = Arc::clone(&config);

    // 策略引擎线程
    let strategy_handle = thread::spawn(move || {
        println!("策略引擎使用配置: {:?}", config_for_strategy.symbols);
    });

    // 风险管理线程
    let risk_handle = thread::spawn(move || {
        println!("风险管理使用配置: 风险限制 = {}", config_for_risk.risk_limit);
    });

    // 交易执行线程
    let trading_handle = thread::spawn(move || {
        println!("交易执行使用配置: 最大仓位 = {}", config_for_trading.max_position_size);
    });

    strategy_handle.join().unwrap();
    risk_handle.join().unwrap();
    trading_handle.join().unwrap();
}
```

### 2. 共享数据库连接池
```rust
use std::sync::Arc;
use bb8_postgres::{bb8::Pool, PostgresConnectionManager};

// 在我们的Strategy Engine中
pub struct AppState {
    pub db_pool: Arc<Pool<PostgresConnectionManager<NoTls>>>,  // 👈 这里用Arc
    pub config: Arc<StrategyEngineConfig>,                     // 👈 这里也用Arc
    pub metrics: Arc<AppMetrics>,                              // 👈 这里也用Arc
}

// 多个handler可以安全地共享同一个数据库连接池
async fn create_strategy(State(state): State<AppState>) {
    let conn = state.db_pool.get().await.unwrap();  // 安全访问
    // 执行数据库操作...
}

async fn list_strategies(State(state): State<AppState>) {
    let conn = state.db_pool.get().await.unwrap();  // 安全访问
    // 执行数据库操作...
}
```

### 3. 共享市场数据
```rust
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

// 市场数据结构
#[derive(Debug, Clone)]
struct MarketData {
    symbol: String,
    price: f64,
    volume: f64,
    timestamp: u64,
}

// 共享的市场数据存储
type SharedMarketData = Arc<RwLock<HashMap<String, MarketData>>>;

fn main() {
    // 创建共享市场数据
    let market_data: SharedMarketData = Arc::new(RwLock::new(HashMap::new()));

    // 数据接收线程
    let data_receiver = Arc::clone(&market_data);
    let receiver_handle = thread::spawn(move || {
        loop {
            // 模拟接收市场数据
            let new_data = MarketData {
                symbol: "BTCUSDT".to_string(),
                price: 50000.0,
                volume: 1.5,
                timestamp: 1640995200,
            };

            // 写入数据
            let mut data = data_receiver.write().unwrap();
            data.insert("BTCUSDT".to_string(), new_data);
            
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    // 策略计算线程
    let data_consumer = Arc::clone(&market_data);
    let consumer_handle = thread::spawn(move || {
        loop {
            // 读取数据
            let data = data_consumer.read().unwrap();
            if let Some(btc_data) = data.get("BTCUSDT") {
                println!("策略使用BTC价格: {}", btc_data.price);
            }
            
            thread::sleep(std::time::Duration::from_secs(2));
        }
    });

    // 让线程运行一段时间
    thread::sleep(std::time::Duration::from_secs(10));
}
```

## 🔍 Arc vs 其他类型对比

### Arc vs Rc
```rust
use std::rc::Rc;      // 单线程引用计数
use std::sync::Arc;   // 多线程引用计数

// Rc - 只能在单线程中使用
let rc_data = Rc::new(vec![1, 2, 3]);
// let rc_clone = Rc::clone(&rc_data);  // 不能跨线程

// Arc - 可以在多线程中使用
let arc_data = Arc::new(vec![1, 2, 3]);
let arc_clone = Arc::clone(&arc_data);  // 可以跨线程
```

### Arc vs Box
```rust
use std::sync::Arc;

// Box - 独占所有权
let box_data = Box::new(vec![1, 2, 3]);
// let box_clone = box_data.clone();  // 错误！Box不能clone

// Arc - 共享所有权
let arc_data = Arc::new(vec![1, 2, 3]);
let arc_clone = Arc::clone(&arc_data);  // 正确！可以有多个所有者
```

## 🚀 在Strategy Engine中的实际用途

### 1. 共享应用状态
```rust
// 在handlers/mod.rs中
pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health_check))
        .route("/strategies", post(create_strategy))
        // 每个handler都会收到AppState的clone
        // 但实际上共享的是Arc包装的数据
}
```

### 2. 服务间通信
```rust
// 多个服务共享同一个指标收集器
let metrics = Arc::new(AppMetrics::new());
let metrics_for_strategy = Arc::clone(&metrics);
let metrics_for_trading = Arc::clone(&metrics);

// 策略服务记录指标
metrics_for_strategy.increment_counter("strategy_executed");

// 交易服务记录指标
metrics_for_trading.increment_counter("trade_executed");
```

## 💡 Arc的优势

### 1. 内存效率
- 只有一份数据副本
- 多个引用共享同一内存
- 自动垃圾回收

### 2. 线程安全
- 原子操作保证引用计数安全
- 可以安全地在线程间传递
- 避免数据竞争

### 3. 零成本抽象
- 编译时优化
- 运行时开销极小
- 接近原生指针性能

## ⚠️ 注意事项

### 1. 循环引用
```rust
use std::sync::{Arc, Weak};

// 避免循环引用，使用Weak
struct Parent {
    children: Vec<Arc<Child>>,
}

struct Child {
    parent: Weak<Parent>,  // 使用Weak避免循环引用
}
```

### 2. 不可变性
```rust
// Arc内的数据默认不可变
let data = Arc::new(vec![1, 2, 3]);
// data.push(4);  // 错误！不能修改

// 需要可变性时使用Mutex或RwLock
let data = Arc::new(Mutex::new(vec![1, 2, 3]));
let mut guard = data.lock().unwrap();
guard.push(4);  // 正确！
```

## 🎯 总结

Arc是Rust中实现**安全多线程数据共享**的核心工具：

- 🔒 **线程安全** - 原子操作保证安全
- 🚀 **高性能** - 零成本抽象
- 💾 **内存高效** - 共享而非复制
- 🛡️ **自动管理** - 引用计数自动释放内存

在量化交易系统中，Arc让我们能够安全地在多个服务、多个线程之间共享配置、数据库连接、市场数据等关键资源！