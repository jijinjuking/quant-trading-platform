import React, { useState } from 'react';
import { Mail, Lock, ArrowRight, Eye, EyeOff, ShieldCheck, Loader2 } from 'lucide-react';
import { LoginState } from '../types';

interface LoginFormProps {
  onRegisterClick: () => void;
}

export const LoginForm: React.FC<LoginFormProps> = ({ onRegisterClick }) => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [showPassword, setShowPassword] = useState(false);
  const [loginState, setLoginState] = useState<LoginState>(LoginState.IDLE);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    setLoginState(LoginState.LOADING);
    
    // Simulate API call
    setTimeout(() => {
      setLoginState(LoginState.SUCCESS);
    }, 2000);
  };

  return (
    <div className="w-full max-w-md mx-auto animate-[fadeIn_0.5s_ease-out]">
      <div className="text-center mb-10">
        <div className="inline-flex items-center justify-center w-12 h-12 rounded-xl bg-quant-accent/10 text-quant-accent mb-4 ring-1 ring-quant-accent/20">
            <ShieldCheck className="w-6 h-6" />
        </div>
        <h2 className="text-3xl font-bold tracking-tight text-white mb-2">欢迎回来</h2>
        <p className="text-gray-400">安全接入 QuantNexus 交易终端</p>
      </div>

      <form onSubmit={handleSubmit} className="space-y-6">
        <div className="space-y-4">
          <div className="group relative">
            <label className="block text-xs font-medium text-gray-500 mb-1 ml-1 uppercase tracking-wider">
              工作邮箱
            </label>
            <div className="relative">
              <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                <Mail className="h-5 w-5 text-gray-500 group-focus-within:text-quant-accent transition-colors duration-200" />
              </div>
              <input
                type="email"
                required
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                className="block w-full pl-10 pr-3 py-3 border border-quant-700 rounded-lg leading-5 bg-quant-800/50 text-gray-100 placeholder-gray-600 focus:outline-none focus:ring-2 focus:ring-quant-accent/50 focus:border-quant-accent transition-all duration-200 sm:text-sm backdrop-blur-sm"
                placeholder="trader@quantnexus.com"
              />
            </div>
          </div>

          <div className="group relative">
            <label className="block text-xs font-medium text-gray-500 mb-1 ml-1 uppercase tracking-wider">
              密码
            </label>
            <div className="relative">
              <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                <Lock className="h-5 w-5 text-gray-500 group-focus-within:text-quant-accent transition-colors duration-200" />
              </div>
              <input
                type={showPassword ? "text" : "password"}
                required
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                className="block w-full pl-10 pr-10 py-3 border border-quant-700 rounded-lg leading-5 bg-quant-800/50 text-gray-100 placeholder-gray-600 focus:outline-none focus:ring-2 focus:ring-quant-accent/50 focus:border-quant-accent transition-all duration-200 sm:text-sm backdrop-blur-sm"
                placeholder="••••••••••••"
              />
              <button
                type="button"
                onClick={() => setShowPassword(!showPassword)}
                className="absolute inset-y-0 right-0 pr-3 flex items-center text-gray-500 hover:text-gray-300 focus:outline-none"
              >
                {showPassword ? (
                  <EyeOff className="h-5 w-5" />
                ) : (
                  <Eye className="h-5 w-5" />
                )}
              </button>
            </div>
          </div>
        </div>

        <div className="flex items-center justify-between">
          <div className="flex items-center">
            <input
              id="remember-me"
              name="remember-me"
              type="checkbox"
              className="h-4 w-4 rounded border-quant-700 bg-quant-800 text-quant-accent focus:ring-quant-accent/50 focus:ring-offset-0 transition-colors cursor-pointer"
            />
            <label htmlFor="remember-me" className="ml-2 block text-sm text-gray-400 cursor-pointer select-none">
              保持登录
            </label>
          </div>

          <div className="text-sm">
            <a href="#" className="font-medium text-quant-accent hover:text-blue-400 transition-colors">
              忘记密码？
            </a>
          </div>
        </div>

        <button
          type="submit"
          disabled={loginState === LoginState.LOADING || loginState === LoginState.SUCCESS}
          className={`w-full flex justify-center items-center py-3 px-4 border border-transparent rounded-lg text-sm font-semibold text-white transition-all duration-300 transform focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-quant-900 focus:ring-quant-accent ${
            loginState === LoginState.SUCCESS
              ? 'bg-quant-success hover:bg-emerald-600'
              : 'bg-quant-accent hover:bg-blue-600 hover:shadow-lg hover:shadow-blue-900/20'
          }`}
        >
          {loginState === LoginState.LOADING ? (
            <Loader2 className="w-5 h-5 animate-spin" />
          ) : loginState === LoginState.SUCCESS ? (
            "验证成功"
          ) : (
            <>
              登录终端 <ArrowRight className="ml-2 w-4 h-4" />
            </>
          )}
        </button>
      </form>

      <div className="mt-8 pt-6 border-t border-quant-800">
        <p className="text-center text-xs text-gray-500">
          采用企业级端对端加密技术保护
          <br />
          还没有账号？ 
          <button 
            onClick={onRegisterClick} 
            className="ml-1 text-quant-accent hover:text-blue-400 font-medium transition-colors focus:outline-none"
          >
            申请开通
          </button>
        </p>
      </div>
    </div>
  );
};