import React from 'react';

export const GlobeVisual: React.FC = () => {
  return (
    <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] opacity-20 pointer-events-none">
      <div className="absolute inset-0 rounded-full border border-quant-accent/30 animate-[spin_60s_linear_infinite]" />
      <div className="absolute inset-[100px] rounded-full border border-quant-accent/20 animate-[spin_40s_linear_infinite_reverse]" />
      <div className="absolute inset-[200px] rounded-full border border-quant-accent/10 animate-[spin_20s_linear_infinite]" />
      
      {/* Decorative dots simulating nodes */}
      <div className="absolute top-0 left-1/2 w-2 h-2 bg-quant-accent rounded-full -translate-x-1/2 -translate-y-1 shadow-[0_0_10px_rgba(59,130,246,0.8)]" />
      <div className="absolute bottom-[200px] right-[43px] w-1.5 h-1.5 bg-quant-success rounded-full shadow-[0_0_10px_rgba(16,185,129,0.8)]" />
      <div className="absolute top-[100px] left-[100px] w-1.5 h-1.5 bg-purple-500 rounded-full shadow-[0_0_10px_rgba(168,85,247,0.8)]" />
    </div>
  );
};