use crate::{
    models::{
        error::ModelParseError,
        moderator::{ModeratorInsertionReturnType, ModeratorInsertionVariant, ModeratorView},
    },
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

/// Possíveis erros para [`super::ModeratorUsecase::post_moderator`].
#[derive(thiserror::Error, Debug)]
pub enum PostModeratorError {
    /// Quando o corpo da requisição não é um json válido.
    #[error("O corpo da requisição não é um json válido:\n\n{}", .0)]
    InvalidBody(String),
    /// O id fornecido é inválido.
    #[error("O id `{}` não é reconhecido como válido!", .0)]
    InvalidId(String),
    /// O usuário de id especificado não foi encontrado.
    #[error("O usuário de id `{}` não existe!", .0)]
    NotFound(i32),
    /// O usuário de id especificado já é um moderador.
    #[error("O usuário de id `{}` já é um moderador!", .0)]
    AlreadyModerator(i32),
    /// O usuário de id especificado está suspenso.
    #[error("O usuário de id `{}` está suspenso (não pode ser moderador)!", .0)]
    IsSuspended(i32),
    /// Quando o erro ocorre no banco de dados.
    #[error(transparent)]
    Repository(RepositoryError),
    /// Quando o erro ocorre durante o parsing do modelo.
    #[error(transparent)]
    ModelParse(ModelParseError<ModeratorInsertionReturnType, PgRow>),
    /// Quando o usuário é adicionado mas por algum motivo é chamada a camada de erro.
    #[error("O usuário `{}` foi adicionado com sucesso. Isso não deveria ser um erro :^(", .0)]
    Unexpected(i32),
}

impl From<PostModeratorError> for HttpResponse {
    fn from(value: PostModeratorError) -> Self {
        let s = value.to_string();
        match value {
            PostModeratorError::InvalidId(_) | PostModeratorError::InvalidBody(_) => {
                HttpResponse::BadRequest().body(s)
            }
            PostModeratorError::NotFound(_) => HttpResponse::NotFound().body(s),
            PostModeratorError::AlreadyModerator(_) | PostModeratorError::IsSuspended(_) => {
                HttpResponse::Conflict().body(s)
            }
            PostModeratorError::Repository(err) => err.into(),
            PostModeratorError::ModelParse(err) => err.into(),
            PostModeratorError::Unexpected(_) => HttpResponse::InternalServerError().body(s),
        }
    }
}

impl From<RepositoryError> for PostModeratorError {
    fn from(value: RepositoryError) -> Self {
        Self::Repository(value)
    }
}

impl From<ModelParseError<ModeratorInsertionReturnType, PgRow>> for PostModeratorError {
    fn from(value: ModelParseError<ModeratorInsertionReturnType, PgRow>) -> Self {
        Self::ModelParse(value)
    }
}

impl From<ModeratorInsertionReturnType> for PostModeratorError {
    fn from(value: ModeratorInsertionReturnType) -> Self {
        let id = value.user_id;
        match value.result {
            ModeratorInsertionVariant::NotFound => Self::NotFound(id),
            ModeratorInsertionVariant::IsSuspended => Self::IsSuspended(id),
            ModeratorInsertionVariant::AlreadyModerator => Self::AlreadyModerator(id),
            ModeratorInsertionVariant::Done => Self::Unexpected(id),
        }
    }
}
