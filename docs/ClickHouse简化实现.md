# ClickHouse简化存储实现

## 概述

为了解决运维遇到的ClickHouse语法错误问题，我们创建了一个简化的ClickHouse存储实现。这个实现采用混合模式，既支持ClickHouse存储，又提供内存存储作为后备方案。

## 实现特性

### 🔄 混合存储模式
- **主存储**: ClickHouse数据库（当连接可用时）
- **后备存储**: 内存存储（当ClickHouse不可用时）
- **自动切换**: 连接失败时自动切换到内存模式

### 🛡️ 错误处理
- **优雅降级**: ClickHouse连接失败时不影响服务运行
- **自动重试**: 定期检查ClickHouse连接状态
- **状态监控**: 实时监控存储连接状态

### 📊 数据管理
- **Tick数据**: 内存保留最近1000条记录
- **K线数据**: 每个交易对保留最近500条记录
- **自动清理**: 定期清理旧数据，防止内存溢出

### 🔍 监控功能
- **健康检查**: 提供存储健康状态检查
- **统计信息**: 实时统计存储数据量
- **连接状态**: 监控ClickHouse连接状态

## 文件结构

```
22/services/market-data/src/storage/
├── clickhouse_store.rs    # 简化的ClickHouse存储实现
├── redis_cache.rs         # Redis缓存实现
└── mod.rs                 # 存储模块导出
```

## 核心方法

### 存储方法
- `store_tick()` - 存储tick数据
- `store_kline()` - 存储K线数据
- `get_recent_ticks()` - 获取最近tick数据
- `get_klines()` - 获取K线数据

### 管理方法
- `health_check()` - 健康检查
- `get_storage_stats()` - 获取统计信息
- `cleanup_old_data()` - 清理旧数据

## 配置要求

### ClickHouse配置（可选）
```toml
[storage.clickhouse]
url = "http://localhost:8123"
database = "market_data"
username = "default"
password = ""
```

### 内存存储配置（自动）
- 无需额外配置
- 自动管理内存使用
- 自动数据清理

## 使用方式

### 初始化
```rust
let config = MarketDataConfig::load()?;
let store = ClickHouseStore::new(config).await?;
```

### 存储数据
```rust
// 存储tick数据
store.store_tick(&tick_data).await?;

// 存储K线数据
store.store_kline(&kline_data).await?;
```

### 查询数据
```rust
// 获取最近tick数据
let ticks = store.get_recent_ticks("BTCUSDT", 100).await?;

// 获取K线数据
let klines = store.get_klines("BTCUSDT", "1m", 100).await?;
```

### 监控状态
```rust
// 健康检查
store.health_check().await?;

// 获取统计信息
let stats = store.get_storage_stats().await?;
println!("ClickHouse连接: {}", stats.is_clickhouse_connected);
println!("内存tick数量: {}", stats.memory_tick_count);
```

## 编译测试

运行编译测试脚本：
```powershell
.\test-clickhouse-compilation.ps1
```

## 优势

1. **零语法错误**: 简化实现避免复杂的ClickHouse客户端语法
2. **高可用性**: 内存后备确保服务不中断
3. **易于维护**: 代码简洁，易于理解和修改
4. **渐进式部署**: 可以先使用内存模式，后续再配置ClickHouse
5. **性能优化**: 内存存储提供极快的读写速度

## 注意事项

1. **内存限制**: 内存存储有数据量限制，适合开发和测试
2. **数据持久性**: 内存数据在服务重启后会丢失
3. **扩展性**: 生产环境建议配置ClickHouse以获得更好的扩展性

## 后续计划

1. **ClickHouse集成**: 当运维解决连接问题后，可以启用ClickHouse存储
2. **性能优化**: 根据实际使用情况优化内存管理
3. **监控增强**: 添加更多监控指标和告警

---

**状态**: ✅ 编译通过，可以部署使用  
**维护者**: 前端集成工程师  
**更新时间**: 2024-12-20