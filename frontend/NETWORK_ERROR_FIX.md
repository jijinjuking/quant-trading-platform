# 🔧 Network Error 修复指南

## 问题
点击下单按钮显示：
```
订单提交失败
Network Error
```

## 原因
**后端服务没有启动！**

前端无法连接到后端API，所以显示网络错误。

---

## ✅ 解决方案

### 步骤1: 启动后端服务

打开一个新的终端窗口：

```bash
cd 23
.\start.bat
```

或者如果是 Linux/Mac：
```bash
cd 23
./start.sh
```

### 步骤2: 等待服务启动

你应该看到类似这样的输出：
```
Starting API Gateway on port 8080...
Starting Market Data Service on port 8081...
Starting Trading Engine on port 8082...
Starting User Management on port 8083...

All services started successfully!
```

### 步骤3: 验证服务状态

在浏览器中访问这些地址，确保都返回正常：

1. **API网关**: http://localhost:8080/health
2. **市场数据**: http://localhost:8081/api/v1/health
3. **交易引擎**: http://localhost:8082/api/v1/health
4. **用户管理**: http://localhost:8083/api/v1/health

如果都能访问，说明服务启动成功！

### 步骤4: 刷新前端页面

在浏览器中按 `F5` 刷新页面，然后重新尝试下单。

---

## 🎯 完整测试流程

### 1. 启动后端（新终端）
```bash
cd 23
.\start.bat
```

### 2. 启动前端（另一个终端）
```bash
cd 23/frontend
npm run dev
```

### 3. 访问应用
```
http://localhost:5173
```

### 4. 登录
```
邮箱: trader@quantnexus.com
密码: Trader123456
```

### 5. 测试下单
```
1. 选择交易对: BTCUSDT
2. 输入价格: 50000
3. 输入数量: 0.001
4. 点击"买入"
5. 确认订单
```

### 6. 查看结果
现在应该显示专业的确认对话框，而不是 Network Error！

---

## 🔍 如何检查服务是否运行

### 方法1: 浏览器访问
直接在浏览器访问：
- http://localhost:8080/health

如果显示 `{"status":"ok"}` 或类似内容，说明服务正常。

### 方法2: PowerShell命令
```powershell
# 检查API网关
Invoke-WebRequest -Uri "http://localhost:8080/health"

# 检查市场数据
Invoke-WebRequest -Uri "http://localhost:8081/api/v1/health"

# 检查交易引擎
Invoke-WebRequest -Uri "http://localhost:8082/api/v1/health"

# 检查用户管理
Invoke-WebRequest -Uri "http://localhost:8083/api/v1/health"
```

### 方法3: 查看进程
```powershell
# 查看端口占用
netstat -ano | findstr "8080"
netstat -ano | findstr "8081"
netstat -ano | findstr "8082"
netstat -ano | findstr "8083"
```

---

## 🚨 常见问题

### Q1: start.bat 执行失败
**A**: 检查是否安装了必要的依赖：
```bash
# 检查 Rust
rustc --version

# 检查 Cargo
cargo --version

# 如果没有安装，访问: https://rustup.rs/
```

### Q2: 端口被占用
**A**: 停止占用端口的进程：
```powershell
# 查找占用端口的进程
netstat -ano | findstr "8080"

# 结束进程（替换 PID）
taskkill /PID <进程ID> /F
```

### Q3: 服务启动后立即退出
**A**: 查看日志文件：
```bash
cd 23/logs
type gateway.log
type market-data.log
type trading-engine.log
type user-management.log
```

### Q4: 数据库连接失败
**A**: 检查数据库配置：
```bash
# 查看 .env 文件
type .env

# 确保数据库服务运行
# PostgreSQL / MySQL / SQLite
```

---

## 📝 服务启动检查清单

启动后端服务后，确认以下内容：

- [ ] API网关 (8080) 运行正常
- [ ] 市场数据服务 (8081) 运行正常
- [ ] 交易引擎 (8082) 运行正常
- [ ] 用户管理 (8083) 运行正常
- [ ] 数据库连接正常
- [ ] 日志没有错误信息

---

## 🎉 修复后的效果

### 修复前 ❌
```
订单提交失败
Network Error
```

### 修复后 ✅
```
[专业的确认对话框]
确认买入订单

交易对: BTCUSDT
类型: 限价单
价格: 50000 USDT
数量: 0.001
预估金额: 50.00 USDT

[取消]  [确定]
```

---

## 💡 提示

### 开发时的最佳实践

1. **先启动后端，再启动前端**
   ```bash
   # 终端1
   cd 23
   .\start.bat
   
   # 终端2
   cd 23/frontend
   npm run dev
   ```

2. **保持后端服务运行**
   - 不要关闭后端服务的终端窗口
   - 如果需要重启，使用 `.\stop.bat` 然后 `.\start.bat`

3. **查看日志**
   - 后端日志在 `23/logs/` 目录
   - 前端日志在浏览器控制台 (F12)

4. **定期检查服务状态**
   - 访问 health 端点
   - 查看日志文件
   - 监控资源使用

---

## 🔧 快速修复命令

如果遇到问题，按顺序执行：

```bash
# 1. 停止所有服务
cd 23
.\stop.bat

# 2. 清理端口（如果需要）
# 手动结束占用端口的进程

# 3. 重新启动
.\start.bat

# 4. 验证服务
# 浏览器访问 http://localhost:8080/health

# 5. 刷新前端
# 浏览器按 F5
```

---

## 📞 需要帮助？

如果按照以上步骤仍然无法解决，请提供：

1. **错误截图**
2. **后端日志** (`23/logs/` 目录下的文件)
3. **浏览器控制台错误** (F12 → Console)
4. **服务状态** (哪些端口能访问，哪些不能)

---

**记住：Network Error = 后端服务没启动！** 🚀

先启动后端，再测试前端！
