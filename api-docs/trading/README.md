# 交易功能 API

## 1. 创建订单

### 请求
```http
POST /api/v1/orders
Authorization: Bearer YOUR_JWT_TOKEN
Content-Type: application/json
```

### 请求体
```json
{
  "symbol": "BTCUSDT",
  "side": "BUY",
  "type": "LIMIT",
  "quantity": 0.001,
  "price": 45000.00,
  "time_in_force": "GTC"
}
```

### 参数说明
- `symbol`: 交易对 (如: BTCUSDT, ETHUSDT)
- `side`: 买卖方向 (BUY/SELL)
- `type`: 订单类型 (MARKET/LIMIT/STOP_LOSS/TAKE_PROFIT)
- `quantity`: 数量
- `price`: 价格 (市价单可选)
- `time_in_force`: 有效期 (GTC/IOC/FOK)

### 成功响应 (201)
```json
{
  "success": true,
  "data": {
    "order_id": "order_123456",
    "symbol": "BTCUSDT",
    "side": "BUY",
    "type": "LIMIT",
    "quantity": 0.001,
    "price": 45000.00,
    "status": "NEW",
    "created_at": "2024-12-18T10:30:00Z"
  },
  "error": null,
  "timestamp": "2024-12-18T10:30:00Z"
}
```

### 前端调用示例
```javascript
// 下单函数
async function createOrder(orderData) {
  const response = await fetch('http://localhost:8091/api/v1/orders', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${localStorage.getItem('token')}`
    },
    body: JSON.stringify(orderData)
  });
  
  const result = await response.json();
  
  if (result.success) {
    return result.data;
  } else {
    throw new Error(result.error.message);
  }
}

// 使用示例
const order = await createOrder({
  symbol: 'BTCUSDT',
  side: 'BUY',
  type: 'LIMIT',
  quantity: 0.001,
  price: 45000.00,
  time_in_force: 'GTC'
});
```

---

## 2. 查询订单列表

### 请求
```http
GET /api/v1/orders?symbol=BTCUSDT&status=OPEN&page=1&limit=20
Authorization: Bearer YOUR_JWT_TOKEN
```

### 查询参数
- `symbol`: 交易对 (可选)
- `status`: 订单状态 (可选: NEW/PARTIALLY_FILLED/FILLED/CANCELED/EXPIRED)
- `page`: 页码 (默认: 1)
- `limit`: 每页数量 (默认: 20, 最大: 100)

### 成功响应 (200)
```json
{
  "success": true,
  "data": {
    "items": [
      {
        "order_id": "order_123456",
        "symbol": "BTCUSDT",
        "side": "BUY",
        "type": "LIMIT",
        "quantity": 0.001,
        "price": 45000.00,
        "filled_quantity": 0.0005,
        "status": "PARTIALLY_FILLED",
        "created_at": "2024-12-18T10:30:00Z",
        "updated_at": "2024-12-18T10:35:00Z"
      }
    ],
    "pagination": {
      "current_page": 1,
      "page_size": 20,
      "total_items": 150,
      "total_pages": 8,
      "has_next": true,
      "has_prev": false
    }
  },
  "error": null,
  "timestamp": "2024-12-18T10:30:00Z"
}
```

---

## 3. 查询单个订单

### 请求
```http
GET /api/v1/orders/{order_id}
Authorization: Bearer YOUR_JWT_TOKEN
```

### 成功响应 (200)
```json
{
  "success": true,
  "data": {
    "order_id": "order_123456",
    "symbol": "BTCUSDT",
    "side": "BUY",
    "type": "LIMIT",
    "quantity": 0.001,
    "price": 45000.00,
    "filled_quantity": 0.001,
    "average_price": 44950.00,
    "status": "FILLED",
    "created_at": "2024-12-18T10:30:00Z",
    "updated_at": "2024-12-18T10:35:00Z",
    "trades": [
      {
        "trade_id": "trade_789",
        "quantity": 0.001,
        "price": 44950.00,
        "fee": 0.045,
        "fee_asset": "USDT",
        "timestamp": "2024-12-18T10:35:00Z"
      }
    ]
  },
  "error": null,
  "timestamp": "2024-12-18T10:30:00Z"
}
```

---

## 4. 取消订单

### 请求
```http
DELETE /api/v1/orders/{order_id}
Authorization: Bearer YOUR_JWT_TOKEN
```

### 成功响应 (200)
```json
{
  "success": true,
  "data": {
    "order_id": "order_123456",
    "status": "CANCELED",
    "canceled_at": "2024-12-18T10:40:00Z"
  },
  "error": null,
  "timestamp": "2024-12-18T10:40:00Z"
}
```

---

## 5. 查询持仓

### 请求
```http
GET /api/v1/positions
Authorization: Bearer YOUR_JWT_TOKEN
```

### 成功响应 (200)
```json
{
  "success": true,
  "data": [
    {
      "symbol": "BTCUSDT",
      "side": "LONG",
      "size": 0.001,
      "entry_price": 44950.00,
      "mark_price": 45200.00,
      "unrealized_pnl": 0.25,
      "realized_pnl": 0.0,
      "margin": 4.495,
      "percentage": 5.56,
      "created_at": "2024-12-18T10:35:00Z"
    }
  ],
  "error": null,
  "timestamp": "2024-12-18T10:30:00Z"
}
```

---

## 6. 查询账户余额

### 请求
```http
GET /api/v1/balances
Authorization: Bearer YOUR_JWT_TOKEN
```

### 成功响应 (200)
```json
{
  "success": true,
  "data": {
    "total_balance": 10000.00,
    "available_balance": 8500.00,
    "frozen_balance": 1500.00,
    "assets": [
      {
        "asset": "USDT",
        "free": 8500.00,
        "locked": 1500.00,
        "total": 10000.00
      },
      {
        "asset": "BTC",
        "free": 0.001,
        "locked": 0.0,
        "total": 0.001
      }
    ]
  },
  "error": null,
  "timestamp": "2024-12-18T10:30:00Z"
}
```

---

## 7. 查询交易历史

### 请求
```http
GET /api/v1/trades?symbol=BTCUSDT&start_time=1703721600&end_time=1703808000&page=1&limit=50
Authorization: Bearer YOUR_JWT_TOKEN
```

### 查询参数
- `symbol`: 交易对 (可选)
- `start_time`: 开始时间戳 (可选)
- `end_time`: 结束时间戳 (可选)
- `page`: 页码 (默认: 1)
- `limit`: 每页数量 (默认: 50, 最大: 100)

### 成功响应 (200)
```json
{
  "success": true,
  "data": {
    "items": [
      {
        "trade_id": "trade_789",
        "order_id": "order_123456",
        "symbol": "BTCUSDT",
        "side": "BUY",
        "quantity": 0.001,
        "price": 44950.00,
        "fee": 0.045,
        "fee_asset": "USDT",
        "timestamp": "2024-12-18T10:35:00Z"
      }
    ],
    "pagination": {
      "current_page": 1,
      "page_size": 50,
      "total_items": 25,
      "total_pages": 1,
      "has_next": false,
      "has_prev": false
    }
  },
  "error": null,
  "timestamp": "2024-12-18T10:30:00Z"
}
```

---

## React 交易组件示例

```javascript
import React, { useState, useEffect } from 'react';
import axios from 'axios';

const TradingPanel = () => {
  const [symbol, setSymbol] = useState('BTCUSDT');
  const [side, setSide] = useState('BUY');
  const [quantity, setQuantity] = useState('');
  const [price, setPrice] = useState('');
  const [orders, setOrders] = useState([]);
  const [balance, setBalance] = useState(null);

  // 获取余额
  useEffect(() => {
    fetchBalance();
    fetchOrders();
  }, []);

  const fetchBalance = async () => {
    try {
      const response = await axios.get('http://localhost:8091/api/v1/balances');
      setBalance(response.data.data);
    } catch (error) {
      console.error('获取余额失败:', error);
    }
  };

  const fetchOrders = async () => {
    try {
      const response = await axios.get('http://localhost:8091/api/v1/orders');
      setOrders(response.data.data.items);
    } catch (error) {
      console.error('获取订单失败:', error);
    }
  };

  const handleSubmitOrder = async (e) => {
    e.preventDefault();
    
    try {
      const orderData = {
        symbol,
        side,
        type: 'LIMIT',
        quantity: parseFloat(quantity),
        price: parseFloat(price),
        time_in_force: 'GTC'
      };

      const response = await axios.post('http://localhost:8091/api/v1/orders', orderData);
      
      if (response.data.success) {
        alert('订单创建成功!');
        setQuantity('');
        setPrice('');
        fetchOrders(); // 刷新订单列表
        fetchBalance(); // 刷新余额
      }
    } catch (error) {
      alert('订单创建失败: ' + error.response?.data?.error?.message);
    }
  };

  const cancelOrder = async (orderId) => {
    try {
      await axios.delete(`http://localhost:8091/api/v1/orders/${orderId}`);
      alert('订单取消成功!');
      fetchOrders();
    } catch (error) {
      alert('订单取消失败: ' + error.response?.data?.error?.message);
    }
  };

  return (
    <div className="trading-panel">
      <h2>交易面板</h2>
      
      {/* 余额显示 */}
      {balance && (
        <div className="balance-info">
          <p>可用余额: {balance.available_balance} USDT</p>
        </div>
      )}

      {/* 下单表单 */}
      <form onSubmit={handleSubmitOrder}>
        <select value={symbol} onChange={(e) => setSymbol(e.target.value)}>
          <option value="BTCUSDT">BTC/USDT</option>
          <option value="ETHUSDT">ETH/USDT</option>
        </select>

        <select value={side} onChange={(e) => setSide(e.target.value)}>
          <option value="BUY">买入</option>
          <option value="SELL">卖出</option>
        </select>

        <input
          type="number"
          placeholder="数量"
          value={quantity}
          onChange={(e) => setQuantity(e.target.value)}
          required
        />

        <input
          type="number"
          placeholder="价格"
          value={price}
          onChange={(e) => setPrice(e.target.value)}
          required
        />

        <button type="submit">下单</button>
      </form>

      {/* 订单列表 */}
      <div className="orders-list">
        <h3>当前订单</h3>
        {orders.map(order => (
          <div key={order.order_id} className="order-item">
            <span>{order.symbol} {order.side} {order.quantity} @ {order.price}</span>
            <span>状态: {order.status}</span>
            {order.status === 'NEW' && (
              <button onClick={() => cancelOrder(order.order_id)}>取消</button>
            )}
          </div>
        ))}
      </div>
    </div>
  );
};

export default TradingPanel;
```