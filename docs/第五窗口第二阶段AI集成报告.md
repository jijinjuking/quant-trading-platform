# 窗口5 - Phase 2 AI集成任务

**时间**: 2024-12-20 18:45  
**执行者**: 窗口5 (AI集成工程师)  
**优先级**: 🔥 P1 - 智能功能  
**预计时间**: 4.5小时  

---

## 🎯 **任务概述**

集成DeepSeek AI API，开发量化交易专用的AI助手功能，实现智能策略生成和风险评估。

### **核心目标**:
1. **DeepSeek API集成** - 替换Google Gemini
2. **量化交易AI助手** - 专业的交易建议
3. **智能策略生成** - AI驱动的策略创建
4. **风险智能评估** - AI风险分析

---

## 📋 **详细任务清单**

### **阶段1: DeepSeek API集成 (19:30-21:30, 2小时)**

#### **1.1 DeepSeek API客户端实现 (60分钟)**

创建 `quant-backend66/src/api/deepseekApiClient.ts`：
```typescript
export interface DeepSeekConfig {
  apiKey: string;
  baseUrl: string;
  model: string;
}

export class DeepSeekApiClient {
  private config: DeepSeekConfig;

  constructor(config: DeepSeekConfig) {
    this.config = config;
  }

  async chat(messages: Array<{role: string, content: string}>): Promise<string> {
    try {
      const response = await fetch(`${this.config.baseUrl}/chat/completions`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${this.config.apiKey}`
        },
        body: JSON.stringify({
          model: this.config.model,
          messages: messages,
          temperature: 0.7,
          max_tokens: 2000,
          stream: false
        })
      });

      if (!response.ok) {
        throw new Error(`DeepSeek API error: ${response.status}`);
      }

      const data = await response.json();
      return data.choices[0]?.message?.content || '';
    } catch (error) {
      console.error('DeepSeek API call failed:', error);
      throw error;
    }
  }

  // 量化交易专用方法
  async analyzeMarketData(marketData: any): Promise<string> {
    const prompt = `
作为专业的量化交易分析师，请分析以下市场数据：

当前价格数据：
${JSON.stringify(marketData, null, 2)}

请提供：
1. 技术分析观点
2. 短期价格预测
3. 风险评估
4. 交易建议

请用专业但易懂的语言回答。
    `;

    return this.chat([
      { role: 'system', content: '你是一个专业的量化交易分析师，具有丰富的市场分析经验。' },
      { role: 'user', content: prompt }
    ]);
  }

  async generateTradingStrategy(params: {
    symbol: string;
    timeframe: string;
    riskLevel: 'LOW' | 'MEDIUM' | 'HIGH';
    capital: number;
  }): Promise<string> {
    const prompt = `
请为以下参数设计一个量化交易策略：

交易对: ${params.symbol}
时间周期: ${params.timeframe}
风险等级: ${params.riskLevel}
资金规模: $${params.capital}

请提供：
1. 策略名称和概述
2. 入场条件
3. 出场条件
4. 风险管理规则
5. 预期收益和风险
6. 具体的技术指标参数

请提供可执行的策略细节。
    `;

    return this.chat([
      { role: 'system', content: '你是一个专业的量化策略设计师，擅长创建盈利的交易策略。' },
      { role: 'user', content: prompt }
    ]);
  }

  async assessRisk(position: any): Promise<string> {
    const prompt = `
请评估以下交易持仓的风险：

持仓信息：
${JSON.stringify(position, null, 2)}

请分析：
1. 当前风险等级
2. 潜在损失
3. 风险控制建议
4. 是否需要调整仓位

请提供具体的风险管理建议。
    `;

    return this.chat([
      { role: 'system', content: '你是一个专业的风险管理专家，专注于交易风险控制。' },
      { role: 'user', content: prompt }
    ]);
  }
}

// 配置DeepSeek客户端
export const deepseekClient = new DeepSeekApiClient({
  apiKey: process.env.DEEPSEEK_API_KEY || 'your-deepseek-api-key',
  baseUrl: 'https://api.deepseek.com/v1',
  model: 'deepseek-chat'
});
```

#### **1.2 AI助手界面组件 (60分钟)**

创建 `quant-backend66/src/components/AITradingAssistant.tsx`：
```typescript
import React, { useState, useRef, useEffect } from 'react';
import { Bot, Send, TrendingUp, AlertTriangle, Lightbulb } from 'lucide-react';
import { deepseekClient } from '../api/deepseekApiClient';

interface Message {
  id: string;
  role: 'user' | 'assistant';
  content: string;
  timestamp: Date;
  type?: 'analysis' | 'strategy' | 'risk' | 'general';
}

export const AITradingAssistant: React.FC<{
  currentPrice: number;
  positions: any[];
  marketData: any;
}> = ({ currentPrice, positions, marketData }) => {
  const [messages, setMessages] = useState<Message[]>([
    {
      id: '1',
      role: 'assistant',
      content: '你好！我是DeepSeek量化交易助手。我可以帮你分析市场、生成策略、评估风险。有什么需要帮助的吗？',
      timestamp: new Date(),
      type: 'general'
    }
  ]);
  const [inputMessage, setInputMessage] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const sendMessage = async (content: string, type: 'general' | 'analysis' | 'strategy' | 'risk' = 'general') => {
    const userMessage: Message = {
      id: Date.now().toString(),
      role: 'user',
      content,
      timestamp: new Date(),
      type
    };

    setMessages(prev => [...prev, userMessage]);
    setIsLoading(true);

    try {
      let response = '';
      
      switch (type) {
        case 'analysis':
          response = await deepseekClient.analyzeMarketData(marketData);
          break;
        case 'strategy':
          response = await deepseekClient.generateTradingStrategy({
            symbol: 'BTCUSDT',
            timeframe: '15m',
            riskLevel: 'MEDIUM',
            capital: 10000
          });
          break;
        case 'risk':
          response = await deepseekClient.assessRisk(positions[0]);
          break;
        default:
          response = await deepseekClient.chat([
            { role: 'user', content }
          ]);
      }

      const assistantMessage: Message = {
        id: (Date.now() + 1).toString(),
        role: 'assistant',
        content: response,
        timestamp: new Date(),
        type
      };

      setMessages(prev => [...prev, assistantMessage]);
    } catch (error) {
      const errorMessage: Message = {
        id: (Date.now() + 1).toString(),
        role: 'assistant',
        content: '抱歉，AI服务暂时不可用。请稍后再试。',
        timestamp: new Date(),
        type: 'general'
      };
      setMessages(prev => [...prev, errorMessage]);
    } finally {
      setIsLoading(false);
    }
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (inputMessage.trim() && !isLoading) {
      sendMessage(inputMessage.trim());
      setInputMessage('');
    }
  };

  const quickActions = [
    {
      label: '分析市场',
      icon: TrendingUp,
      action: () => sendMessage('请分析当前市场情况', 'analysis'),
      color: 'text-blue-400'
    },
    {
      label: '生成策略',
      icon: Lightbulb,
      action: () => sendMessage('请为我生成一个交易策略', 'strategy'),
      color: 'text-green-400'
    },
    {
      label: '风险评估',
      icon: AlertTriangle,
      action: () => sendMessage('请评估我的持仓风险', 'risk'),
      color: 'text-yellow-400'
    }
  ];

  return (
    <div className="flex flex-col h-full bg-[#0d1014] border border-[#2b3139] rounded-lg">
      {/* 头部 */}
      <div className="flex items-center p-3 border-b border-[#2b3139] bg-[#111418]">
        <Bot className="w-5 h-5 text-[#3b82f6] mr-2" />
        <div>
          <div className="text-sm font-bold text-white">DeepSeek AI助手</div>
          <div className="text-xs text-gray-400">量化交易专家</div>
        </div>
        <div className="ml-auto">
          <div className="w-2 h-2 bg-green-500 rounded-full"></div>
        </div>
      </div>

      {/* 快捷操作 */}
      <div className="p-3 border-b border-[#2b3139] bg-[#0f1114]">
        <div className="flex space-x-2">
          {quickActions.map((action, index) => (
            <button
              key={index}
              onClick={action.action}
              disabled={isLoading}
              className={`flex items-center space-x-1 px-2 py-1 rounded text-xs font-medium bg-gray-800 hover:bg-gray-700 transition-colors disabled:opacity-50 ${action.color}`}
            >
              <action.icon className="w-3 h-3" />
              <span>{action.label}</span>
            </button>
          ))}
        </div>
      </div>

      {/* 消息列表 */}
      <div className="flex-1 overflow-y-auto p-3 space-y-3">
        {messages.map((message) => (
          <div
            key={message.id}
            className={`flex ${message.role === 'user' ? 'justify-end' : 'justify-start'}`}
          >
            <div
              className={`max-w-[80%] p-3 rounded-lg text-sm ${
                message.role === 'user'
                  ? 'bg-[#3b82f6] text-white'
                  : 'bg-[#1e2329] text-gray-200 border border-[#2b3139]'
              }`}
            >
              {message.type && message.type !== 'general' && (
                <div className="text-xs opacity-70 mb-1 uppercase font-bold">
                  {message.type === 'analysis' && '📊 市场分析'}
                  {message.type === 'strategy' && '💡 策略生成'}
                  {message.type === 'risk' && '⚠️ 风险评估'}
                </div>
              )}
              <div className="whitespace-pre-wrap">{message.content}</div>
              <div className="text-xs opacity-50 mt-1">
                {message.timestamp.toLocaleTimeString()}
              </div>
            </div>
          </div>
        ))}
        {isLoading && (
          <div className="flex justify-start">
            <div className="bg-[#1e2329] border border-[#2b3139] p-3 rounded-lg">
              <div className="flex items-center space-x-2">
                <div className="w-2 h-2 bg-blue-500 rounded-full animate-pulse"></div>
                <div className="w-2 h-2 bg-blue-500 rounded-full animate-pulse" style={{animationDelay: '0.2s'}}></div>
                <div className="w-2 h-2 bg-blue-500 rounded-full animate-pulse" style={{animationDelay: '0.4s'}}></div>
                <span className="text-xs text-gray-400">AI正在思考...</span>
              </div>
            </div>
          </div>
        )}
        <div ref={messagesEndRef} />
      </div>

      {/* 输入框 */}
      <form onSubmit={handleSubmit} className="p-3 border-t border-[#2b3139] bg-[#111418]">
        <div className="flex space-x-2">
          <input
            type="text"
            value={inputMessage}
            onChange={(e) => setInputMessage(e.target.value)}
            placeholder="询问AI助手..."
            className="flex-1 px-3 py-2 bg-[#1e2329] border border-[#2b3139] rounded text-white text-sm focus:outline-none focus:border-[#3b82f6]"
            disabled={isLoading}
          />
          <button
            type="submit"
            disabled={!inputMessage.trim() || isLoading}
            className="px-3 py-2 bg-[#3b82f6] text-white rounded hover:bg-[#2563eb] disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            <Send className="w-4 h-4" />
          </button>
        </div>
      </form>
    </div>
  );
};
```

### **阶段2: 智能策略生成 (21:30-22:30, 1小时)**

#### **2.1 策略生成引擎 (30分钟)**

创建 `quant-backend66/src/services/strategyGenerator.ts`：
```typescript
import { deepseekClient } from '../api/deepseekApiClient';

export interface StrategyParams {
  symbol: string;
  timeframe: string;
  riskLevel: 'LOW' | 'MEDIUM' | 'HIGH';
  capital: number;
  strategyType: 'TREND' | 'MEAN_REVERSION' | 'BREAKOUT' | 'GRID';
}

export interface GeneratedStrategy {
  id: string;
  name: string;
  description: string;
  entryConditions: string[];
  exitConditions: string[];
  riskManagement: string[];
  parameters: Record<string, any>;
  expectedReturn: string;
  maxDrawdown: string;
  confidence: number;
}

export class StrategyGenerator {
  async generateStrategy(params: StrategyParams): Promise<GeneratedStrategy> {
    const prompt = `
请设计一个${params.strategyType}类型的量化交易策略：

参数：
- 交易对: ${params.symbol}
- 时间周期: ${params.timeframe}
- 风险等级: ${params.riskLevel}
- 资金: $${params.capital}

请返回JSON格式的策略，包含：
{
  "name": "策略名称",
  "description": "策略描述",
  "entryConditions": ["入场条件1", "入场条件2"],
  "exitConditions": ["出场条件1", "出场条件2"],
  "riskManagement": ["风险管理规则1", "风险管理规则2"],
  "parameters": {
    "stopLoss": "止损百分比",
    "takeProfit": "止盈百分比",
    "positionSize": "仓位大小"
  },
  "expectedReturn": "预期年化收益",
  "maxDrawdown": "最大回撤",
  "confidence": 85
}
    `;

    try {
      const response = await deepseekClient.chat([
        { role: 'system', content: '你是专业的量化策略设计师，请返回有效的JSON格式策略。' },
        { role: 'user', content: prompt }
      ]);

      // 解析AI返回的JSON
      const jsonMatch = response.match(/\{[\s\S]*\}/);
      if (jsonMatch) {
        const strategyData = JSON.parse(jsonMatch[0]);
        return {
          id: Date.now().toString(),
          ...strategyData
        };
      }

      // 如果解析失败，返回默认策略
      return this.getDefaultStrategy(params);
    } catch (error) {
      console.error('Strategy generation failed:', error);
      return this.getDefaultStrategy(params);
    }
  }

  private getDefaultStrategy(params: StrategyParams): GeneratedStrategy {
    return {
      id: Date.now().toString(),
      name: `AI ${params.strategyType} Strategy`,
      description: `基于${params.symbol}的${params.strategyType}策略`,
      entryConditions: ['技术指标确认', '成交量配合'],
      exitConditions: ['止盈目标达成', '止损触发'],
      riskManagement: ['单笔风险不超过2%', '最大仓位50%'],
      parameters: {
        stopLoss: '2%',
        takeProfit: '6%',
        positionSize: '10%'
      },
      expectedReturn: '15-25%',
      maxDrawdown: '8-12%',
      confidence: 75
    };
  }
}

export const strategyGenerator = new StrategyGenerator();
```

#### **2.2 策略展示组件 (30分钟)**

创建策略卡片组件，集成到主界面中。

### **阶段3: 智能风险评估 (22:30-23:00, 30分钟)**

#### **3.1 风险评估引擎 (30分钟)**

创建实时风险监控和AI评估功能。

---

## 🎯 **成功验证标准**

### **DeepSeek集成验证**:
```typescript
// 测试API连接
const response = await deepseekClient.chat([
  { role: 'user', content: '你好，请介绍一下你的功能' }
]);
console.log('DeepSeek response:', response);
```

### **AI助手验证**:
```typescript
// 在前端界面中
// 1. 点击"分析市场"按钮
// 2. 观察AI返回专业的市场分析
// 3. 测试策略生成功能
// 4. 验证风险评估功能
```

---

## 🚨 **常见问题和解决方案**

### **问题1: DeepSeek API密钥**
```typescript
// 在环境变量中设置
// .env.local
DEEPSEEK_API_KEY=your-actual-api-key

// 或在代码中临时设置
const apiKey = 'sk-your-deepseek-api-key';
```

### **问题2: API调用失败**
```typescript
// 添加重试机制
async function callWithRetry(fn: () => Promise<string>, retries = 3): Promise<string> {
  try {
    return await fn();
  } catch (error) {
    if (retries > 0) {
      await new Promise(resolve => setTimeout(resolve, 1000));
      return callWithRetry(fn, retries - 1);
    }
    throw error;
  }
}
```

---

## 📊 **进度汇报格式**

每30分钟汇报一次：
```
🔄 [窗口5] DeepSeek AI集成 - 进行中 (60%)
✅ DeepSeek API客户端完成
✅ AI助手界面完成
🔄 策略生成引擎 (进行中)
⏳ 风险评估引擎 (待开始)
```

---

## 🏆 **完成标志**

当你看到以下结果时，任务完成：

1. **AI助手正常对话** - 可以与DeepSeek AI正常交流
2. **市场分析功能** - AI能提供专业的市场分析
3. **策略生成功能** - AI能生成可执行的交易策略
4. **风险评估功能** - AI能评估持仓风险
5. **界面集成完成** - AI助手完美集成到交易界面

**这将为平台增加强大的AI智能功能！** 🤖

---

**立即开始执行！让我们为量化交易平台注入AI的力量！** 🚀