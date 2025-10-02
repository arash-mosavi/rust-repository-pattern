use async_trait::async_trait;
use mongodb::{Client, ClientSession};
use core_db::UnitOfWork;
use pkg::{RepositoryError, RepositoryResult};

pub struct MongoUnitOfWork {
    client: Client,
    session: Option<ClientSession>,
}

impl MongoUnitOfWork {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            session: None,
        }
    }

    pub fn session(&mut self) -> Option<&mut ClientSession> {
        self.session.as_mut()
    }
}

#[async_trait]
impl UnitOfWork for MongoUnitOfWork {
    async fn begin(&mut self) -> RepositoryResult<()> {
        if self.session.is_some() {
            return Err(RepositoryError::InternalError(
                "Transaction already started".to_string(),
            ));
        }

        let session = self
            .client
            .start_session(None)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        self.session = Some(session);

        // Start transaction
        if let Some(session) = &mut self.session {
            session
                .start_transaction(None)
                .await
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        }

        Ok(())
    }

    async fn commit(&mut self) -> RepositoryResult<()> {
        if let Some(mut session) = self.session.take() {
            session
                .commit_transaction()
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
        if let Some(mut session) = self.session.take() {
            session
                .abort_transaction()
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
