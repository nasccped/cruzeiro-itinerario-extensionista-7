use super::error::RepositoryError;
use sqlx::{PgPool, postgres::PgRow};

const SELECT_FROM_REPORT_VIEW_QUERY: &str =
    include_str!("../../db-templates/callables/select-from-reports-view.sql");

/// Repository para as operações com os reports.
pub struct ReportRepository {
    conn: PgPool,
}

impl From<PgPool> for ReportRepository {
    fn from(value: PgPool) -> Self {
        Self { conn: value }
    }
}

impl ReportRepository {
    /// Retorna os report views do banco de dados.
    pub async fn get_reports_view(&self) -> Result<Vec<PgRow>, RepositoryError> {
        sqlx::query(SELECT_FROM_REPORT_VIEW_QUERY)
            .fetch_all(&self.conn)
            .await
            .map_err(|err| RepositoryError::from_err_and_query(err, SELECT_FROM_REPORT_VIEW_QUERY))
    }
}
