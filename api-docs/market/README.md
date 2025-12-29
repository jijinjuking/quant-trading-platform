# 市场数据 API

## 1. 获取K线数据

### 请求
```http
GET /api/v1/market/klines?symbol=BTCUSDT&interval=1h&limit=100&start_time=1703721600&end_time=1703808000
```

### 查询参数
- `symbol`: 交易对 (必需, 如: BTCUSDT, ETHUSDT)
- `interval`: 时间间隔 (必需: 1m, 5m, 15m, 30m, 1h, 4h, 1d, 1w, 1M)
- `limit`: 返回数量 (可选, 默认: 500, 最大: 1000)
- `start_time`: 开始时间戳 (可选)
- `end_time`: 结束时间戳 (可选)

### 成功响应 (200)
```json
{
  "success": true,
  "data": [
    {
      "open_time": 1703721600000,
      "open": "44950.00",
      "high": "45200.00",
      "low": "44800.00",
      "close": "45100.00",
      "volume": "125.45",
      "close_time": 1703725199999,
      "quote_volume": "5654321.50",
      "count": 1250,
      "taker_buy_volume": "62.75",
      "taker_buy_quote_volume": "2827160.75"
    }
  ],
  "error": null,
  "timestamp": "2024-12-18T10:30:00Z"
}
```

### 前端调用示例

**获取K线数据**
```javascript
async function getKlineData(symbol, interval, limit = 100) {
  const params = new URLSearchParams({
    symbol,
    interval,
    limit: limit.toString()
  });

  const response = await fetch(`http://localhost:8091/api/v1/market/klines?${params}`);
  const result = await response.json();
  
  if (result.success) {
    return result.data.map(kline => ({
      timestamp: kline.open_time,
      open: parseFloat(kline.open),
      high: parseFloat(kline.high),
      low: parseFloat(kline.low),
      close: parseFloat(kline.close),
      volume: parseFloat(kline.volume)
    }));
  } else {
    throw new Error(result.error.message);
  }
}

// 使用示例
const klines = await getKlineData('BTCUSDT', '1h', 100);
```

**TradingView 图表集成**
```javascript
import { createChart } from 'lightweight-charts';

const ChartComponent = ({ symbol }) => {
  const chartRef = useRef();
  const [chart, setChart] = useState(null);

  useEffect(() => {
    const chartInstance = createChart(chartRef.current, {
      width: 800,
      height: 400,
      layout: {
        backgroundColor: '#ffffff',
        textColor: '#333',
      },
      grid: {
        vertLines: { color: '#f0f0f0' },
        horzLines: { color: '#f0f0f0' },
      },
    });

    const candlestickSeries = chartInstance.addCandlestickSeries();
    setChart({ instance: chartInstance, series: candlestickSeries });

    return () => chartInstance.remove();
  }, []);

  useEffect(() => {
    if (chart && symbol) {
      loadKlineData();
    }
  }, [chart, symbol]);

  const loadKlineData = async () => {
    try {
      const klines = await getKlineData(symbol, '1h', 200);
      const chartData = klines.map(kline => ({
        time: kline.timestamp / 1000,
        open: kline.open,
        high: kline.high,
        low: kline.low,
        close: kline.close
      }));
      
      chart.series.setData(chartData);
    } catch (error) {
      console.error('加载K线数据失败:', error);
    }
  };

  return <div ref={chartRef} />;
};
```

---

## 2. 获取实时价格

### 请求
```http
GET /api/v1/market/tickers?symbol=BTCUSDT
```

### 查询参数
- `symbol`: 交易对 (可选, 不传则返回所有交易对)

### 成功响应 (200)
```json
{
  "success": true,
  "data": [
    {
      "symbol": "BTCUSDT",
      "price": "45100.00",
      "price_change": "150.00",
      "price_change_percent": "0.33",
      "high_24h": "45500.00",
      "low_24h": "44200.00",
      "volume_24h": "12450.75",
      "quote_volume_24h": "561234567.50",
      "open_24h": "44950.00",
      "timestamp": 1703808000000
    }
  ],
  "error": null,
  "timestamp": "2024-12-18T10:30:00Z"
}
```

### WebSocket 实时价格推送

**连接WebSocket**
```javascript
class PriceWebSocket {
  constructor(symbols) {
    this.symbols = symbols;
    this.ws = null;
    this.callbacks = new Map();
  }

  connect() {
    this.ws = new WebSocket('ws://localhost:8091/ws/market/ticker');
    
    this.ws.onopen = () => {
      console.log('WebSocket连接成功');
      // 订阅价格推送
      this.subscribe(this.symbols);
    };

    this.ws.onmessage = (event) => {
      const data = JSON.parse(event.data);
      if (data.type === 'ticker') {
        this.handlePriceUpdate(data.data);
      }
    };

    this.ws.onclose = () => {
      console.log('WebSocket连接关闭');
      // 重连逻辑
      setTimeout(() => this.connect(), 5000);
    };
  }

  subscribe(symbols) {
    const message = {
      method: 'SUBSCRIBE',
      params: symbols.map(symbol => `${symbol.toLowerCase()}@ticker`),
      id: Date.now()
    };
    this.ws.send(JSON.stringify(message));
  }

  onPriceUpdate(symbol, callback) {
    this.callbacks.set(symbol, callback);
  }

  handlePriceUpdate(tickerData) {
    const callback = this.callbacks.get(tickerData.symbol);
    if (callback) {
      callback(tickerData);
    }
  }
}

// 使用示例
const priceWs = new PriceWebSocket(['BTCUSDT', 'ETHUSDT']);
priceWs.connect();

priceWs.onPriceUpdate('BTCUSDT', (ticker) => {
  console.log(`BTC价格更新: ${ticker.price}`);
  // 更新UI
  document.getElementById('btc-price').textContent = ticker.price;
});
```

---

## 3. 获取订单簿

### 请求
```http
GET /api/v1/market/orderbook?symbol=BTCUSDT&limit=20
```

### 查询参数
- `symbol`: 交易对 (必需)
- `limit`: 深度档位 (可选: 5, 10, 20, 50, 100, 500, 1000, 默认: 100)

### 成功响应 (200)
```json
{
  "success": true,
  "data": {
    "symbol": "BTCUSDT",
    "bids": [
      ["45090.00", "0.125"],
      ["45085.00", "0.250"],
      ["45080.00", "0.100"]
    ],
    "asks": [
      ["45100.00", "0.150"],
      ["45105.00", "0.200"],
      ["45110.00", "0.175"]
    ],
    "timestamp": 1703808000000
  },
  "error": null,
  "timestamp": "2024-12-18T10:30:00Z"
}
```

### 订单簿组件示例

```javascript
import React, { useState, useEffect } from 'react';

const OrderBook = ({ symbol }) => {
  const [orderbook, setOrderbook] = useState({ bids: [], asks: [] });

  useEffect(() => {
    fetchOrderbook();
    const interval = setInterval(fetchOrderbook, 1000); // 每秒更新
    return () => clearInterval(interval);
  }, [symbol]);

  const fetchOrderbook = async () => {
    try {
      const response = await fetch(`http://localhost:8091/api/v1/market/orderbook?symbol=${symbol}&limit=20`);
      const result = await response.json();
      
      if (result.success) {
        setOrderbook(result.data);
      }
    } catch (error) {
      console.error('获取订单簿失败:', error);
    }
  };

  return (
    <div className="orderbook">
      <h3>订单簿 - {symbol}</h3>
      
      <div className="orderbook-content">
        {/* 卖单 (asks) */}
        <div className="asks">
          <div className="header">
            <span>价格</span>
            <span>数量</span>
            <span>累计</span>
          </div>
          {orderbook.asks.slice().reverse().map(([price, quantity], index) => (
            <div key={index} className="order-row ask">
              <span className="price">{parseFloat(price).toFixed(2)}</span>
              <span className="quantity">{parseFloat(quantity).toFixed(6)}</span>
              <span className="total">{(parseFloat(price) * parseFloat(quantity)).toFixed(2)}</span>
            </div>
          ))}
        </div>

        {/* 当前价格 */}
        <div className="current-price">
          <span>当前价格: {orderbook.asks[0] ? orderbook.asks[0][0] : '--'}</span>
        </div>

        {/* 买单 (bids) */}
        <div className="bids">
          {orderbook.bids.map(([price, quantity], index) => (
            <div key={index} className="order-row bid">
              <span className="price">{parseFloat(price).toFixed(2)}</span>
              <span className="quantity">{parseFloat(quantity).toFixed(6)}</span>
              <span className="total">{(parseFloat(price) * parseFloat(quantity)).toFixed(2)}</span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default OrderBook;
```

---

## 4. 获取最新成交

### 请求
```http
GET /api/v1/market/trades?symbol=BTCUSDT&limit=50
```

### 查询参数
- `symbol`: 交易对 (必需)
- `limit`: 返回数量 (可选, 默认: 50, 最大: 1000)

### 成功响应 (200)
```json
{
  "success": true,
  "data": [
    {
      "id": "trade_12345",
      "price": "45100.00",
      "quantity": "0.125",
      "timestamp": 1703808000000,
      "is_buyer_maker": false
    },
    {
      "id": "trade_12344",
      "price": "45095.00",
      "quantity": "0.250",
      "timestamp": 1703807995000,
      "is_buyer_maker": true
    }
  ],
  "error": null,
  "timestamp": "2024-12-18T10:30:00Z"
}
```

---

## React 市场数据面板示例

```javascript
import React, { useState, useEffect } from 'react';

const MarketDataPanel = () => {
  const [selectedSymbol, setSelectedSymbol] = useState('BTCUSDT');
  const [tickers, setTickers] = useState([]);
  const [klines, setKlines] = useState([]);
  const [trades, setTrades] = useState([]);

  useEffect(() => {
    fetchAllTickers();
    fetchKlines();
    fetchRecentTrades();
  }, [selectedSymbol]);

  const fetchAllTickers = async () => {
    try {
      const response = await fetch('http://localhost:8091/api/v1/market/tickers');
      const result = await response.json();
      if (result.success) {
        setTickers(result.data);
      }
    } catch (error) {
      console.error('获取价格数据失败:', error);
    }
  };

  const fetchKlines = async () => {
    try {
      const response = await fetch(`http://localhost:8091/api/v1/market/klines?symbol=${selectedSymbol}&interval=1h&limit=24`);
      const result = await response.json();
      if (result.success) {
        setKlines(result.data);
      }
    } catch (error) {
      console.error('获取K线数据失败:', error);
    }
  };

  const fetchRecentTrades = async () => {
    try {
      const response = await fetch(`http://localhost:8091/api/v1/market/trades?symbol=${selectedSymbol}&limit=20`);
      const result = await response.json();
      if (result.success) {
        setTrades(result.data);
      }
    } catch (error) {
      console.error('获取成交数据失败:', error);
    }
  };

  const currentTicker = tickers.find(t => t.symbol === selectedSymbol);

  return (
    <div className="market-data-panel">
      <h2>市场数据</h2>

      {/* 交易对选择 */}
      <select value={selectedSymbol} onChange={(e) => setSelectedSymbol(e.target.value)}>
        {tickers.map(ticker => (
          <option key={ticker.symbol} value={ticker.symbol}>
            {ticker.symbol}
          </option>
        ))}
      </select>

      {/* 当前价格信息 */}
      {currentTicker && (
        <div className="price-info">
          <h3>{currentTicker.symbol}</h3>
          <div className="price">{currentTicker.price}</div>
          <div className={`change ${parseFloat(currentTicker.price_change) >= 0 ? 'positive' : 'negative'}`}>
            {currentTicker.price_change} ({currentTicker.price_change_percent}%)
          </div>
          <div className="stats">
            <span>24h高: {currentTicker.high_24h}</span>
            <span>24h低: {currentTicker.low_24h}</span>
            <span>24h量: {currentTicker.volume_24h}</span>
          </div>
        </div>
      )}

      {/* 最新成交 */}
      <div className="recent-trades">
        <h4>最新成交</h4>
        <div className="trades-list">
          {trades.map(trade => (
            <div key={trade.id} className={`trade-item ${trade.is_buyer_maker ? 'sell' : 'buy'}`}>
              <span className="price">{trade.price}</span>
              <span className="quantity">{trade.quantity}</span>
              <span className="time">{new Date(trade.timestamp).toLocaleTimeString()}</span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default MarketDataPanel;
```

## CSS 样式示例

```css
.market-data-panel {
  padding: 20px;
  background: #f8f9fa;
}

.price-info {
  background: white;
  padding: 20px;
  border-radius: 8px;
  margin: 20px 0;
}

.price {
  font-size: 2em;
  font-weight: bold;
  color: #333;
}

.change.positive {
  color: #00d4aa;
}

.change.negative {
  color: #f84960;
}

.orderbook {
  background: white;
  border-radius: 8px;
  padding: 15px;
}

.order-row {
  display: flex;
  justify-content: space-between;
  padding: 2px 0;
  font-family: monospace;
}

.ask .price {
  color: #f84960;
}

.bid .price {
  color: #00d4aa;
}

.recent-trades .trade-item {
  display: flex;
  justify-content: space-between;
  padding: 3px 0;
  font-family: monospace;
}

.trade-item.buy .price {
  color: #00d4aa;
}

.trade-item.sell .price {
  color: #f84960;
}
```