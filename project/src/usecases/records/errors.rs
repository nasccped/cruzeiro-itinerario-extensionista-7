use crate::{
    models::{error::ModelParseError, record::RecordView},
    repositories::error::RepositoryError,
};
use actix_web::HttpResponse;
use sqlx::postgres::PgRow;

/// Possíveis erros para [`super::RecordUsecase::get_records`].
#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub enum GetRecordsError {
    /// Falha no banco de dados.
    Repository(RepositoryError),
    /// Falha no parsingo do [`RecordView`].
    ModelParse(ModelParseError<RecordView, PgRow>),
}

impl From<GetRecordsError> for HttpResponse {
    fn from(value: GetRecordsError) -> Self {
        match value {
            GetRecordsError::Repository(err) => err.into(),
            GetRecordsError::ModelParse(err) => err.into(),
        }
    }
}

impl From<RepositoryError> for GetRecordsError {
    fn from(value: RepositoryError) -> Self {
        Self::Repository(value)
    }
}

impl From<ModelParseError<RecordView, PgRow>> for GetRecordsError {
    fn from(value: ModelParseError<RecordView, PgRow>) -> Self {
        Self::ModelParse(value)
    }
}
