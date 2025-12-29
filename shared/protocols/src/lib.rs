// src/lib.rs

pub mod grpc;
pub mod http;
pub mod kafka;
pub mod websocket;
pub mod standards;

// 暂时注释掉导出，避免编译错误
// pub use grpc::{GrpcService, GrpcClient, GrpcServer, GrpcConfig, GrpcError, GrpcMessage, GrpcHandler, GrpcInterceptor};
// pub use http::{HttpRequest, HttpResponse, HttpClient, HttpServer, HttpConfig, HttpError, HttpHandler, HttpMiddleware};
// pub use kafka::{KafkaProducer, KafkaConsumer, KafkaConfig, KafkaError, KafkaMessage, KafkaPartition, KafkaOffset, KafkaMessageRouter as KafkaRouter, KafkaMessageHandler as KafkaHandler};
// pub use websocket::{WebSocketConnection, WebSocketMessage, WebSocketConfig, WebSocketError, WebSocketHandler, WebSocketServer, WebSocketClient, WebSocketMessageRouter as WsRouter, WebSocketMessageHandler as WsHandler};



