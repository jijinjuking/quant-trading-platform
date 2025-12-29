# K8s部署配置 (将来使用)

## 迁移路径

### 阶段1: Docker Compose (当前)
```bash
docker-compose -f docker-compose.unified.yml up -d
```

### 阶段2: K8s迁移 (将来)
```bash
# 1. 创建命名空间
kubectl create namespace quant-trading

# 2. 部署存储层
kubectl apply -f k8s/storage/

# 3. 部署服务层  
kubectl apply -f k8s/services/

# 4. 部署前端
kubectl apply -f k8s/frontend/
```

## 设计原则

### 1. 服务发现兼容
- Docker: `redis:6379`
- K8s: `redis.quant-trading.svc.cluster.local:6379`

### 2. 配置管理
- Docker: `.env` 文件
- K8s: `ConfigMap` + `Secret`

### 3. 存储
- Docker: Named volumes
- K8s: `PersistentVolume`

### 4. 网络
- Docker: Bridge network
- K8s: Service mesh

## 迁移检查清单

- [ ] 所有服务Docker化
- [ ] 配置外部化 (环境变量)
- [ ] 健康检查实现
- [ ] 服务间通信标准化
- [ ] 监控和日志集成
- [ ] 数据持久化方案
- [ ] 安全配置 (RBAC准备)

## 预估迁移时间
- 准备阶段: 1-2天 (创建K8s配置)
- 迁移阶段: 半天 (如果Docker版本稳定)
- 测试阶段: 1天 (验证功能完整性)