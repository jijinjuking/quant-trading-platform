# Strategy Engine Phase 1 修复规范

> **版本**: 1.0
> **创建日期**: 2026-01-08
> **状态**: 待实施
> **优先级**: 🔴 高（阻塞其他开发）

---

## 一、背景与问题

### 1.1 当前状态

Strategy Engine 的核心模块已搭建，但存在编译错误，阻塞整个系统的集成测试。

### 1.2 发现的问题

| 问题 | 位置 | 严重程度 |
|------|------|----------|
| `ExecutionRequest`/`ExecutionResult` 类型不匹配 | `strategy_registry.rs` vs `strategy_handle.rs` | 🔴 编译失败 |
| `StrategyHandle::new()` 参数数量错误 | `strategy_registry.rs` 测试代码 | 🔴 编译失败 |
| `execute()` 方法返回类型不一致 | `strategy_handle.rs` vs `strategy_runtime.rs` | 🔴 编译失败 |
| 未使用的导入 | `strategy_registry.rs` | 🟡 警告 |

### 1.3 根本原因

1. `strategy_runtime.rs` 定义了 `ExecutionRequest`/`ExecutionResult`
2. `strategy_handle.rs` 从 `strategy_runtime` 导入这些类型
3. `strategy_registry.rs` 也从 `strategy_runtime` 导入，但编译器认为是不同类型
4. `StrategyHandle` 有两个构造函数：`new(metadata)` 和 `with_runtime(metadata, runtime)`
5. 测试代码只传了一个参数，但 `new()` 确实只需要一个参数（问题可能在其他地方）

---

## 二、用户故事

### US-1: 作为开发者，我需要 Strategy Engine 能够编译通过

**验收标准**:
- [ ] `cargo check -p strategy-engine` 无错误
- [ ] `cargo test -p strategy-engine` 全部通过
- [ ] 无 `unwrap()`/`expect()`/`panic!()` 违规

### US-2: 作为开发者，我需要类型定义统一且清晰

**验收标准**:
- [ ] `ExecutionRequest`/`ExecutionResult` 只在一处定义
- [ ] 所有模块使用相同的类型路径
- [ ] 类型导出路径清晰（通过 `mod.rs` 统一导出）

### US-3: 作为开发者，我需要 StrategyHandle 支持有/无 Runtime 两种模式

**验收标准**:
- [ ] `StrategyHandle::new(metadata)` - 无 Runtime（用于测试/占位）
- [ ] `StrategyHandle::with_runtime(metadata, runtime)` - 有 Runtime（生产使用）
- [ ] 测试代码使用正确的构造函数

---

## 三、技术设计

### 3.1 类型统一方案

**目标**: 所有 `ExecutionRequest`/`ExecutionResult` 使用同一定义

**方案**: 在 `domain/model/mod.rs` 中统一导出

```rust
// domain/model/mod.rs
pub mod strategy_runtime;
pub mod strategy_handle;
pub mod lifecycle_state;
// ...

// 统一导出核心类型
pub use strategy_runtime::{
    ExecutionRequest,
    ExecutionResult,
    TradeIntent,
    RuntimeHandle,
    RuntimeExitReason,
    StrategyExecutor,
    spawn_runtime,
};
```

**使用方式**:
```rust
// 其他模块统一使用
use crate::domain::model::{ExecutionRequest, ExecutionResult};
```

### 3.2 StrategyHandle 设计

**当前设计（保持不变）**:
- `new(metadata)` - 创建无 Runtime 的句柄（生命周期管理测试用）
- `with_runtime(metadata, runtime)` - 创建有 Runtime 的句柄（生产用）

**execute() 方法**:
- 无 Runtime 时返回错误
- 有 Runtime 时委托给 RuntimeHandle

### 3.3 StrategyRegistry 修复

**问题**: 导入路径导致类型不匹配

**修复**:
```rust
// 修改前
use crate::domain::model::strategy_runtime::{ExecutionRequest, ExecutionResult};

// 修改后
use crate::domain::model::{ExecutionRequest, ExecutionResult};
```

### 3.4 测试代码修复

**问题**: 测试创建 Handle 后调用 `execute()`，但无 Runtime

**修复**: 测试只测试生命周期管理，不测试执行

---

## 四、实施任务

### Task 1: 统一类型导出 (domain/model/mod.rs)

**文件**: `services/strategy-engine/src/domain/model/mod.rs`

**变更**:
1. 添加 `strategy_runtime` 核心类型的 re-export
2. 确保 `ExecutionRequest`/`ExecutionResult` 只有一个导出路径

### Task 2: 修复 strategy_registry.rs 导入

**文件**: `services/strategy-engine/src/domain/service/strategy_registry.rs`

**变更**:
1. 修改导入路径使用统一导出
2. 移除未使用的 `rust_decimal::Decimal` 导入
3. 确保 `execute_async` 方法类型正确

### Task 3: 修复 strategy_handle.rs

**文件**: `services/strategy-engine/src/domain/model/strategy_handle.rs`

**变更**:
1. 确保 `execute()` 方法签名与 `RuntimeHandle::execute()` 一致
2. 确保返回类型正确

### Task 4: 验证编译

**命令**:
```bash
cargo check -p strategy-engine
cargo test -p strategy-engine
```

---

## 五、验收标准

### 5.1 编译检查
```bash
cargo check -p strategy-engine
# 期望: 无错误，无警告（或仅有可接受的警告）
```

### 5.2 测试检查
```bash
cargo test -p strategy-engine
# 期望: 所有测试通过
```

### 5.3 代码规范检查
- [ ] 无 `unwrap()`/`expect()`/`panic!()`
- [ ] 所有 public 结构体有文档注释
- [ ] 遵循 DDD + Hexagonal 架构

---

## 六、风险与注意事项

### 6.1 不要做的事

| 禁止 | 原因 |
|------|------|
| ❌ 修改 Trading Engine | 已冻结 |
| ❌ 修改 shared/ 类型定义 | 影响其他服务 |
| ❌ 重构整体架构 | 超出范围 |
| ❌ 添加新功能 | 本阶段只修复编译 |

### 6.2 依赖关系

```
strategy_runtime.rs (定义类型)
       ↓
strategy_handle.rs (使用类型)
       ↓
strategy_registry.rs (使用类型)
```

修改顺序：从底层向上层修改

---

## 七、后续阶段

Phase 1 完成后，可进入：

- **Phase 2**: 多用户策略隔离
- **Phase 3**: CopyTrading / Commission 服务

---

**有问题先问，不要猜！**
