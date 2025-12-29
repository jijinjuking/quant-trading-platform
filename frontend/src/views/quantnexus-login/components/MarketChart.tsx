import React, { useEffect, useState } from 'react';
import { AreaChart, Area, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';
import { Activity } from 'lucide-react';

const generateData = () => {
  const data = [];
  let val = 4000;
  for (let i = 0; i < 30; i++) {
    const change = Math.random() * 200 - 100;
    val += change;
    data.push({
      time: i.toString(),
      value: Math.abs(val),
      volume: Math.floor(Math.random() * 1000)
    });
  }
  return data;
};

export const MarketChart: React.FC = () => {
  const [data, setData] = useState(generateData());

  useEffect(() => {
    const interval = setInterval(() => {
      setData(prev => {
        const lastVal = prev[prev.length - 1].value;
        const change = Math.random() * 100 - 45; // Slight upward trend bias
        const newVal = lastVal + change;
        const newEntry = {
          time: (parseInt(prev[prev.length - 1].time) + 1).toString(),
          value: newVal,
          volume: Math.floor(Math.random() * 1000)
        };
        return [...prev.slice(1), newEntry];
      });
    }, 2000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="w-full h-full flex flex-col p-6 bg-quant-800/50 backdrop-blur-sm rounded-xl border border-quant-700 shadow-2xl overflow-hidden relative group hover:border-quant-accent/50 transition-colors duration-500">
      <div className="flex items-center justify-between mb-4 z-10">
        <div className="flex items-center space-x-2">
          <div className="p-2 bg-quant-accent/10 rounded-lg">
            <Activity className="w-5 h-5 text-quant-accent" />
          </div>
          <div>
            <h3 className="text-sm font-semibold text-gray-200">BTC/USD 永续合约</h3>
            <p className="text-xs text-gray-500">实时指数</p>
          </div>
        </div>
        <div className="text-right">
          <p className="text-lg font-mono font-bold text-quant-success">
            ${data[data.length - 1].value.toFixed(2)}
          </p>
          <p className="text-xs font-medium text-quant-success flex items-center justify-end">
            +2.45%
          </p>
        </div>
      </div>

      <div className="flex-1 min-h-[200px] w-full relative z-10">
        <ResponsiveContainer width="100%" height="100%">
          <AreaChart data={data}>
            <defs>
              <linearGradient id="colorVal" x1="0" y1="0" x2="0" y2="1">
                <stop offset="5%" stopColor="#3B82F6" stopOpacity={0.3}/>
                <stop offset="95%" stopColor="#3B82F6" stopOpacity={0}/>
              </linearGradient>
            </defs>
            <CartesianGrid strokeDasharray="3 3" stroke="#1E2433" vertical={false} />
            <XAxis dataKey="time" hide />
            <YAxis domain={['auto', 'auto']} hide />
            <Tooltip 
              contentStyle={{ backgroundColor: '#0B0E14', border: '1px solid #1E2433', borderRadius: '8px' }}
              itemStyle={{ color: '#3B82F6' }}
              labelStyle={{ display: 'none' }}
            />
            <Area 
              type="monotone" 
              dataKey="value" 
              stroke="#3B82F6" 
              strokeWidth={2}
              fillOpacity={1} 
              fill="url(#colorVal)" 
              isAnimationActive={true}
            />
          </AreaChart>
        </ResponsiveContainer>
      </div>
      
      {/* Decorative Grid Background */}
      <div className="absolute inset-0 z-0 opacity-10" 
           style={{ backgroundImage: 'radial-gradient(#3B82F6 1px, transparent 1px)', backgroundSize: '20px 20px' }}>
      </div>
    </div>
  );
};