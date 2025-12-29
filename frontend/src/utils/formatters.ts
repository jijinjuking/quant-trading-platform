/**
 * 格式化工具函数
 */

export const formatPrice = (price: number): string => {
  return price.toLocaleString('en-US', { 
    minimumFractionDigits: 2, 
    maximumFractionDigits: 2 
  })
}

export const formatPriceChange = (change: number): string => {
  const sign = change >= 0 ? '+' : ''
  return `${sign}${change.toFixed(2)}%`
}

export const formatVolume = (volume: number): string => {
  if (volume >= 1e9) {
    return (volume / 1e9).toFixed(2) + 'B'
  } else if (volume >= 1e6) {
    return (volume / 1e6).toFixed(2) + 'M'
  } else if (volume >= 1e3) {
    return (volume / 1e3).toFixed(2) + 'K'
  }
  return volume.toFixed(2)
}

export const formatQuantity = (quantity: number): string => {
  return quantity.toFixed(4)
}

export const formatBalance = (balance: number): string => {
  return balance.toFixed(2)
}

export const formatTime = (timestamp: number): string => {
  return new Date(timestamp).toLocaleTimeString()
}