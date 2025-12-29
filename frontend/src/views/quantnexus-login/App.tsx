import React, { useState } from 'react';
import { LoginForm } from './components/LoginForm';
import { RegisterForm } from './components/RegisterForm';
import { MarketChart } from './components/MarketChart';
import { GlobeVisual } from './components/GlobeVisual';
import { Terminal, Cpu, Zap, BarChart3, Globe } from 'lucide-react';

export default function App() {
  const [isRegistering, setIsRegistering] = useState(false);

  return (
    <div className="min-h-screen w-full flex bg-quant-900 overflow-hidden font-sans selection:bg-quant-accent/30 selection:text-white">
      
      {/* Left Side - Login/Register Form */}
      <div className="w-full lg:w-[480px] flex flex-col justify-between p-8 lg:p-12 relative z-20 bg-quant-900 border-r border-quant-800 shadow-2xl">
        <div className="flex items-center space-x-2">
          <div className="w-8 h-8 bg-gradient-to-tr from-quant-accent to-blue-400 rounded-lg flex items-center justify-center shadow-lg shadow-blue-900/20">
            <Terminal className="w-5 h-5 text-white" />
          </div>
          <span className="text-xl font-bold tracking-tight text-white">QuantNexus</span>
        </div>

        <div className="my-auto">
          {isRegistering ? (
            <RegisterForm onLoginClick={() => setIsRegistering(false)} />
          ) : (
            <LoginForm onRegisterClick={() => setIsRegistering(true)} />
          )}
        </div>

        <div className="flex items-center justify-between text-xs text-gray-600 font-mono">
          <span>v2.4.0-stable</span>
          <div className="flex items-center space-x-4">
            <span className="flex items-center hover:text-gray-400 cursor-pointer transition-colors">
              <Globe className="w-3 h-3 mr-1" /> CN
            </span>
            <span className="flex items-center hover:text-gray-400 cursor-pointer transition-colors">
              <Zap className="w-3 h-3 mr-1" /> 系统状态: 正常
            </span>
          </div>
        </div>
      </div>

      {/* Right Side - Visuals */}
      <div className="hidden lg:flex flex-1 relative bg-quant-900 flex-col items-center justify-center p-12 overflow-hidden">
        
        {/* Abstract Background Effects */}
        <div className="absolute inset-0 bg-[linear-gradient(rgba(11,14,20,0)_0%,rgba(11,14,20,0.8)_100%),radial-gradient(circle_at_50%_0%,rgba(59,130,246,0.15)_0%,rgba(11,14,20,0)_50%)]" />
        <GlobeVisual />
        
        {/* Floating Widgets Container */}
        <div className="relative z-10 w-full max-w-4xl h-[600px] grid grid-cols-2 grid-rows-2 gap-6 p-6">
            
            {/* Main Chart Widget */}
            <div className="col-span-2 row-span-1 animate-float" style={{ animationDelay: '0s' }}>
              <MarketChart />
            </div>

            {/* Stats Widget 1 */}
            <div className="col-span-1 row-span-1 bg-quant-800/40 backdrop-blur-md rounded-xl p-6 border border-quant-700 hover:border-quant-accent/30 transition-colors group animate-float" style={{ animationDelay: '1s' }}>
              <div className="flex items-start justify-between mb-4">
                <div className="p-2 bg-purple-500/10 rounded-lg">
                  <Cpu className="w-6 h-6 text-purple-400" />
                </div>
                <span className="px-2 py-1 rounded-full bg-green-500/10 text-green-400 text-xs font-mono">
                  +12.5%
                </span>
              </div>
              <h4 className="text-gray-400 text-sm font-medium">系统延迟 (Latency)</h4>
              <p className="text-2xl font-mono text-white mt-1">14.2<span className="text-sm text-gray-500 ml-1">ms</span></p>
              <div className="mt-4 h-1.5 w-full bg-quant-700 rounded-full overflow-hidden">
                <div className="h-full bg-gradient-to-r from-purple-500 to-indigo-500 w-[70%]" />
              </div>
            </div>

            {/* Stats Widget 2 */}
            <div className="col-span-1 row-span-1 bg-quant-800/40 backdrop-blur-md rounded-xl p-6 border border-quant-700 hover:border-quant-accent/30 transition-colors group animate-float" style={{ animationDelay: '2s' }}>
               <div className="flex items-start justify-between mb-4">
                <div className="p-2 bg-emerald-500/10 rounded-lg">
                  <BarChart3 className="w-6 h-6 text-emerald-400" />
                </div>
                 <span className="px-2 py-1 rounded-full bg-blue-500/10 text-blue-400 text-xs font-mono">
                  活跃中
                </span>
              </div>
              <h4 className="text-gray-400 text-sm font-medium">24H 交易量</h4>
              <p className="text-2xl font-mono text-white mt-1">$42.8<span className="text-sm text-gray-500 ml-1">M</span></p>
               <div className="mt-4 flex space-x-1">
                 {[...Array(12)].map((_, i) => (
                   <div key={i} className={`h-1.5 flex-1 rounded-full ${i < 8 ? 'bg-emerald-500' : 'bg-quant-700'}`} />
                 ))}
               </div>
            </div>

        </div>

        {/* Bottom Text */}
        <div className="absolute bottom-12 text-center z-10">
          <h3 className="text-white font-semibold text-lg">机构级交易基础设施</h3>
          <p className="text-gray-400 text-sm mt-2 max-w-md mx-auto">
            体验专为高频交易打造的超低延迟执行引擎与高级分析工具。
          </p>
        </div>
      </div>
    </div>
  );
}