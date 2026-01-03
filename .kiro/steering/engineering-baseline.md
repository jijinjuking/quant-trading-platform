# 🧷 工程技术底线（必须长期遵守）

> **强制执行**: 本规则为工程红线，任何代码修改都必须遵守，无例外

---

## 一、架构约束（DDD + Hexagonal）

### 1.1 严格遵守 DDD + Hexagonal（端口-适配器）架构

```
interface → application → domain ← infrastructure
                           ↑
                    domain::port (trait)
```

### 1.2 依赖方向规则

- **Application 层** 只能依赖 `domain::port` trait
- **禁止** Application / Domain 引用 infrastructure
- **所有 Adapter** 只能在 `bootstrap.rs` / `main.rs` 注入

---

## 二、禁止"聪明优化"

以下行为**严格禁止**：

| 禁止行为 | 说明 |
|----------|------|
| ❌ 不合并文件 | 即使看起来"更简洁" |
| ❌ 不重构目录 | 保持现有结构 |
| ❌ 不引入新业务概念 | 除非明确要求 |
| ❌ 不调整现有职责边界 | 保持模块职责稳定 |

---

## 三、数据库与基础设施约束

### 3.1 数据库连接

- **禁止** 使用 `sqlx`
- **必须** 使用 `deadpool-postgres` + `tokio-postgres`

### 3.2 ORM 约束

- **禁止** 宏式 ORM / 派生宏（如 `#[derive(FromRow)]`）
- **所有 Model** 必须手写 `struct` + 手动 mapping

```rust
// ✅ 正确做法
impl From<&tokio_postgres::Row> for User {
    fn from(row: &tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            username: row.get("username"),
            // ...
        }
    }
}

// ❌ 禁止
#[derive(FromRow)]
struct User { ... }
```

---

## 四、依赖注入与所有权规则

### 4.1 共享方式

- Service / Adapter 之间通过 `Arc<T>` 共享
- **不使用** 全局状态（`lazy_static!`、`once_cell` 等）
- **不在 service 内** `new` adapter

### 4.2 正确的注入模式

```rust
// ✅ 正确：在 bootstrap/main.rs 中组装
let repo = Arc::new(PostgresUserRepository::new(pool));
let service = UserService::new(repo);

// ❌ 错误：在 service 内部创建 adapter
impl UserService {
    pub fn new() -> Self {
        let repo = PostgresUserRepository::new(...); // 禁止！
        Self { repo }
    }
}
```

---

## 五、不确定时的处理规则

### 5.1 无法实现时

如无法在不违反以上规则的情况下实现：

1. **必须停止**
2. **标注 TODO** 并说明原因
3. **等待人工确认**

```rust
// TODO: 无法在不违反架构规则的情况下实现此功能
// 原因: 需要在 Application 层直接访问数据库
// 等待: 人工确认替代方案
```

### 5.2 禁止猜测

- **禁止** 猜测需求
- **禁止** 补全未明确的逻辑
- **禁止** 脑补实现细节

> 不确定 = 不做 = 标注 TODO = 等待确认

---

## 六、违规后果

违反以上任何规则的代码：

1. 视为**无效提交**
2. 必须**回滚修改**
3. 重新按规则实现

---

## 检查清单（每次提交前自检）

- [ ] Application 层是否只依赖 trait？
- [ ] 是否有 infrastructure 被 domain/application 引用？
- [ ] Adapter 是否只在 bootstrap/main.rs 注入？
- [ ] 是否使用了 sqlx？
- [ ] 是否使用了派生宏做 ORM？
- [ ] 是否有全局状态？
- [ ] 是否在 service 内 new adapter？
- [ ] 不确定的地方是否标注了 TODO？
