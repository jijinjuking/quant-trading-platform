# Strategy Engine Phase 1 修复 - 技术设计

> **关联需求**: requirements.md
> **创建日期**: 2026-01-08

---

## 一、问题分析

### 1.1 编译错误详情

```
error[E0308]: mismatched types
  --> strategy_registry.rs
   |
   | handle.execute(request).await
   |                ^^^^^^^ expected `strategy_handle::ExecutionRequest`, 
   |                        found `strategy_runtime::ExecutionRequest`
```

**原因**: Rust 编译器认为这是两个不同的类型，即使它们的定义相同。

### 1.2 类型定义位置

| 类型 | 定义位置 | 使用位置 |
|------|----------|----------|
| `ExecutionRequest` | `strategy_runtime.rs` | `strategy_handle.rs`, `strategy_registry.rs` |
| `ExecutionResult` | `strategy_runtime.rs` | `strategy_handle.rs`, `strategy_registry.rs` |
| `TradeIntent` | `strategy_runtime.rs` | `strategy_handle.rs` (re-export) |

### 1.3 当前导入方式

```rust
// strategy_handle.rs
use super::strategy_runtime::{ExecutionRequest, ExecutionResult, RuntimeHandle};

// strategy_registry.rs
use crate::domain::model::strategy_runtime::{ExecutionRequest, ExecutionResult};
```

**问题**: 两个文件使用不同的导入路径，但应该是同一类型。

---

## 二、解决方案

### 2.1 方案选择

**方案 A**: 统一通过 `domain/model/mod.rs` 导出（推荐）
- 优点：清晰的导出路径，易于维护
- 缺点：需要修改多个文件

**方案 B**: 所有文件使用相同的绝对路径
- 优点：改动小
- 缺点：路径冗长，不够优雅

**选择**: 方案 A

### 2.2 修改计划

#### Step 1: 修改 `domain/model/mod.rs`

```rust
//! # 领域模型 (Domain Model)

pub mod failure_record;
pub mod lifecycle_state;
pub mod strategy_handle;
pub mod strategy_metadata;
pub mod strategy_runtime;

// 统一导出核心类型（避免类型不匹配问题）
pub use strategy_runtime::{
    ExecutionRequest,
    ExecutionResult,
    TradeIntent,
    RuntimeHandle,
    RuntimeExitReason,
    RuntimeCommand,
    StrategyExecutor,
    spawn_runtime,
};

pub use lifecycle_state::{
    LifecycleState,
    LifecycleTransition,
    LifecycleTransitionError,
};

pub use strategy_handle::StrategyHandle;
pub use strategy_metadata::{StrategyMetadata, StrategyKind, MarketType};
pub use failure_record::{FailureRecord, FailureType, FailureHistory};
```

#### Step 2: 修改 `strategy_handle.rs` 导入

```rust
// 修改前
use super::strategy_runtime::{ExecutionRequest, ExecutionResult, RuntimeHandle};

// 修改后（使用相对路径，因为在同一模块内）
use super::strategy_runtime::{RuntimeHandle};
// ExecutionRequest/ExecutionResult 通过 mod.rs 统一导出
```

实际上，由于 `strategy_handle.rs` 和 `strategy_runtime.rs` 在同一目录，使用 `super::` 是正确的。问题在于 `strategy_registry.rs` 的导入。

#### Step 3: 修改 `strategy_registry.rs` 导入

```rust
// 修改前
use crate::domain::model::strategy_runtime::{ExecutionRequest, ExecutionResult};

// 修改后
use crate::domain::model::{ExecutionRequest, ExecutionResult};
```

#### Step 4: 移除未使用的导入

```rust
// 删除这行
use rust_decimal::Decimal;
```

---

## 三、代码变更详情

### 3.1 `domain/model/mod.rs` 完整内容

```rust
//! # 领域模型 (Domain Model)
//!
//! Strategy Engine 的核心领域模型。

pub mod failure_record;
pub mod lifecycle_state;
pub mod strategy_handle;
pub mod strategy_metadata;
pub mod strategy_runtime;

// ============================================================================
// 统一导出（解决类型不匹配问题）
// ============================================================================

// 执行相关类型
pub use strategy_runtime::{
    ExecutionRequest,
    ExecutionResult,
    TradeIntent,
    RuntimeHandle,
    RuntimeExitReason,
    RuntimeCommand,
    StrategyExecutor,
    spawn_runtime,
};

// 生命周期相关类型
pub use lifecycle_state::{
    LifecycleState,
    LifecycleTransition,
    LifecycleTransitionError,
};

// 句柄
pub use strategy_handle::StrategyHandle;

// 元数据
pub use strategy_metadata::{
    StrategyMetadata,
    StrategyKind,
    MarketType,
};

// 故障记录
pub use failure_record::{
    FailureRecord,
    FailureType,
    FailureHistory,
};
```

### 3.2 `strategy_registry.rs` 导入修改

```rust
// 修改前
use crate::domain::model::lifecycle_state::LifecycleState;
use crate::domain::model::strategy_handle::StrategyHandle;
use crate::domain::model::strategy_metadata::{MarketType, StrategyKind};
use crate::domain::model::strategy_runtime::{ExecutionRequest, ExecutionResult};

// 修改后
use crate::domain::model::{
    ExecutionRequest,
    ExecutionResult,
    LifecycleState,
    MarketType,
    StrategyKind,
    StrategyHandle,
    StrategyMetadata,
};
```

### 3.3 测试代码保持不变

测试代码中的 `StrategyHandle::new(metadata)` 是正确的，因为：
- `new()` 创建无 Runtime 的句柄
- 测试只测试生命周期管理，不测试执行
- 执行测试需要使用 `with_runtime()`

---

## 四、验证步骤

### 4.1 编译验证

```bash
cd services/strategy-engine
cargo check
```

### 4.2 测试验证

```bash
cargo test -p strategy-engine
```

### 4.3 预期结果

- 无编译错误
- 所有测试通过
- 无 `unwrap()`/`expect()` 警告

---

## 五、回滚计划

如果修改导致其他问题：

1. 恢复原始 `mod.rs`
2. 恢复原始导入语句
3. 分析新问题原因

---

## 六、后续优化（不在本阶段）

1. 考虑将 `ExecutionRequest`/`ExecutionResult` 移到 `shared/` 供其他服务使用
2. 添加更多单元测试
3. 完善文档注释
