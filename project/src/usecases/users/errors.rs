#![allow(private_interfaces)]
use crate::{
    models::{error::ModelParseError, user::User},
    repositories::error::RepositoryError,
};
use actix_web::HttpResponse;
use sqlx::{error::ErrorKind, postgres::PgRow};

/// Possíveis erros para [`super::UserUsecase::get_users`].
pub struct GetUsersError(CommonUserError);

impl From<RepositoryError> for GetUsersError {
    fn from(value: RepositoryError) -> Self {
        Self(CommonUserError::Repository(value))
    }
}

impl From<ModelParseError<User, PgRow>> for GetUsersError {
    fn from(value: ModelParseError<User, PgRow>) -> Self {
        Self(CommonUserError::UserParsing(value))
    }
}

impl From<GetUsersError> for HttpResponse {
    fn from(val: GetUsersError) -> Self {
        let inner = val.0;
        inner.into()
    }
}

/// Possíveis erros para [`super::UserUsecase::get_user_by_id`].
#[derive(thiserror::Error, Debug)]
pub enum GetUserByIdError {
    /// A falha foi um [`CommonUserError`].
    #[error(transparent)]
    Common(CommonUserError),
    /// Usuário de id [`i32`] não existe no banco de dados.
    #[error("Usuário de id `{}` não foi encontrado!", .0)]
    NotFound(i32),
    /// O id fornecido para busca não é reconhecido como válido.
    #[error("Id fornecido para busca (`{}`) não é válido!", .0)]
    InvalidId(String),
}

impl From<RepositoryError> for GetUserByIdError {
    fn from(value: RepositoryError) -> Self {
        Self::Common(CommonUserError::Repository(value))
    }
}

impl From<ModelParseError<User, PgRow>> for GetUserByIdError {
    fn from(value: ModelParseError<User, PgRow>) -> Self {
        Self::Common(CommonUserError::UserParsing(value))
    }
}

impl From<GetUserByIdError> for HttpResponse {
    fn from(val: GetUserByIdError) -> Self {
        let s = val.to_string();
        match val {
            GetUserByIdError::Common(err) => err.into(),
            GetUserByIdError::NotFound(_) => HttpResponse::NotFound().body(s),
            GetUserByIdError::InvalidId(_) => HttpResponse::BadRequest().body(s),
        }
    }
}

/// Possíveis erros quando o fetching do user por meio do repositório falha.
#[derive(thiserror::Error, Debug)]
#[error(transparent)]
enum CommonUserError {
    /// Quando o erro ocorre na execução da query.
    Repository(RepositoryError),
    /// Quando o erro ocorre na deserialização da [`PgRow`].
    UserParsing(ModelParseError<User, PgRow>),
}

impl From<CommonUserError> for HttpResponse {
    fn from(val: CommonUserError) -> Self {
        match val {
            CommonUserError::Repository(err) => err.into(),
            CommonUserError::UserParsing(err) => err.into(),
        }
    }
}

/// Possíveis erros para [`super::UserUsecase::post_user`].
#[derive(Debug, thiserror::Error)]
pub enum PostUserError {
    /// O corpo da requisição não é válido.
    #[error("O corpo da requisição não é valido:\n\n{}", .0)]
    InvalidBody(String),
    /// O nome fornecido não é válido.
    #[error("O nome fornecido não é valido: {}", .0)]
    InvalidName(String),
    /// O e-mail fornecido não é válido.
    #[error("O e-mail fornecido não é valido: {}", .0)]
    InvalidMail(String),
    /// Quando o erro ocorre no db.
    #[error(transparent)]
    Repository(RepositoryError),
    /// Nome já está sendo usado por outro usuário.
    #[error("O nome '{}' já está sendo usado!", .0.as_deref().unwrap_or("<INDEFINIDO>"))]
    NameUniqueViolation(Option<String>),
    /// E-mail já está sendo usado por outro usuário.
    #[error("O e-mail '{}' já está sendo usado!", .0.as_deref().unwrap_or("<INDEFINIDO>"))]
    MailUniqueViolation(Option<String>),
}

impl PostUserError {
    /// Especifica um nome caso a variante seja [`PostUserError::NameUniqueViolation`]
    pub fn with_name_violation(mut self, name: String) -> Self {
        if let Self::NameUniqueViolation(n) = &mut self {
            *n = Some(name);
        }
        self
    }

    /// Especifica um e-mail caso a variante seja [`PostUserError::MailUniqueViolation`]
    pub fn with_mail_violation(mut self, mail: String) -> Self {
        if let Self::MailUniqueViolation(m) = &mut self {
            *m = Some(mail);
        }
        self
    }
}

impl From<RepositoryError> for PostUserError {
    fn from(value: RepositoryError) -> Self {
        let as_db_err = value.error().as_database_error();
        if as_db_err.is_none() {
            return PostUserError::Repository(value);
        }
        let as_db_err = as_db_err.unwrap();
        if as_db_err.kind() == ErrorKind::UniqueViolation
            && let Some(constraint) = as_db_err.constraint()
        {
            if constraint.starts_with("users_user_name") {
                return PostUserError::NameUniqueViolation(None);
            } else if constraint.starts_with("users_user_mail") {
                return PostUserError::MailUniqueViolation(None);
            }
            log::warn!("constraint inesperado surgiu: {}", constraint);
        }
        PostUserError::Repository(value)
    }
}

impl From<PostUserError> for HttpResponse {
    fn from(val: PostUserError) -> Self {
        let s = val.to_string();
        match val {
            PostUserError::InvalidBody(_)
            | PostUserError::InvalidName(_)
            | PostUserError::InvalidMail(_) => HttpResponse::BadRequest().body(s),
            PostUserError::NameUniqueViolation(_) | PostUserError::MailUniqueViolation(_) => {
                HttpResponse::Conflict().body(s)
            }
            PostUserError::Repository(repo) => repo.into(),
        }
    }
}
