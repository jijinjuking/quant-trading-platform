<template>
  <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] opacity-20 pointer-events-none">
    <!-- 旋转的轨道环 -->
    <div class="absolute inset-0 rounded-full border border-quant-accent/30 animate-spin-slow" />
    <div class="absolute inset-[100px] rounded-full border border-quant-accent/20 animate-spin-reverse" />
    <div class="absolute inset-[200px] rounded-full border border-quant-accent/10 animate-spin-fast" />
    
    <!-- 中心地球 -->
    <div class="absolute inset-[250px] rounded-full bg-gradient-to-br from-quant-accent/20 to-transparent border border-quant-accent/40 animate-pulse-slow">
      <!-- 地球表面纹理 -->
      <div class="absolute inset-2 rounded-full bg-gradient-to-br from-blue-500/30 via-green-500/20 to-blue-600/30 animate-spin-earth" />
    </div>
    
    <!-- 装饰性节点 -->
    <div class="absolute top-0 left-1/2 w-2 h-2 bg-quant-accent rounded-full -translate-x-1/2 -translate-y-1 shadow-glow-blue animate-pulse" />
    <div class="absolute bottom-[200px] right-[43px] w-1.5 h-1.5 bg-quant-success rounded-full shadow-glow-green animate-pulse" style="animation-delay: 0.5s" />
    <div class="absolute top-[100px] left-[100px] w-1.5 h-1.5 bg-purple-500 rounded-full shadow-glow-purple animate-pulse" style="animation-delay: 1s" />
    <div class="absolute bottom-[100px] left-[200px] w-1 h-1 bg-yellow-500 rounded-full shadow-glow-yellow animate-pulse" style="animation-delay: 1.5s" />
    <div class="absolute top-[200px] right-[150px] w-1 h-1 bg-pink-500 rounded-full shadow-glow-pink animate-pulse" style="animation-delay: 2s" />
    
    <!-- 连接线动画 -->
    <svg class="absolute inset-0 w-full h-full">
      <defs>
        <linearGradient id="lineGradient" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" style="stop-color:#3B82F6;stop-opacity:0" />
          <stop offset="50%" style="stop-color:#3B82F6;stop-opacity:0.6" />
          <stop offset="100%" style="stop-color:#3B82F6;stop-opacity:0" />
        </linearGradient>
      </defs>
      
      <!-- 动态连接线 -->
      <path 
        d="M 300,50 Q 200,150 150,250" 
        stroke="url(#lineGradient)" 
        stroke-width="1" 
        fill="none" 
        class="animate-dash"
      />
      <path 
        d="M 500,150 Q 400,200 350,300" 
        stroke="url(#lineGradient)" 
        stroke-width="1" 
        fill="none" 
        class="animate-dash"
        style="animation-delay: 1s"
      />
      <path 
        d="M 100,300 Q 200,250 300,300" 
        stroke="url(#lineGradient)" 
        stroke-width="1" 
        fill="none" 
        class="animate-dash"
        style="animation-delay: 2s"
      />
    </svg>
    
    <!-- 数据流粒子 -->
    <div class="absolute inset-0">
      <div 
        v-for="(particle, index) in particles" 
        :key="index"
        class="absolute w-1 h-1 bg-quant-accent rounded-full animate-float-particle"
        :style="{ 
          left: particle.x + 'px', 
          top: particle.y + 'px',
          animationDelay: particle.delay + 's',
          animationDuration: particle.duration + 's'
        }"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

interface Particle {
  x: number
  y: number
  delay: number
  duration: number
}

const particles = ref<Particle[]>([])

// 生成随机粒子
const generateParticles = () => {
  const newParticles: Particle[] = []
  for (let i = 0; i < 20; i++) {
    newParticles.push({
      x: Math.random() * 600,
      y: Math.random() * 600,
      delay: Math.random() * 5,
      duration: 3 + Math.random() * 4
    })
  }
  particles.value = newParticles
}

onMounted(() => {
  generateParticles()
})
</script>

<style scoped>
/* 自定义动画 */
@keyframes spin-slow {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@keyframes spin-reverse {
  from { transform: rotate(360deg); }
  to { transform: rotate(0deg); }
}

@keyframes spin-fast {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@keyframes spin-earth {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@keyframes dash {
  0% { stroke-dasharray: 0 100; }
  50% { stroke-dasharray: 50 50; }
  100% { stroke-dasharray: 100 0; }
}

@keyframes float-particle {
  0%, 100% { 
    transform: translateY(0px) scale(0.5);
    opacity: 0;
  }
  50% { 
    transform: translateY(-20px) scale(1);
    opacity: 1;
  }
}

.animate-spin-slow {
  animation: spin-slow 60s linear infinite;
}

.animate-spin-reverse {
  animation: spin-reverse 40s linear infinite;
}

.animate-spin-fast {
  animation: spin-fast 20s linear infinite;
}

.animate-spin-earth {
  animation: spin-earth 30s linear infinite;
}

.animate-dash {
  stroke-dasharray: 100;
  animation: dash 3s ease-in-out infinite;
}

.animate-float-particle {
  animation: float-particle 4s ease-in-out infinite;
}

.animate-pulse-slow {
  animation: pulse 4s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}

/* 发光效果 */
.shadow-glow-blue {
  box-shadow: 0 0 10px rgba(59, 130, 246, 0.8), 0 0 20px rgba(59, 130, 246, 0.4);
}

.shadow-glow-green {
  box-shadow: 0 0 10px rgba(16, 185, 129, 0.8), 0 0 20px rgba(16, 185, 129, 0.4);
}

.shadow-glow-purple {
  box-shadow: 0 0 10px rgba(168, 85, 247, 0.8), 0 0 20px rgba(168, 85, 247, 0.4);
}

.shadow-glow-yellow {
  box-shadow: 0 0 8px rgba(234, 179, 8, 0.8), 0 0 16px rgba(234, 179, 8, 0.4);
}

.shadow-glow-pink {
  box-shadow: 0 0 8px rgba(236, 72, 153, 0.8), 0 0 16px rgba(236, 72, 153, 0.4);
}
</style>