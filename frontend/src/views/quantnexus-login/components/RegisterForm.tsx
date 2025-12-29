import React, { useState, useEffect } from 'react';
import { Mail, Lock, ArrowRight, ShieldCheck, Loader2, Phone, MessageSquare, Eye, EyeOff } from 'lucide-react';
import { LoginState } from '../types';

interface RegisterFormProps {
  onLoginClick: () => void;
}

export const RegisterForm: React.FC<RegisterFormProps> = ({ onLoginClick }) => {
  const [formData, setFormData] = useState({
    email: '',
    phone: '',
    code: '',
    password: ''
  });
  const [showPassword, setShowPassword] = useState(false);
  const [registerState, setRegisterState] = useState<LoginState>(LoginState.IDLE);
  const [countdown, setCountdown] = useState(0);

  useEffect(() => {
    let timer: number;
    if (countdown > 0) {
      timer = window.setInterval(() => {
        setCountdown((prev) => prev - 1);
      }, 1000);
    }
    return () => clearInterval(timer);
  }, [countdown]);

  const handleSendCode = () => {
    if (countdown === 0 && formData.phone) {
      // Simulate sending code
      setCountdown(60);
    }
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    setRegisterState(LoginState.LOADING);
    
    // Simulate API call
    setTimeout(() => {
      setRegisterState(LoginState.SUCCESS);
    }, 2000);
  };

  return (
    <div className="w-full max-w-md mx-auto animate-[fadeIn_0.5s_ease-out]">
      <div className="text-center mb-8">
        <div className="inline-flex items-center justify-center w-12 h-12 rounded-xl bg-quant-accent/10 text-quant-accent mb-4 ring-1 ring-quant-accent/20">
            <ShieldCheck className="w-6 h-6" />
        </div>
        <h2 className="text-3xl font-bold tracking-tight text-white mb-2">申请开户</h2>
        <p className="text-gray-400">创建您的 QuantNexus 量化交易账户</p>
      </div>

      <form onSubmit={handleSubmit} className="space-y-5">
        
        {/* Email Field */}
        <div className="group relative">
          <label className="block text-xs font-medium text-gray-500 mb-1 ml-1 uppercase tracking-wider">
            电子邮箱
          </label>
          <div className="relative">
            <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
              <Mail className="h-5 w-5 text-gray-500 group-focus-within:text-quant-accent transition-colors duration-200" />
            </div>
            <input
              type="email"
              required
              value={formData.email}
              onChange={(e) => setFormData({...formData, email: e.target.value})}
              className="block w-full pl-10 pr-3 py-3 border border-quant-700 rounded-lg leading-5 bg-quant-800/50 text-gray-100 placeholder-gray-600 focus:outline-none focus:ring-2 focus:ring-quant-accent/50 focus:border-quant-accent transition-all duration-200 sm:text-sm backdrop-blur-sm"
              placeholder="name@company.com"
            />
          </div>
        </div>

        {/* Phone Field */}
        <div className="group relative">
          <label className="block text-xs font-medium text-gray-500 mb-1 ml-1 uppercase tracking-wider">
            手机号码
          </label>
          <div className="relative">
            <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
              <Phone className="h-5 w-5 text-gray-500 group-focus-within:text-quant-accent transition-colors duration-200" />
            </div>
            <input
              type="tel"
              required
              value={formData.phone}
              onChange={(e) => setFormData({...formData, phone: e.target.value})}
              className="block w-full pl-10 pr-3 py-3 border border-quant-700 rounded-lg leading-5 bg-quant-800/50 text-gray-100 placeholder-gray-600 focus:outline-none focus:ring-2 focus:ring-quant-accent/50 focus:border-quant-accent transition-all duration-200 sm:text-sm backdrop-blur-sm"
              placeholder="138 0000 0000"
            />
          </div>
        </div>

        {/* SMS Code Field */}
        <div className="group relative">
          <label className="block text-xs font-medium text-gray-500 mb-1 ml-1 uppercase tracking-wider">
            短信验证码
          </label>
          <div className="flex space-x-3">
            <div className="relative flex-1">
              <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                <MessageSquare className="h-5 w-5 text-gray-500 group-focus-within:text-quant-accent transition-colors duration-200" />
              </div>
              <input
                type="text"
                required
                maxLength={6}
                value={formData.code}
                onChange={(e) => setFormData({...formData, code: e.target.value})}
                className="block w-full pl-10 pr-3 py-3 border border-quant-700 rounded-lg leading-5 bg-quant-800/50 text-gray-100 placeholder-gray-600 focus:outline-none focus:ring-2 focus:ring-quant-accent/50 focus:border-quant-accent transition-all duration-200 sm:text-sm backdrop-blur-sm"
                placeholder="000000"
              />
            </div>
            <button
              type="button"
              onClick={handleSendCode}
              disabled={countdown > 0 || !formData.phone}
              className={`px-4 py-3 rounded-lg text-sm font-medium transition-colors min-w-[120px] border border-quant-700 ${
                countdown > 0 || !formData.phone
                  ? 'bg-quant-800 text-gray-500 cursor-not-allowed'
                  : 'bg-quant-800 hover:bg-quant-700 text-quant-accent hover:text-blue-400 hover:border-quant-accent/50'
              }`}
            >
              {countdown > 0 ? `${countdown}s 后重发` : '获取验证码'}
            </button>
          </div>
        </div>

        {/* Password Field */}
        <div className="group relative">
          <label className="block text-xs font-medium text-gray-500 mb-1 ml-1 uppercase tracking-wider">
            设置密码
          </label>
          <div className="relative">
            <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
              <Lock className="h-5 w-5 text-gray-500 group-focus-within:text-quant-accent transition-colors duration-200" />
            </div>
            <input
              type={showPassword ? "text" : "password"}
              required
              value={formData.password}
              onChange={(e) => setFormData({...formData, password: e.target.value})}
              className="block w-full pl-10 pr-10 py-3 border border-quant-700 rounded-lg leading-5 bg-quant-800/50 text-gray-100 placeholder-gray-600 focus:outline-none focus:ring-2 focus:ring-quant-accent/50 focus:border-quant-accent transition-all duration-200 sm:text-sm backdrop-blur-sm"
              placeholder="8-16位字符，包含字母和数字"
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

        <button
          type="submit"
          disabled={registerState === LoginState.LOADING || registerState === LoginState.SUCCESS}
          className={`w-full flex justify-center items-center py-3 px-4 mt-6 border border-transparent rounded-lg text-sm font-semibold text-white transition-all duration-300 transform focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-quant-900 focus:ring-quant-accent ${
            registerState === LoginState.SUCCESS
              ? 'bg-quant-success hover:bg-emerald-600'
              : 'bg-quant-accent hover:bg-blue-600 hover:shadow-lg hover:shadow-blue-900/20'
          }`}
        >
          {registerState === LoginState.LOADING ? (
            <Loader2 className="w-5 h-5 animate-spin" />
          ) : registerState === LoginState.SUCCESS ? (
            "注册成功，跳转中..."
          ) : (
            <>
              立即注册 <ArrowRight className="ml-2 w-4 h-4" />
            </>
          )}
        </button>
      </form>

      <div className="mt-8 pt-6 border-t border-quant-800 text-center">
        <p className="text-xs text-gray-500">
          已有账号？ 
          <button 
            onClick={onLoginClick}
            className="ml-1 text-quant-accent hover:text-blue-400 font-medium transition-colors focus:outline-none"
          >
            立即登录
          </button>
        </p>
      </div>
    </div>
  );
};