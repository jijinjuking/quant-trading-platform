# 开发规范 (Development Standards)

> **强制执行**: 本规范适用于所有代码修改，AI必须严格遵守

---

## 一、核心原则

### AI工程角色定位
- AI是**受约束的工程执行者**，不是架构师
- 首要目标：**不破坏现有系统的前提下实现需求**
- 默认假设：现有代码 = 已上线 / 已被依赖 / 已产生价值

### 工程宪法（最高优先级）

1. **老代码保护原则**
   - 不得随意修改已有代码的核心逻辑
   - 不得为了新需求而"推翻/重写"老代码
   - 修改处理顺序：扩展 → 组合 → 适配 → 最小重构

2. **修改红线（违反即失败）**
   - ❌ 直接修改老函数行为
   - ❌ 跨文件"顺手改一改"
   - ❌ 删除旧逻辑而不解释影响
   - ❌ 改动后不评估历史调用方

---

## 二、代码结构规范

### 文件行数限制
- **单文件不得超过 800 行代码**
- 超过必须：停止编码 → 提出拆分方案 → 等待确认
- 当文件接近600行时，开始考虑拆分

### 模块化要求
- 新功能必须：新模块或新文件
- 禁止直接塞进旧文件
- 不允许隐式跨模块行为修改

### 受保护代码区
以下目录默认为**受保护区域**，不得修改其内部逻辑：
```
src/core/
src/engine/
src/legacy/
```
如需适配，使用：wrapper / adapter / facade

---

## 三、强制工作流

### 写代码前必须执行
1. **STEP 1**: 复述理解的现有系统职责
2. **STEP 2**: 分析新需求是否与老逻辑冲突
3. **STEP 3**: 给出 ≥ 2 种实现方案，并标注风险
4. **STEP 4**: 选择"改动最小"的方案并说明原因
5. **STEP 5**: 仅在此之后，才允许写代码

### 写完代码后强制自检
1. 是否改变了已有函数的语义？
2. 是否引入新的 panic / 崩溃点？
3. 是否影响历史调用方？
4. 是否需要数据迁移或兼容？

> 如任意一条为"是"，必须高亮说明

---

## 四、Rust代码质量规范

### 通用要求
- 禁止 panic 作为正常控制流
- 禁止隐式行为改变
- 禁止"看起来能跑但不可维护"的实现

### Rust特定要求
- 默认 stable Rust
- 不允许 `unsafe`（除非明确授权）
- 不允许 `unwrap / expect`
- 优先使用 `Result / Option`

### 代码组织
```rust
// 1. 模块导入顺序
use std::collections::HashMap;
use anyhow::Result;
use axum::extract::State;
use shared_models::User;

// 2. 结构体定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserService {
    config: UserConfig,
    database: Arc<Database>,
}

// 3. 实现块组织
impl UserService {
    // 构造函数
    pub fn new(config: UserConfig) -> Self { }
    
    // 公共方法
    pub async fn create_user(&self) -> Result<User> { }
    
    // 私有方法
    async fn validate_user(&self) -> Result<()> { }
}
```

---

## 五、服务端口分配（正式版本）

| 服务名称 | 端口 | 备注 |
|---------|------|------|
| trading-engine | 8081 | 交易引擎 |
| market-data | 8082 | 市场数据服务 |
| strategy-engine | 8083 | 策略引擎服务 |
| user-management | 8084 | 用户管理服务 |
| risk-management | 8085 | 风险管理服务 |
| notification | 8086 | 通知服务 |
| ai-service | 8087 | AI服务 |
| analytics | 8088 | 分析服务 |

### 端口配置规范
```rust
// 标准端口配置方式
let port = std::env::var("SERVICE_NAME_PORT")
    .unwrap_or_else(|_| "DEFAULT_PORT".to_string())
    .parse::<u16>()
    .unwrap_or(DEFAULT_PORT);
```

- ❌ 不得随意修改已确定的端口分配
- ❌ 不得在代码中硬编码端口号

---

## 六、测试与交付

### 测试要求
- 新逻辑必须提供最小可运行示例或单元测试
- 测试完成后立即删除测试文件
- 测试文件不得提交到版本库

### 输出顺序（固定）
1. 设计与修改说明
2. 代码 Diff（新增 / 最小修改）
3. 测试方案
4. 风险与影响评估

---

## 七、质量标准

| 指标 | 要求 |
|------|------|
| 文件行数 | ≤ 800行 |
| 函数长度 | ≤ 50行 |
| 圈复杂度 | ≤ 10 |
| 测试覆盖率 | ≥ 80% |
| API响应时间 | < 100ms |

---

## 最终原则

> **工程的第一目标不是"新增功能"，而是"系统不崩"。**
> **AI 必须服从工程纪律，而不是反过来。**
