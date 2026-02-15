CREATE TABLE IF NOT EXISTS receipts (
    leaf_hash BYTEA PRIMARY KEY,
    leaf_index BIGINT NOT NULL,
    root_hash BYTEA NOT NULL,
    context JSONB NOT NULL,
    receipt_jwt TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_receipts_created_at ON receipts(created_at);
CREATE INDEX IF NOT EXISTS idx_receipts_leaf_index ON receipts(leaf_index);
