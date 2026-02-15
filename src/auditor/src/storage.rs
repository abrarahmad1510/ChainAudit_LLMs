use sqlx::{PgPool, postgres::PgPoolOptions};
use anyhow::Result;

pub struct Storage {
    pool: PgPool,
}

pub struct ReceiptRecord {
    pub leaf_hash: Vec<u8>,
    pub leaf_index: i64,
    pub root_hash: Vec<u8>,
    pub context: serde_json::Value,
    pub receipt_jwt: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Storage {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await?;
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await?;
        Ok(Self { pool })
    }

    pub async fn store_receipt(
        &self,
        leaf_hash: &[u8],
        leaf_index: i64,
        root_hash: &[u8],
        metadata: &[u8],
        receipt_jwt: &str,
    ) -> Result<()> {
        let context: serde_json::Value = serde_json::from_slice(metadata)?;
        sqlx::query!(
            r#"
            INSERT INTO receipts (leaf_hash, leaf_index, root_hash, context, receipt_jwt, created_at)
            VALUES ($1, $2, $3, $4, $5, NOW())
            "#,
            leaf_hash,
            leaf_index,
            root_hash,
            context,
            receipt_jwt
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_receipt(&self, leaf_hash: &[u8]) -> Result<ReceiptRecord> {
        let row = sqlx::query!(
            r#"
            SELECT leaf_hash, leaf_index, root_hash, context, receipt_jwt, created_at
            FROM receipts
            WHERE leaf_hash = $1
            "#,
            leaf_hash
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(ReceiptRecord {
            leaf_hash: row.leaf_hash,
            leaf_index: row.leaf_index,
            root_hash: row.root_hash,
            context: row.context,
            receipt_jwt: row.receipt_jwt,
            created_at: row.created_at,
        })
    }
}
