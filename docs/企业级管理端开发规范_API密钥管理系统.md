# 管理端开发规范 - API密钥管理系统

## 🎯 项目概述

基于现有量化交易平台架构，开发统一的管理端系统，重点实现API密钥的集中管理和配置。

## 🏗️ 系统架构

### 管理端定位
```
┌─────────────────────────────────────────────────────────────┐
│                    量化交易平台生态系统                        │
├─────────────────┬─────────────────┬─────────────────────────┤
│   管理端        │    用户端       │      手机端             │
│  (Vue 3 + TS)   │ (React + TS)    │  (React Native)         │
│  系统管理       │  专业交易       │   移动交易              │
├─────────────────┼─────────────────┼─────────────────────────┤
│ • API密钥管理   │ • 实时交易      │ • 基础交易              │
│ • 用户管理      │ • 策略管理      │ • 行情查看              │
│ • 系统配置      │ • 数据分析      │ • 账户管理              │
│ • 监控运维      │ • 风险控制      │ • 通知推送              │
│ • 权限控制      │ • AI交易        │ • 简化操作              │
└─────────────────┴─────────────────┴─────────────────────────┘
```

## 🔑 API密钥管理系统设计

### 1. 核心功能需求

#### 1.1 AI模型API密钥管理
```typescript
interface AIModelConfig {
  id: string;
  name: string;           // DeepSeek V2, GPT-4 Turbo, Claude 3.5, Gemini Pro
  provider: string;       // deepseek, openai, anthropic, google
  apiKey: string;         // 加密存储
  baseUrl: string;        // API基础URL
  modelName: string;      // 具体模型名称
  status: 'active' | 'inactive' | 'error';
  rateLimit: number;      // 请求限制
  costPerToken: number;   // 成本计算
  lastUsed: Date;
  createdAt: Date;
  updatedAt: Date;
}
```

#### 1.2 交易所API密钥管理
```typescript
interface ExchangeAPIConfig {
  id: string;
  exchange: string;       // binance, okx, huobi
  name: string;           // 显示名称
  apiKey: string;         // 加密存储
  secretKey: string;      // 加密存储
  passphrase?: string;    // OKX需要
  sandbox: boolean;       // 是否沙盒环境
  permissions: string[];  // spot, futures, margin
  ipWhitelist: string[];  // IP白名单
  status: 'active' | 'inactive' | 'error';
  lastUsed: Date;
  createdAt: Date;
  updatedAt: Date;
}
```

#### 1.3 第三方服务API密钥管理
```typescript
interface ThirdPartyAPIConfig {
  id: string;
  service: string;        // news_api, alpha_vantage, coingecko
  name: string;
  apiKey: string;         // 加密存储
  baseUrl: string;
  rateLimit: number;
  status: 'active' | 'inactive' | 'error';
  lastUsed: Date;
  createdAt: Date;
  updatedAt: Date;
}
```

### 2. 数据库设计

#### 2.1 API密钥表结构
```sql
-- AI模型API配置表
CREATE TABLE ai_model_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    provider VARCHAR(50) NOT NULL,
    api_key_encrypted TEXT NOT NULL,
    base_url VARCHAR(255) NOT NULL,
    model_name VARCHAR(100) NOT NULL,
    status VARCHAR(20) DEFAULT 'inactive',
    rate_limit INTEGER DEFAULT 1000,
    cost_per_token DECIMAL(10,8) DEFAULT 0.0,
    last_used TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id),
    UNIQUE(provider, model_name)
);

-- 交易所API配置表
CREATE TABLE exchange_api_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    exchange VARCHAR(50) NOT NULL,
    name VARCHAR(100) NOT NULL,
    api_key_encrypted TEXT NOT NULL,
    secret_key_encrypted TEXT NOT NULL,
    passphrase_encrypted TEXT,
    sandbox BOOLEAN DEFAULT true,
    permissions TEXT[] DEFAULT '{}',
    ip_whitelist TEXT[] DEFAULT '{}',
    status VARCHAR(20) DEFAULT 'inactive',
    last_used TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id),
    UNIQUE(exchange, api_key_encrypted)
);

-- 第三方服务API配置表
CREATE TABLE third_party_api_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    service VARCHAR(50) NOT NULL,
    name VARCHAR(100) NOT NULL,
    api_key_encrypted TEXT NOT NULL,
    base_url VARCHAR(255) NOT NULL,
    rate_limit INTEGER DEFAULT 1000,
    status VARCHAR(20) DEFAULT 'inactive',
    last_used TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id),
    UNIQUE(service, name)
);

-- API密钥使用日志表
CREATE TABLE api_key_usage_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    config_type VARCHAR(20) NOT NULL, -- 'ai_model', 'exchange', 'third_party'
    config_id UUID NOT NULL,
    service_name VARCHAR(100) NOT NULL,
    request_count INTEGER DEFAULT 1,
    tokens_used INTEGER DEFAULT 0,
    cost_incurred DECIMAL(10,8) DEFAULT 0.0,
    success_count INTEGER DEFAULT 0,
    error_count INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### 3. 后端API设计

#### 3.1 AI模型API管理
```rust
// AI模型配置管理API
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAIModelConfigRequest {
    pub name: String,
    pub provider: String,
    pub api_key: String,
    pub base_url: String,
    pub model_name: String,
    pub rate_limit: Option<i32>,
    pub cost_per_token: Option<f64>,
}

// API路由
POST   /api/admin/ai-models              // 创建AI模型配置
GET    /api/admin/ai-models              // 获取AI模型配置列表
GET    /api/admin/ai-models/{id}         // 获取单个AI模型配置
PUT    /api/admin/ai-models/{id}         // 更新AI模型配置
DELETE /api/admin/ai-models/{id}         // 删除AI模型配置
POST   /api/admin/ai-models/{id}/test    // 测试API连接
GET    /api/admin/ai-models/{id}/usage   // 获取使用统计
```

#### 3.2 交易所API管理
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateExchangeAPIConfigRequest {
    pub exchange: String,
    pub name: String,
    pub api_key: String,
    pub secret_key: String,
    pub passphrase: Option<String>,
    pub sandbox: bool,
    pub permissions: Vec<String>,
    pub ip_whitelist: Vec<String>,
}

// API路由
POST   /api/admin/exchanges              // 创建交易所API配置
GET    /api/admin/exchanges              // 获取交易所API配置列表
GET    /api/admin/exchanges/{id}         // 获取单个交易所API配置
PUT    /api/admin/exchanges/{id}         // 更新交易所API配置
DELETE /api/admin/exchanges/{id}         // 删除交易所API配置
POST   /api/admin/exchanges/{id}/test    // 测试API连接
GET    /api/admin/exchanges/{id}/balance // 获取账户余额
```

#### 3.3 系统配置分发API
```rust
// 配置分发到各个服务
POST   /api/admin/config/distribute      // 分发配置到所有服务
GET    /api/admin/config/status          // 获取配置分发状态
POST   /api/admin/config/reload          // 重新加载服务配置
GET    /api/admin/services/health        // 获取所有服务健康状态
```

### 4. 前端管理界面设计

#### 4.1 Vue 3 + TypeScript 技术栈
```typescript
// 项目结构
admin-frontend/
├── src/
│   ├── components/
│   │   ├── APIKeyManagement/
│   │   │   ├── AIModelConfig.vue      // AI模型配置
│   │   │   ├── ExchangeConfig.vue     // 交易所配置
│   │   │   ├── ThirdPartyConfig.vue   // 第三方服务配置
│   │   │   └── ConfigTest.vue         // 配置测试
│   │   ├── SystemMonitor/
│   │   │   ├── ServiceHealth.vue      // 服务健康监控
│   │   │   ├── UsageStatistics.vue    // 使用统计
│   │   │   └── ConfigStatus.vue       // 配置状态
│   │   └── UserManagement/
│   │       ├── UserList.vue           // 用户列表
│   │       ├── RoleManagement.vue     // 角色管理
│   │       └── PermissionControl.vue  // 权限控制
│   ├── stores/
│   │   ├── apiConfig.ts               // API配置状态管理
│   │   ├── systemMonitor.ts           // 系统监控状态
│   │   └── userManagement.ts          // 用户管理状态
│   ├── services/
│   │   ├── apiConfigService.ts        // API配置服务
│   │   ├── systemService.ts           // 系统服务
│   │   └── userService.ts             // 用户服务
│   └── views/
│       ├── Dashboard.vue              // 仪表板
│       ├── APIManagement.vue          // API管理
│       ├── SystemSettings.vue         // 系统设置
│       └── UserManagement.vue         // 用户管理
```

#### 4.2 核心组件设计

##### AI模型配置组件
```vue
<template>
  <div class="ai-model-config">
    <div class="config-header">
      <h2>AI模型API配置</h2>
      <el-button type="primary" @click="showAddDialog = true">
        添加AI模型
      </el-button>
    </div>
    
    <el-table :data="aiModels" style="width: 100%">
      <el-table-column prop="name" label="模型名称" />
      <el-table-column prop="provider" label="提供商" />
      <el-table-column prop="status" label="状态">
        <template #default="{ row }">
          <el-tag :type="getStatusType(row.status)">
            {{ getStatusText(row.status) }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column prop="rateLimit" label="请求限制" />
      <el-table-column prop="lastUsed" label="最后使用" />
      <el-table-column label="操作" width="200">
        <template #default="{ row }">
          <el-button size="small" @click="testConnection(row.id)">
            测试连接
          </el-button>
          <el-button size="small" type="primary" @click="editConfig(row)">
            编辑
          </el-button>
          <el-button size="small" type="danger" @click="deleteConfig(row.id)">
            删除
          </el-button>
        </template>
      </el-table-column>
    </el-table>
    
    <!-- 添加/编辑对话框 -->
    <el-dialog v-model="showAddDialog" title="AI模型配置">
      <el-form :model="currentConfig" label-width="120px">
        <el-form-item label="模型名称">
          <el-input v-model="currentConfig.name" />
        </el-form-item>
        <el-form-item label="提供商">
          <el-select v-model="currentConfig.provider">
            <el-option label="DeepSeek" value="deepseek" />
            <el-option label="OpenAI" value="openai" />
            <el-option label="Anthropic" value="anthropic" />
            <el-option label="Google" value="google" />
          </el-select>
        </el-form-item>
        <el-form-item label="API密钥">
          <el-input v-model="currentConfig.apiKey" type="password" show-password />
        </el-form-item>
        <el-form-item label="基础URL">
          <el-input v-model="currentConfig.baseUrl" />
        </el-form-item>
        <el-form-item label="模型名称">
          <el-input v-model="currentConfig.modelName" />
        </el-form-item>
        <el-form-item label="请求限制">
          <el-input-number v-model="currentConfig.rateLimit" :min="1" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showAddDialog = false">取消</el-button>
        <el-button type="primary" @click="saveConfig">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { apiConfigService } from '@/services/apiConfigService'

const aiModels = ref([])
const showAddDialog = ref(false)
const currentConfig = ref({
  name: '',
  provider: '',
  apiKey: '',
  baseUrl: '',
  modelName: '',
  rateLimit: 1000
})

const loadAIModels = async () => {
  try {
    const response = await apiConfigService.getAIModels()
    aiModels.value = response.data
  } catch (error) {
    ElMessage.error('加载AI模型配置失败')
  }
}

const testConnection = async (id: string) => {
  try {
    await apiConfigService.testAIModel(id)
    ElMessage.success('连接测试成功')
  } catch (error) {
    ElMessage.error('连接测试失败')
  }
}

const saveConfig = async () => {
  try {
    if (currentConfig.value.id) {
      await apiConfigService.updateAIModel(currentConfig.value.id, currentConfig.value)
    } else {
      await apiConfigService.createAIModel(currentConfig.value)
    }
    ElMessage.success('保存成功')
    showAddDialog.value = false
    loadAIModels()
  } catch (error) {
    ElMessage.error('保存失败')
  }
}

onMounted(() => {
  loadAIModels()
})
</script>
```

### 5. 安全设计

#### 5.1 API密钥加密存储
```rust
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};

pub struct APIKeyEncryption {
    cipher: Aes256Gcm,
}

impl APIKeyEncryption {
    pub fn new(key: &[u8; 32]) -> Self {
        let key = Key::from_slice(key);
        let cipher = Aes256Gcm::new(key);
        Self { cipher }
    }
    
    pub fn encrypt(&self, plaintext: &str) -> Result<String, Box<dyn std::error::Error>> {
        let nonce = Nonce::from_slice(b"unique nonce"); // 实际使用中应该是随机生成
        let ciphertext = self.cipher.encrypt(nonce, plaintext.as_bytes())?;
        Ok(base64::encode(ciphertext))
    }
    
    pub fn decrypt(&self, ciphertext: &str) -> Result<String, Box<dyn std::error::Error>> {
        let nonce = Nonce::from_slice(b"unique nonce");
        let ciphertext = base64::decode(ciphertext)?;
        let plaintext = self.cipher.decrypt(nonce, ciphertext.as_ref())?;
        Ok(String::from_utf8(plaintext)?)
    }
}
```

#### 5.2 权限控制
```rust
#[derive(Debug, Serialize, Deserialize)]
pub enum AdminPermission {
    APIKeyManage,      // API密钥管理
    SystemConfig,      // 系统配置
    UserManage,        // 用户管理
    SystemMonitor,     // 系统监控
    AuditLog,          // 审计日志
}

pub fn check_admin_permission(user_id: &str, permission: AdminPermission) -> bool {
    // 检查用户是否有指定的管理权限
    // 实现权限验证逻辑
    true
}
```

### 6. 配置分发机制

#### 6.1 配置推送到各服务
```rust
pub struct ConfigDistributor {
    services: Vec<ServiceEndpoint>,
}

impl ConfigDistributor {
    pub async fn distribute_ai_config(&self, config: &AIModelConfig) -> Result<(), Error> {
        // 推送AI配置到AI服务
        let ai_service_url = "http://localhost:8088/api/admin/config/update";
        let payload = serde_json::json!({
            "ai_models": [config]
        });
        
        // 发送配置更新请求
        let client = reqwest::Client::new();
        let response = client
            .post(ai_service_url)
            .json(&payload)
            .send()
            .await?;
            
        if response.status().is_success() {
            info!("AI配置推送成功");
        } else {
            error!("AI配置推送失败: {}", response.status());
        }
        
        Ok(())
    }
    
    pub async fn distribute_exchange_config(&self, config: &ExchangeAPIConfig) -> Result<(), Error> {
        // 推送交易所配置到市场数据服务和交易引擎
        let services = vec![
            "http://localhost:8081/api/admin/config/update", // 市场数据服务
            "http://localhost:8082/api/admin/config/update", // 交易引擎
        ];
        
        for service_url in services {
            // 发送配置更新请求
            // 实现配置推送逻辑
        }
        
        Ok(())
    }
}
```

### 7. 部署和运维

#### 7.1 Docker部署
```dockerfile
# 管理端后端
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/admin-backend /usr/local/bin/admin-backend
EXPOSE 8090
CMD ["admin-backend"]
```

```dockerfile
# 管理端前端
FROM node:18-alpine as builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/nginx.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
```

#### 7.2 服务端口分配
```yaml
# 管理端服务端口
admin-backend: 8090    # 管理端后端API
admin-frontend: 3000   # 管理端前端界面
```

### 8. 开发计划

#### Phase 1: 基础架构 (2周)
- [ ] 数据库设计和迁移
- [ ] 后端API框架搭建
- [ ] 前端项目初始化
- [ ] 基础认证和权限系统

#### Phase 2: API密钥管理 (3周)
- [ ] AI模型API配置管理
- [ ] 交易所API配置管理
- [ ] 第三方服务API配置管理
- [ ] 配置测试和验证功能

#### Phase 3: 系统集成 (2周)
- [ ] 配置分发机制
- [ ] 服务健康监控
- [ ] 使用统计和日志
- [ ] 安全加固和审计

#### Phase 4: 用户界面 (2周)
- [ ] 管理界面开发
- [ ] 用户体验优化
- [ ] 文档和培训材料
- [ ] 测试和部署

### 9. 总结

通过统一的管理端系统，可以实现：

1. **集中管理**: 所有API密钥在一个地方管理
2. **安全存储**: 加密存储敏感信息
3. **权限控制**: 细粒度的权限管理
4. **配置分发**: 自动推送配置到各个服务
5. **监控审计**: 完整的使用日志和审计跟踪
6. **用户友好**: 直观的Web界面操作

这样就解决了你提到的API密钥管理问题，所有的AI模型、交易所、第三方服务的API密钥都可以在管理端统一配置和管理。