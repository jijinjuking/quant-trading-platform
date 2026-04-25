-- 成交去重与幂等约束
-- 1) 清理历史重复 exchange_trade_id 数据（保留最早一条）
-- 2) 为 exchange_trade_id 添加唯一约束，支持 ON CONFLICT 幂等写入

WITH ranked AS (
    SELECT
        ctid,
        ROW_NUMBER() OVER (
            PARTITION BY exchange_trade_id
            ORDER BY trade_time ASC, id ASC
        ) AS rn
    FROM trades
    WHERE exchange_trade_id IS NOT NULL
)
DELETE FROM trades t
USING ranked r
WHERE t.ctid = r.ctid
  AND r.rn > 1;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_constraint
        WHERE conname = 'trades_exchange_trade_id_unique'
    ) THEN
        ALTER TABLE trades
        ADD CONSTRAINT trades_exchange_trade_id_unique UNIQUE (exchange_trade_id);
    END IF;
END $$;
