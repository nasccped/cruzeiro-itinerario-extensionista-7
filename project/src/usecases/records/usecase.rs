use super::{errors::GetRecordsError, output::GetRecordsOutput};
use crate::{
    models::{error::ModelParseError, record::RecordView},
    repositories::RecordRepository,
};
use sqlx::{FromRow, PgPool};

/// Casos de uso para records.
pub struct RecordUsecase {
    repository: RecordRepository,
}

impl RecordUsecase {
    /// Cria um novo [`RecordUsecase`] a partir de uma connection pool.
    pub fn new(conn: PgPool) -> Self {
        Self {
            repository: RecordRepository::from(conn),
        }
    }

    /// Retorna os record_view's do banco de dados.
    pub async fn get_records(&self) -> Result<GetRecordsOutput, GetRecordsError> {
        let rows = self.repository.get_records().await?;
        let mut records = Vec::new();
        for row in rows {
            let model = RecordView::from_row(&row)
                .map_err(|err| ModelParseError::from_err_and_input(err, row))?;
            records.push(model);
        }
        Ok(GetRecordsOutput::from(records))
    }
}
