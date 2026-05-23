use crate::repositories::error::RepositoryError;
use sqlx::{PgPool, postgres::PgRow};

const SELECT_FROM_RECORD_VIEW_QUERY: &str =
    include_str!("../../db-templates/callables/select-from-record-view.sql");

/// Repository para as operações com os records.
pub struct RecordRepository {
    conn: PgPool,
}

impl From<PgPool> for RecordRepository {
    fn from(value: PgPool) -> Self {
        Self { conn: value }
    }
}

impl RecordRepository {
    /// Obtém os record views registrados no banco de dados.
    pub async fn get_records(&self) -> Result<Vec<PgRow>, RepositoryError> {
        sqlx::query(SELECT_FROM_RECORD_VIEW_QUERY)
            .fetch_all(&self.conn)
            .await
            .map_err(|err| RepositoryError::from_err_and_query(err, SELECT_FROM_RECORD_VIEW_QUERY))
    }
}
