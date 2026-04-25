//! Redis cache adapter.

use redis::Commands;

use crate::domain::port::cache_port::CachePort;

pub struct RedisCache {
    client: redis::Client,
    prefix: String,
}

impl RedisCache {
    pub fn new(url: String) -> anyhow::Result<Self> {
        let client = redis::Client::open(url)?;
        Ok(Self {
            client,
            prefix: "gateway".to_string(),
        })
    }

    fn key(&self, raw_key: &str) -> String {
        format!("{}:{}", self.prefix, raw_key)
    }
}

impl CachePort for RedisCache {
    fn get(&self, key: &str) -> Option<String> {
        let full_key = self.key(key);
        let mut conn = self.client.get_connection().ok()?;
        conn.get(full_key).ok()
    }

    fn set(&self, key: &str, value: &str, ttl_seconds: u64) -> bool {
        let full_key = self.key(key);
        let mut conn = match self.client.get_connection() {
            Ok(c) => c,
            Err(_) => return false,
        };

        let set_ok: redis::RedisResult<()> = conn.set_ex(full_key, value, ttl_seconds as u64);
        set_ok.is_ok()
    }

    fn delete(&self, key: &str) -> bool {
        let full_key = self.key(key);
        let mut conn = match self.client.get_connection() {
            Ok(c) => c,
            Err(_) => return false,
        };

        let deleted: redis::RedisResult<u64> = conn.del(full_key);
        deleted.map(|n| n > 0).unwrap_or(false)
    }

    fn check_rate_limit(&self, key: &str, max_requests: u32) -> bool {
        let full_key = self.key(key);
        let mut conn = match self.client.get_connection() {
            Ok(c) => c,
            Err(_) => return false,
        };

        let count: i64 = match conn.incr(&full_key, 1_i64) {
            Ok(v) => v,
            Err(_) => return false,
        };

        if count == 1 {
            let _: redis::RedisResult<bool> = conn.expire(&full_key, 60_i64);
        }

        count <= i64::from(max_requests)
    }
}
