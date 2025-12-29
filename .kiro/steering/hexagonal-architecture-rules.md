# 🚧 DDD + Hexagonal 架构强制执行规则

> **强制执行**: 本规则适用于所有代码生成，AI必须严格遵守

---

## 一、总体目标

- **目标**: 为每一个微服务生成标准 DDD + 端口-适配器（Hexagonal）骨架
- **当前阶段**: 只允许生成结构、trait、空实现
- ❌ 禁止新增业务逻辑
- ❌ 禁止重构 / 优化 / 合并已有逻辑
- ❌ 禁止"顺手帮忙"

---

## 二、统一架构规则（硬约束）

### 2.1 分层结构（所有 service 必须一致）

```
src/
├── interface/        # 接口层（HTTP / gRPC / WS）
├── application/      # 应用层（用例编排）
├── domain/           # 领域层（核心）
│   └── port/         # ⭐ 端口（trait，只定义接口）
├── infrastructure/   # 基础设施层（adapter 实现）
└── shared/           # 共享内核（错误、基础类型）
```

### 2.2 依赖方向（必须严格遵守）

```
interface → application → domain ← infrastructure
                           ↑
                    domain::port (trait)
```

- **domain** 不允许依赖任何外部层
- **infrastructure** 只能实现 `domain::port` 中的 trait
- **application** 只能依赖 trait，不允许依赖具体实现

---

## 三、Domain 层规则（最重要）

### 3.1 目录结构

```
domain/
├── model/        # 领域模型（entity / value object / aggregate）
├── service/      # 领域服务（可选）
├── port/         # ⭐ 端口定义（trait）
└── event/        # 领域事件（可选）
```

### 3.2 domain::port 规则（强制）

- 只允许 trait
- 入参 / 出参：只能是 Domain 对象或基础类型（String / i64 / bool 等）

**❌ 禁止出现：**
- HTTP 类型
- DB 类型
- SDK 类型
- Redis / Kafka / ORM 类型
- 外部错误类型

**✅ 示例（只允许这种）：**
```rust
pub trait StrategyRepository {
    fn save(&self, strategy: Strategy) -> Result<(), DomainError>;
    fn find_by_id(&self, id: StrategyId) -> Option<Strategy>;
}
```

---

## 四、Infrastructure 层规则（Adapter）

### 4.1 目录结构

```
infrastructure/
├── persistence/     # DB 实现
├── cache/           # Redis 实现
├── messaging/       # Kafka 实现
├── external/        # 外部 SDK / API
```

### 4.2 规则

- **必须** 实现 `domain::port` 中的 trait
- **允许** 做：DTO ↔ Domain 转换、SDK ↔ Domain 转换
- ❌ application / interface 禁止直接调用 infrastructure

---

## 五、Application 层规则（用例编排）

### 5.1 目录结构

```
application/
└── service/
```

### 5.2 规则

- 只依赖 `domain::port` trait
- 只负责编排流程（用例）

**❌ 禁止：**
- SQL
- HTTP client
- Redis client
- SDK 调用

---

## 六、Interface 层规则（接入层）

### 6.1 目录结构

```
interface/
└── http/
    ├── handlers/
    ├── dto/
    └── routes.rs
```

### 6.2 规则

- 只做：请求接收、DTO ↔ Application DTO 转换
- ❌ 不允许出现领域逻辑

---

## 七、执行范围

**所有服务全部执行：**
- Market Data
- Trading Engine
- User Management
- Risk Management
- Notification
- Analytics
- Strategy Engine
- AI Service
- Gateway

**每个服务：**
- 只生成骨架
- 保证能编译
- 允许空实现 / todo!

---

## 八、输出要求

**必须做到：**
- 完整目录结构
- 每个文件存在
- trait 已定义
- adapter 已实现（可空）

**❌ 不写测试**
**❌ 不写业务**

---

## 九、完成后

- **停止**
- 不要继续扩展
- 等待人工检查

> **如果有任何不确定的地方，不要猜，不要发挥，直接停止。**
