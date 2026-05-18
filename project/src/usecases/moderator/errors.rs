use crate::{
    models::{
        error::ModelParseError,
        moderator::{
            ModeratorDeletionReturnType, ModeratorDeletionVariant, ModeratorInsertionReturnType,
            ModeratorInsertionVariant, ModeratorView,
        },
    },
    repositories::error::RepositoryError,
};
use actix_web::HttpResponse;
use sqlx::postgres::PgRow;

/// Possíveis erros para [`super::ModeratorUsecase::get_moderators`].
pub struct GetModeratorsError(CommonModeratorsError<ModeratorView>);

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
#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub enum CommonModeratorsError<T: std::fmt::Debug + 'static> {
    /// Falha em parsear o model.
    ModelParse(ModelParseError<T, PgRow>),
    /// Erro no banco de dados.
    Repository(RepositoryError),
}

impl<T: std::fmt::Debug> From<CommonModeratorsError<T>> for HttpResponse {
    fn from(value: CommonModeratorsError<T>) -> Self {
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
    /// Erros comuns para operações com moderadores.
    #[error(transparent)]
    Common(CommonModeratorsError<ModeratorInsertionReturnType>),
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
            PostModeratorError::Common(err) => err.into(),
            PostModeratorError::Unexpected(_) => HttpResponse::InternalServerError().body(s),
        }
    }
}

impl From<RepositoryError> for PostModeratorError {
    fn from(value: RepositoryError) -> Self {
        Self::Common(CommonModeratorsError::Repository(value))
    }
}

impl From<ModelParseError<ModeratorInsertionReturnType, PgRow>> for PostModeratorError {
    fn from(value: ModelParseError<ModeratorInsertionReturnType, PgRow>) -> Self {
        Self::Common(CommonModeratorsError::ModelParse(value))
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

/// Possíveis erros para [`super::ModeratorUsecase::delete_moderator`].
#[derive(thiserror::Error, Debug)]
pub enum DeleteModeratorError {
    #[error("O user id fornecido (`{}`) não é reconhecido como válido!", .0)]
    InvalidId(String),
    /// Erro comum.
    #[error(transparent)]
    Common(CommonModeratorsError<ModeratorDeletionReturnType>),
    /// Usuário de id especificado não existe.
    #[error("O usuário de id `{}` não foi encontrado!", .0)]
    NotFound(i32),
    /// Usuário de id especificado existe, mas não é moderador.
    #[error("O usuário de id `{}` existe, mas não é um moderador!", .0)]
    NotAModerator(i32),
    #[error("O usuário `{}` foi remove com sucesso. Isso não deveria ser um erro :^(", .0)]
    Unexpected(i32),
}

impl From<DeleteModeratorError> for HttpResponse {
    fn from(value: DeleteModeratorError) -> Self {
        let s = value.to_string();
        match value {
            DeleteModeratorError::InvalidId(_) => Self::BadRequest().body(s),
            DeleteModeratorError::NotFound(_) => Self::NotFound().body(s),
            DeleteModeratorError::Common(err) => err.into(),
            DeleteModeratorError::NotAModerator(_) => Self::Conflict().body(s),
            DeleteModeratorError::Unexpected(_) => Self::InternalServerError().body(s),
        }
    }
}

impl From<RepositoryError> for DeleteModeratorError {
    fn from(value: RepositoryError) -> Self {
        Self::Common(CommonModeratorsError::Repository(value))
    }
}

impl From<ModelParseError<ModeratorDeletionReturnType, PgRow>> for DeleteModeratorError {
    fn from(value: ModelParseError<ModeratorDeletionReturnType, PgRow>) -> Self {
        Self::Common(CommonModeratorsError::ModelParse(value))
    }
}

impl From<ModeratorDeletionReturnType> for DeleteModeratorError {
    fn from(value: ModeratorDeletionReturnType) -> Self {
        let id = value.user_id;
        match value.result {
            ModeratorDeletionVariant::NotFound => Self::NotFound(id),
            ModeratorDeletionVariant::NotAModerator => Self::NotAModerator(id),
            ModeratorDeletionVariant::Done => Self::Unexpected(id),
        }
    }
}
