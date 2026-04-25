# Strategy Engine Phase 1 修复 - 任务清单

> **关联需求**: requirements.md
> **关联设计**: design.md
> **创建日期**: 2026-01-08

---

## 任务列表

### Task 1: 更新 domain/model/mod.rs 统一导出类型

- [ ] **文件**: `services/strategy-engine/src/domain/model/mod.rs`
- [ ] **变更**: 添加核心类型的 re-export
- [ ] **验证**: `cargo check -p strategy-engine`

**代码变更**:
```rust
// 添加统一导出
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
```

---

### Task 2: 修复 strategy_registry.rs 导入路径

- [ ] **文件**: `services/strategy-engine/src/domain/service/strategy_registry.rs`
- [ ] **变更**: 
  - 修改导入使用统一路径
  - 移除未使用的 `rust_decimal::Decimal`
- [ ] **验证**: `cargo check -p strategy-engine`

**代码变更**:
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

// 删除
// use rust_decimal::Decimal;
```

---

### Task 3: 验证编译通过

- [ ] **命令**: `cargo check -p strategy-engine`
- [ ] **期望**: 无错误

---

### Task 4: 验证测试通过

- [ ] **命令**: `cargo test -p strategy-engine`
- [ ] **期望**: 所有测试通过

---

### Task 5: 代码规范检查

- [ ] 无 `unwrap()`
- [ ] 无 `expect()`
- [ ] 无 `panic!()`
- [ ] 所有 public 结构体有文档注释

---

## 完成标准

- [ ] `cargo check -p strategy-engine` 无错误
- [ ] `cargo test -p strategy-engine` 全部通过
- [ ] 代码符合项目规范

---

## 执行顺序

1. Task 1 (mod.rs)
2. Task 2 (strategy_registry.rs)
3. Task 3 (编译验证)
4. Task 4 (测试验证)
5. Task 5 (规范检查)

---

## 注意事项

- 不要修改 `strategy_runtime.rs` 的类型定义
- 不要修改 `strategy_handle.rs` 的核心逻辑
- 只修改导入路径和导出声明
