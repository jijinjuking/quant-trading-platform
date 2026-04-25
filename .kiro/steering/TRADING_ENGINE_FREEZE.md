# 《职责冻结文档》—— Trading Engine（工程级量化平台）

> **文档状态**: 🔒 冻结
> **生效日期**: 2026-01-07
> **适用范围**: Trading Engine / Strategy Engine

---

## 文档目的

1. **冻结** 当前阶段各模块职责边界，防止后续 Strategy / CopyTrading / 分佣设计阶段发生职责污染
2. 基于现有代码真实状态，而不是理想设计，给出工程级判断
3. 作为后续多人协作、AI 协作、策略扩展的**唯一裁决基线**

---

## 一、系统总体定位（先冻结认知）

### 1. 系统不是「策略主动引擎」

**关键前提（必须冻结）**：

- `services/trading-engine` **不是**行情消费方
- **不**直接连接 WebSocket / MarketData
- **不**负责行情订阅、数据拉取、数据缓存
- **永远是被动调用方**（被 Trading Brain / 上游调度器驱动）

**Trading Engine 的本质是**：
> "交易执行 + 风控裁决 + 状态编排的被动执行核心"

它只回答三类问题：
1. 这个指令能不能执行？（Risk）
2. 如果能，如何以正确顺序执行？（Lifecycle / Execution）
3. 执行后状态如何变化？（State）

---

## 二、当前已完成模块职责冻结

### 1. main.rs / bootstrap

**职责（冻结）**：

仅负责：
- 环境初始化
- 服务装配（wiring）
- 生命周期托管（spawn / join）

❌ 不包含任何业务逻辑
❌ 不做策略调度、不做风控判断

**当前状态**：
- 已从单文件重构为目录
- 生命周期收口清晰
- 属于合格的工程级 bootstrap

✅ **状态：可冻结，不再修改职责**

---

### 2. RiskStateCoordinator

**职责（冻结）**：

风控"状态协调器"，而非风控规则本身

统一管理：
- 风控状态生命周期
- WS 重连后的状态修复
- 风控状态的可见性与一致性

**明确不做的事**：
- ❌ 不定义具体风控策略
- ❌ 不关心订单来自哪个策略
- ❌ 不直接操作订单

**当前状态**：
- 已完成
- 已接入 main.rs
- WS 重连问题已修复
- 生命周期已收口

✅ **状态：冻结，可作为后续所有执行前置依赖**

---

### 3. OrderLifecycleService

**职责（冻结）**：

订单级生命周期编排

将「一个交易指令」拆解为：
1. 校验
2. 风控
3. 下单
4. 回执处理
5. 状态回写

**明确不做的事**：
- ❌ 不决定"是否交易"（策略职责）
- ❌ 不关心策略逻辑
- ❌ 不维护策略状态

**当前状态**：
- 已正确 spawn
- 生命周期清晰
- 与 RiskStateCoordinator 解耦

⚠️ **待完善点（但职责不变）**：
- 对 Strategy / CopyTrading 的调用接口尚未冻结

---

### 4. BinanceFillStream

**职责（冻结）**：

仅负责成交回执流的抽象

提供：
- on_reconnect 回调
- 成交事件推送

**明确不做的事**：
- ❌ 不参与策略计算
- ❌ 不做订单决策
- ❌ 不维护策略状态

**当前状态**：
- on_reconnect 已支持
- 行为符合工程预期

✅ **状态：冻结**

---

## 三、Strategy Engine 的正确工程定位（核心）

### 1. 当前最大误区（已识别）

`activate / deactivate / reset` 属于**假生命周期**

**问题本质**：
- 只是 bool 切换
- 没有资源管理
- 没有隔离
- 无法支撑真实生产策略规模

> 该结论是 100% 正确的工程判断。

### 2. 冻结正确的 Strategy Engine 定位

**Strategy Engine 不是**：
- 行情消费者
- 调度中心
- 主动执行者

**Strategy Engine 是**：
> "策略执行单元的托管容器（Execution Container）"

它只做三件事：
1. 托管策略实例
2. 接收上游调用指令
3. 将策略决策结果转化为标准交易意图

---

## 四、真实生命周期模型（冻结标准）

### 策略 ≠ 一个 struct

**工程级冻结定义**：

> 一个策略实例 = 一个独立运行单元

必须具备：
- 独立 Task
- 独立状态
- 独立通信通道
- 可被销毁、重启、隔离

### 正确的生命周期语义

| 操作 | 工程级含义 |
|------|------------|
| start | spawn Task + 建立通道 |
| stop | 发送 Shutdown + 等待 Join |
| pause | 停止接收外部驱动 |
| resume | 恢复接收 |
| restart | stop → start |
| unregister | stop + Registry 移除 |

> **activate / deactivate 在工程层面禁止使用。**

---

## 五、Strategy Registry 的冻结职责

### Registry 只做一件事

> 管理策略"句柄"，而不是策略本身

**Registry 必须是**：
- 轻量
- 无业务逻辑
- 不可阻塞

**它维护的是**：
- 实例 ID
- 通信端点
- 状态观测
- Task 句柄

**不维护**：
- 策略状态
- 策略逻辑
- 策略内部资源

---

## 六、与 Trading Brain 的边界冻结

### 冻结调用方向

**唯一合法方向**：
```
Trading Brain → Trading Engine → Strategy Engine
```

**反方向一律禁止。**

### Strategy Engine 接收的不是行情

而是：
- `execute(signal)`
- `on_position_update(...)`
- `on_order_feedback(...)`

> **行情计算永远属于 Trading Brain。**

---

## 七、当前缺口评估（基于现有代码）

### 已完成（可冻结）

- ✅ 启动与生命周期收口
- ✅ 风控状态协调
- ✅ 执行服务骨架
- ✅ 成交回执抽象

### 明确缺失（下一阶段必须做）

- ❌ Strategy Registry（真实生命周期）
- ❌ Strategy Task 隔离模型
- ❌ Strategy → Execution 的标准接口
- ❌ 策略失败 / panic 的处理策略

---

## 八、实施路线图（冻结版）

### Phase 1（必须先做）

1. Strategy Registry（句柄模型）
2. Strategy Task 生命周期
3. 禁止 bool 生命周期

### Phase 2

1. CopyTrading（复用 Execution 层）
2. 分佣系统（监听成交，不介入执行）

### Phase 3

1. 策略热加载
2. 策略版本化
3. 策略沙箱

---

## 九、最终冻结声明

**从本文件起**：

1. Trading Engine **不再承担**策略计算职责
2. Strategy Engine **不再允许**假生命周期
3. Registry **只管句柄，不管逻辑**

**任何违反本冻结文档的实现**：
> 视为架构违规，必须回滚或重构

---

**—— 文档结束 ——**
