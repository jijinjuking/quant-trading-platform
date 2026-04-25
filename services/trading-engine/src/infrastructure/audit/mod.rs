//! Audit infrastructure adapters.

pub mod postgres_audit;

pub use postgres_audit::PostgresTradeAuditAdapter;
