use crate::{
    models::{error::ModelParseError, moderator::ModeratorView},
    repositories::error::RepositoryError,
};
use actix_web::HttpResponse;
use sqlx::postgres::PgRow;

/// Possíveis erros para [`super::ModeratorUsecase::get_moderators`].
pub struct GetModeratorsError(CommonModeratorsError);

impl From<RepositoryError> for GetModeratorsError {
    fn from(value: RepositoryError) -> Self {
        Self(CommonModeratorsError::Repository(value))
    }
}

impl From<ModelParseError<ModeratorView, PgRow>> for GetModeratorsError {
    fn from(value: ModelParseError<ModeratorView, PgRow>) -> Self {
        Self(CommonModeratorsError::ModelParse(value))
    }
}

impl From<GetModeratorsError> for HttpResponse {
    fn from(value: GetModeratorsError) -> Self {
        let inner = value.0;
        inner.into()
    }
}

/// Erros comuns para operações com moderadores.
enum CommonModeratorsError {
    /// Falha em parsear o model.
    ModelParse(ModelParseError<ModeratorView, PgRow>),
    /// Erro no banco de dados.
    Repository(RepositoryError),
}

impl From<CommonModeratorsError> for HttpResponse {
    fn from(value: CommonModeratorsError) -> Self {
        match value {
            CommonModeratorsError::ModelParse(err) => err.into(),
            CommonModeratorsError::Repository(err) => err.into(),
        }
    }
}
