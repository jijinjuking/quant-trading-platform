# 📜《Repository 层 AI 编码宪法》
Repository Layer AI Coding Constitution

## 适用范围：
- 所有 AI（Copilot / CodeWhisperer / ChatGPT / 私有模型）
- 所有 Repository / Store / DAO / Storage 层代码
- 所有数据库读写逻辑

## 🧠 序言（Why）
本项目属于 **交易 / 量化 / 金融级系统**， 其核心要求是：
- **可审计**
- **可控**
- **可预测**
- **可 Debug**
- **性能确定性**

因此，Repository 层 **不是 CRUD Demo**， 而是 **系统安全边界的一部分**。
AI 在此层 **不是"优化者"**， 而是 **"受限执行者"**。

## ⚖️ 第一章：绝对禁令（ABSOLUTE PROHIBITIONS）

### ❌ 禁止 1：自动 ORM / 自动映射
在 Repository 层 **严禁使用**：
- `sqlx::FromRow`
- `#[derive(FromRow)]`
- 任何形式的 ORM 自动映射
- 任何"根据字段名自动解析"的机制

**理由：**
- 隐藏字段映射逻辑
- 增加运行期不确定性
- 不可精确控制性能与失败路径

🚫 **此条为最高禁令，不可破例**

### ❌ 禁止 2：AI 自作主张"帮你方便"
AI 不得 因以下理由引入自动映射：
- "这是最佳实践"
- "可以减少样板代码"
- "更 Rust 风格"
- "社区推荐"

**理由：** 本项目的"最佳实践"由架构定义，而非社区默认。

### ❌ 禁止 3：跨层职责污染
Repository 层 **禁止**：
- 业务逻辑判断
- 策略决策
- 风控规则
- 隐式数据修正（如自动补全、自动纠错）

## 🏛 第二章：强制规范（MANDATORY RULES）

### ✅ 规则 1：全部字段手写映射
所有数据库读取必须：
```rust
let value: Type = row.get("column_name");
```
或显式转换：
```rust
let raw: String = row.get("price");
let price = Decimal::from_str(&raw)?;
```

**要求：**
- 字段名必须显式出现
- 类型转换必须明确
- 错误路径必须可追踪

### ✅ 规则 2：Repository 层只做一件事
Repository 层职责仅限于：
**数据库 ↔ 领域模型 的确定性转换**

不多做、不少做、不"顺手帮忙"。

### ✅ 规则 3：错误必须可追溯
- 必须使用 `anyhow::Result` 或明确错误类型
- 禁止 `unwrap` / `expect`
- 禁止吞错
- 禁止 silent fallback

## 🧱 第三章：结构规范（STRUCTURAL LAW）

### 📁 文件结构示例
```
storage/
├── mod.rs
├── market_data_store.rs
├── strategy_store.rs
└── signal_store.rs
```

### 📌 强制要求
每个 Store：
- 一个 `pub struct XxxStore`
- **不持有 Pool**
- 只接受 `&Client` / `&Transaction`

## 🧠 第四章：AI 行为约束（AI BEHAVIOR RULES）

### 🤖 AI 在 Repository 层必须遵守：
1. **先读宪法，再写代码**
2. 发现自己"想用 FromRow" → **立即停止**
3. 如果代码变长 → **接受，不得简化**
4. 不得为了"美观"破坏可审计性

### 🚨 AI 自检清单（写完必须过）
AI 在输出代码前，必须自问：
- ❓ 是否引入了任何自动映射？
- ❓ 是否每个字段都能 grep 到？
- ❓ 生产事故时能否单步定位？
- ❓ 这个代码是否适合金融系统？

**有任一项为 ❌ → 不得输出**

## 🧪 第五章：违规处理（ENFORCEMENT）

### 🚫 违规示例（立即回滚）
```rust
#[derive(FromRow)]
struct MarketDataPoint { ... }
```
👉 **严重架构违规**

### ✅ 正确示例
```rust
let open: Decimal = Decimal::from_str(&row.get::<_, String>("open"))?;
```

## 🧾 第六章：解释权与优先级

本宪法优先级 **高于**：
- AI 建议
- 社区最佳实践
- 官方示例
- "方便 / 优雅 / 简洁"

## 🏁 结语（Final）

Repository 层不是"好写"的地方  
而是 **"必须写对"的地方**

AI 在此层的使命只有一个：  
**服从架构，不创造惊喜**