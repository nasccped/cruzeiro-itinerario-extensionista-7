use super::{errors::GetReportsError, outputs::GetReportsOutput};
use crate::{
    models::{error::ModelParseError, report::ReportView},
    repositories::ReportRepository,
};
use sqlx::{FromRow, PgPool};

/// Casos de uso para os reports.
pub struct ReportsUsecase {
    repository: ReportRepository,
}

impl ReportsUsecase {
    pub fn new(conn: PgPool) -> Self {
        let repository = ReportRepository::from(conn);
        Self { repository }
    }

    /// Obtém os reports do banco de dados.
    pub async fn get_reports(&self) -> Result<GetReportsOutput, GetReportsError> {
        let rows = self.repository.get_reports_view().await?;
        let mut views = Vec::new();
        for row in rows {
            let model = ReportView::from_row(&row)
                .map_err(|err| ModelParseError::from_err_and_input(err, row))?;
            views.push(model)
        }
        Ok(GetReportsOutput::from(views))
    }
}
