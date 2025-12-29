# 策略管理 API

## 1. 创建策略

### 请求
```http
POST /api/v1/strategies
Authorization: Bearer YOUR_JWT_TOKEN
Content-Type: application/json
```

### 请求体
```json
{
  "name": "多因子量化策略",
  "description": "基于技术指标和市场情绪的多因子策略",
  "type": "MULTI_FACTOR",
  "symbol": "BTCUSDT",
  "parameters": {
    "rsi_period": 14,
    "rsi_overbought": 70,
    "rsi_oversold": 30,
    "ma_short": 10,
    "ma_long": 20,
    "volume_threshold": 1.5,
    "position_size": 0.1,
    "stop_loss": 0.02,
    "take_profit": 0.04
  },
  "risk_management": {
    "max_drawdown": 0.15,
    "max_daily_loss": 0.05,
    "position_limit": 0.3
  }
}
```

### 成功响应 (201)
```json
{
  "success": true,
  "data": {
    "strategy_id": "strategy_123456",
    "name": "多因子量化策略",
    "type": "MULTI_FACTOR",
    "symbol": "BTCUSDT",
    "status": "INACTIVE",
    "parameters": {
      "rsi_period": 14,
      "rsi_overbought": 70,
      "rsi_oversold": 30,
      "ma_short": 10,
      "ma_long": 20,
      "volume_threshold": 1.5,
      "position_size": 0.1,
      "stop_loss": 0.02,
      "take_profit": 0.04
    },
    "created_at": "2024-12-18T10:30:00Z",
    "updated_at": "2024-12-18T10:30:00Z"
  },
  "error": null,
  "timestamp": "2024-12-18T10:30:00Z"
}
```

### 前端调用示例

```javascript
// 创建策略
async function createStrategy(strategyData) {
  const response = await fetch('http://localhost:8091/api/v1/strategies', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${localStorage.getItem('token')}`
    },
    body: JSON.stringify(strategyData)
  });
  
  const result = await response.json();
  
  if (result.success) {
    return result.data;
  } else {
    throw new Error(result.error.message);
  }
}

// 使用示例
const strategy = await createStrategy({
  name: '我的RSI策略',
  type: 'RSI',
  symbol: 'BTCUSDT',
  parameters: {
    rsi_period: 14,
    rsi_overbought: 70,
    rsi_oversold: 30,
    position_size: 0.05
  }
});
```

---

## 2. 查询策略列表

### 请求
```http
GET /api/v1/strategies?status=ACTIVE&type=MULTI_FACTOR&page=1&limit=20
Authorization: Bearer YOUR_JWT_TOKEN
```

### 查询参数
- `status`: 策略状态 (可选: ACTIVE/INACTIVE/PAUSED)
- `type`: 策略类型 (可选: RSI/MACD/MULTI_FACTOR/GRID/ARBITRAGE)
- `symbol`: 交易对 (可选)
- `page`: 页码 (默认: 1)
- `limit`: 每页数量 (默认: 20)

### 成功响应 (200)
```json
{
  "success": true,
  "data": {
    "items": [
      {
        "strategy_id": "strategy_123456",
        "name": "多因子量化策略",
        "type": "MULTI_FACTOR",
        "symbol": "BTCUSDT",
        "status": "ACTIVE",
        "performance": {
          "total_return": 0.125,
          "daily_return": 0.008,
          "max_drawdown": 0.045,
          "sharpe_ratio": 1.85,
          "win_rate": 0.68
        },
        "created_at": "2024-12-18T10:30:00Z",
        "last_signal": "2024-12-18T15:45:00Z"
      }
    ],
    "pagination": {
      "current_page": 1,
      "page_size": 20,
      "total_items": 5,
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

## 3. 查询单个策略详情

### 请求
```http
GET /api/v1/strategies/{strategy_id}
Authorization: Bearer YOUR_JWT_TOKEN
```

### 成功响应 (200)
```json
{
  "success": true,
  "data": {
    "strategy_id": "strategy_123456",
    "name": "多因子量化策略",
    "description": "基于技术指标和市场情绪的多因子策略",
    "type": "MULTI_FACTOR",
    "symbol": "BTCUSDT",
    "status": "ACTIVE",
    "parameters": {
      "rsi_period": 14,
      "rsi_overbought": 70,
      "rsi_oversold": 30,
      "ma_short": 10,
      "ma_long": 20,
      "volume_threshold": 1.5,
      "position_size": 0.1,
      "stop_loss": 0.02,
      "take_profit": 0.04
    },
    "risk_management": {
      "max_drawdown": 0.15,
      "max_daily_loss": 0.05,
      "position_limit": 0.3
    },
    "performance": {
      "total_return": 0.125,
      "daily_return": 0.008,
      "max_drawdown": 0.045,
      "sharpe_ratio": 1.85,
      "win_rate": 0.68,
      "total_trades": 156,
      "winning_trades": 106,
      "losing_trades": 50
    },
    "recent_signals": [
      {
        "signal_id": "signal_789",
        "type": "BUY",
        "price": 45100.00,
        "confidence": 0.85,
        "timestamp": "2024-12-18T15:45:00Z"
      }
    ],
    "created_at": "2024-12-18T10:30:00Z",
    "updated_at": "2024-12-18T15:45:00Z"
  },
  "error": null,
  "timestamp": "2024-12-18T10:30:00Z"
}
```

---

## 4. 启动/停止策略

### 启动策略
```http
POST /api/v1/strategies/{strategy_id}/start
Authorization: Bearer YOUR_JWT_TOKEN
```

### 停止策略
```http
POST /api/v1/strategies/{strategy_id}/stop
Authorization: Bearer YOUR_JWT_TOKEN
```

### 成功响应 (200)
```json
{
  "success": true,
  "data": {
    "strategy_id": "strategy_123456",
    "status": "ACTIVE",
    "message": "策略已启动"
  },
  "error": null,
  "timestamp": "2024-12-18T10:30:00Z"
}
```

---

## 5. 更新策略参数

### 请求
```http
PUT /api/v1/strategies/{strategy_id}
Authorization: Bearer YOUR_JWT_TOKEN
Content-Type: application/json
```

### 请求体
```json
{
  "parameters": {
    "rsi_period": 21,
    "rsi_overbought": 75,
    "rsi_oversold": 25,
    "position_size": 0.15
  },
  "risk_management": {
    "max_drawdown": 0.12,
    "stop_loss": 0.015
  }
}
```

### 成功响应 (200)
```json
{
  "success": true,
  "data": {
    "strategy_id": "strategy_123456",
    "message": "策略参数已更新",
    "updated_at": "2024-12-18T16:00:00Z"
  },
  "error": null,
  "timestamp": "2024-12-18T16:00:00Z"
}
```

---

## 6. 创建回测

### 请求
```http
POST /api/v1/backtests
Authorization: Bearer YOUR_JWT_TOKEN
Content-Type: application/json
```

### 请求体
```json
{
  "strategy_id": "strategy_123456",
  "symbol": "BTCUSDT",
  "start_date": "2024-01-01",
  "end_date": "2024-12-01",
  "initial_capital": 10000.0,
  "commission": 0.001,
  "slippage": 0.0005
}
```

### 成功响应 (201)
```json
{
  "success": true,
  "data": {
    "backtest_id": "backtest_789",
    "strategy_id": "strategy_123456",
    "status": "RUNNING",
    "progress": 0,
    "estimated_completion": "2024-12-18T10:35:00Z",
    "created_at": "2024-12-18T10:30:00Z"
  },
  "error": null,
  "timestamp": "2024-12-18T10:30:00Z"
}
```

---

## 7. 查询回测结果

### 请求
```http
GET /api/v1/backtests/{backtest_id}
Authorization: Bearer YOUR_JWT_TOKEN
```

### 成功响应 (200)
```json
{
  "success": true,
  "data": {
    "backtest_id": "backtest_789",
    "strategy_id": "strategy_123456",
    "status": "COMPLETED",
    "progress": 100,
    "results": {
      "initial_capital": 10000.0,
      "final_capital": 12500.0,
      "total_return": 0.25,
      "annual_return": 0.28,
      "max_drawdown": 0.08,
      "sharpe_ratio": 2.15,
      "sortino_ratio": 3.02,
      "win_rate": 0.72,
      "profit_factor": 1.85,
      "total_trades": 245,
      "winning_trades": 176,
      "losing_trades": 69,
      "average_win": 85.50,
      "average_loss": -42.30,
      "largest_win": 450.00,
      "largest_loss": -180.00
    },
    "equity_curve": [
      {
        "date": "2024-01-01",
        "equity": 10000.0,
        "drawdown": 0.0
      },
      {
        "date": "2024-01-02",
        "equity": 10125.0,
        "drawdown": 0.0
      }
    ],
    "trades": [
      {
        "entry_time": "2024-01-01T10:00:00Z",
        "exit_time": "2024-01-01T14:30:00Z",
        "side": "BUY",
        "entry_price": 44000.0,
        "exit_price": 44500.0,
        "quantity": 0.1,
        "pnl": 50.0,
        "commission": 4.4,
        "net_pnl": 45.6
      }
    ],
    "created_at": "2024-12-18T10:30:00Z",
    "completed_at": "2024-12-18T10:35:00Z"
  },
  "error": null,
  "timestamp": "2024-12-18T10:30:00Z"
}
```

---

## React 策略管理组件示例

```javascript
import React, { useState, useEffect } from 'react';
import { Line } from 'react-chartjs-2';

const StrategyManager = () => {
  const [strategies, setStrategies] = useState([]);
  const [selectedStrategy, setSelectedStrategy] = useState(null);
  const [backtests, setBacktests] = useState([]);
  const [showCreateForm, setShowCreateForm] = useState(false);

  useEffect(() => {
    fetchStrategies();
  }, []);

  const fetchStrategies = async () => {
    try {
      const response = await fetch('http://localhost:8091/api/v1/strategies', {
        headers: {
          'Authorization': `Bearer ${localStorage.getItem('token')}`
        }
      });
      const result = await response.json();
      if (result.success) {
        setStrategies(result.data.items);
      }
    } catch (error) {
      console.error('获取策略列表失败:', error);
    }
  };

  const startStrategy = async (strategyId) => {
    try {
      const response = await fetch(`http://localhost:8091/api/v1/strategies/${strategyId}/start`, {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${localStorage.getItem('token')}`
        }
      });
      const result = await response.json();
      if (result.success) {
        alert('策略启动成功!');
        fetchStrategies();
      }
    } catch (error) {
      alert('策略启动失败: ' + error.message);
    }
  };

  const stopStrategy = async (strategyId) => {
    try {
      const response = await fetch(`http://localhost:8091/api/v1/strategies/${strategyId}/stop`, {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${localStorage.getItem('token')}`
        }
      });
      const result = await response.json();
      if (result.success) {
        alert('策略已停止!');
        fetchStrategies();
      }
    } catch (error) {
      alert('策略停止失败: ' + error.message);
    }
  };

  const createBacktest = async (strategyId) => {
    try {
      const backtestData = {
        strategy_id: strategyId,
        symbol: 'BTCUSDT',
        start_date: '2024-01-01',
        end_date: '2024-12-01',
        initial_capital: 10000.0,
        commission: 0.001,
        slippage: 0.0005
      };

      const response = await fetch('http://localhost:8091/api/v1/backtests', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${localStorage.getItem('token')}`
        },
        body: JSON.stringify(backtestData)
      });

      const result = await response.json();
      if (result.success) {
        alert('回测已开始!');
        // 可以定期查询回测进度
        checkBacktestProgress(result.data.backtest_id);
      }
    } catch (error) {
      alert('创建回测失败: ' + error.message);
    }
  };

  const checkBacktestProgress = async (backtestId) => {
    try {
      const response = await fetch(`http://localhost:8091/api/v1/backtests/${backtestId}`, {
        headers: {
          'Authorization': `Bearer ${localStorage.getItem('token')}`
        }
      });
      const result = await response.json();
      
      if (result.success) {
        if (result.data.status === 'COMPLETED') {
          alert('回测完成!');
          // 显示回测结果
          setBacktests(prev => [...prev, result.data]);
        } else if (result.data.status === 'RUNNING') {
          // 继续检查进度
          setTimeout(() => checkBacktestProgress(backtestId), 5000);
        }
      }
    } catch (error) {
      console.error('检查回测进度失败:', error);
    }
  };

  return (
    <div className="strategy-manager">
      <h2>策略管理</h2>

      <button onClick={() => setShowCreateForm(true)}>创建新策略</button>

      {/* 策略列表 */}
      <div className="strategies-list">
        <h3>我的策略</h3>
        {strategies.map(strategy => (
          <div key={strategy.strategy_id} className="strategy-card">
            <div className="strategy-header">
              <h4>{strategy.name}</h4>
              <span className={`status ${strategy.status.toLowerCase()}`}>
                {strategy.status}
              </span>
            </div>
            
            <div className="strategy-info">
              <p>类型: {strategy.type}</p>
              <p>交易对: {strategy.symbol}</p>
              {strategy.performance && (
                <div className="performance">
                  <p>总收益: {(strategy.performance.total_return * 100).toFixed(2)}%</p>
                  <p>胜率: {(strategy.performance.win_rate * 100).toFixed(1)}%</p>
                  <p>夏普比率: {strategy.performance.sharpe_ratio?.toFixed(2)}</p>
                </div>
              )}
            </div>

            <div className="strategy-actions">
              {strategy.status === 'INACTIVE' ? (
                <button onClick={() => startStrategy(strategy.strategy_id)}>
                  启动策略
                </button>
              ) : (
                <button onClick={() => stopStrategy(strategy.strategy_id)}>
                  停止策略
                </button>
              )}
              <button onClick={() => createBacktest(strategy.strategy_id)}>
                创建回测
              </button>
              <button onClick={() => setSelectedStrategy(strategy)}>
                查看详情
              </button>
            </div>
          </div>
        ))}
      </div>

      {/* 回测结果 */}
      {backtests.length > 0 && (
        <div className="backtests-section">
          <h3>回测结果</h3>
          {backtests.map(backtest => (
            <BacktestResults key={backtest.backtest_id} backtest={backtest} />
          ))}
        </div>
      )}

      {/* 策略详情模态框 */}
      {selectedStrategy && (
        <StrategyDetailsModal 
          strategy={selectedStrategy} 
          onClose={() => setSelectedStrategy(null)} 
        />
      )}

      {/* 创建策略表单 */}
      {showCreateForm && (
        <CreateStrategyForm 
          onClose={() => setShowCreateForm(false)}
          onSuccess={fetchStrategies}
        />
      )}
    </div>
  );
};

// 回测结果组件
const BacktestResults = ({ backtest }) => {
  const equityData = {
    labels: backtest.equity_curve?.map(point => point.date) || [],
    datasets: [{
      label: '资金曲线',
      data: backtest.equity_curve?.map(point => point.equity) || [],
      borderColor: '#00d4aa',
      backgroundColor: 'rgba(0, 212, 170, 0.1)',
      fill: true
    }]
  };

  return (
    <div className="backtest-results">
      <h4>回测报告 - {backtest.backtest_id}</h4>
      
      <div className="results-summary">
        <div className="metric">
          <span>总收益率</span>
          <span>{(backtest.results.total_return * 100).toFixed(2)}%</span>
        </div>
        <div className="metric">
          <span>年化收益率</span>
          <span>{(backtest.results.annual_return * 100).toFixed(2)}%</span>
        </div>
        <div className="metric">
          <span>最大回撤</span>
          <span>{(backtest.results.max_drawdown * 100).toFixed(2)}%</span>
        </div>
        <div className="metric">
          <span>夏普比率</span>
          <span>{backtest.results.sharpe_ratio.toFixed(2)}</span>
        </div>
        <div className="metric">
          <span>胜率</span>
          <span>{(backtest.results.win_rate * 100).toFixed(1)}%</span>
        </div>
        <div className="metric">
          <span>总交易次数</span>
          <span>{backtest.results.total_trades}</span>
        </div>
      </div>

      <div className="equity-chart">
        <Line data={equityData} options={{
          responsive: true,
          plugins: {
            title: {
              display: true,
              text: '资金曲线'
            }
          },
          scales: {
            y: {
              beginAtZero: false
            }
          }
        }} />
      </div>
    </div>
  );
};

export default StrategyManager;
```