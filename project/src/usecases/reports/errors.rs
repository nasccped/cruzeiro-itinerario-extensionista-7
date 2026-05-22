use crate::{
    models::{error::ModelParseError, report::ReportView},
    repositories::error::RepositoryError,
};
use actix_web::HttpResponse;
use sqlx::postgres::PgRow;

/// Possíveis erros para [`super::usecase::ReportsUsecase::get_reports`].
#[derive(thiserror::Error, Debug)]
pub enum GetReportsError {
    #[error(transparent)]
    Repository(RepositoryError),
    #[error(transparent)]
    ModelParse(ModelParseError<ReportView, PgRow>),
}

impl From<GetReportsError> for HttpResponse {
    fn from(value: GetReportsError) -> Self {
        match value {
            GetReportsError::Repository(err) => err.into(),
            GetReportsError::ModelParse(err) => err.into(),
        }
    }
}

impl From<RepositoryError> for GetReportsError {
    fn from(value: RepositoryError) -> Self {
        Self::Repository(value)
    }
}

impl From<ModelParseError<ReportView, PgRow>> for GetReportsError {
    fn from(value: ModelParseError<ReportView, PgRow>) -> Self {
        Self::ModelParse(value)
    }
}
