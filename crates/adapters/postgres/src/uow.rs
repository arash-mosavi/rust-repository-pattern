use async_trait::async_trait;
use sqlx::{PgPool, Transaction, Postgres};
use core_db::UnitOfWork;
use pkg::{RepositoryError, RepositoryResult};

/// PostgreSQL Unit of Work implementation
pub struct PostgresUnitOfWork {
    pool: PgPool,
    transaction: Option<Transaction<'static, Postgres>>,
}

impl PostgresUnitOfWork {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            transaction: None,
        }
    }

    pub fn transaction(&mut self) -> Option<&mut Transaction<'static, Postgres>> {
        self.transaction.as_mut()
    }
}

#[async_trait]
impl UnitOfWork for PostgresUnitOfWork {
    async fn begin(&mut self) -> RepositoryResult<()> {
        if self.transaction.is_some() {
            return Err(RepositoryError::InternalError(
                "Transaction already started".to_string(),
            ));
        }

        let tx = self
            .pool
            .begin()
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        // SAFETY: We need to convert the transaction to 'static lifetime
        // This is safe because we manage the transaction lifetime ourselves
        let tx_static: Transaction<'static, Postgres> = unsafe {
            std::mem::transmute(tx)
        };

        self.transaction = Some(tx_static);
        Ok(())
    }

    async fn commit(&mut self) -> RepositoryResult<()> {
        if let Some(tx) = self.transaction.take() {
            tx.commit()
                .await
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
            Ok(())
        } else {
            Err(RepositoryError::InternalError(
                "No active transaction to commit".to_string(),
            ))
        }
    }

    async fn rollback(&mut self) -> RepositoryResult<()> {
        if let Some(tx) = self.transaction.take() {
            tx.rollback()
                .await
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
            Ok(())
        } else {
            Err(RepositoryError::InternalError(
                "No active transaction to rollback".to_string(),
            ))
        }
    }
}
