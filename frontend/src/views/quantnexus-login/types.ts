export interface MarketData {
  time: string;
  value: number;
  volume: number;
}

export interface UserCredentials {
  email: string;
  pass: string;
}

export enum LoginState {
  IDLE = 'IDLE',
  LOADING = 'LOADING',
  SUCCESS = 'SUCCESS',
  ERROR = 'ERROR'
}