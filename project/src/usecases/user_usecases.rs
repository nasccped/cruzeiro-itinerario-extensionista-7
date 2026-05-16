#![allow(clippy::from_over_into)]
use crate::{
    models::{
        error::ModelParseError,
        users::{User, Users},
    },
    repositories::{UserRepository, error::DbOperationError},
};
use actix_web::HttpResponse;
use sqlx::{FromRow, PgPool, postgres::PgRow};

/// Casos de uso para as operações com os usuários.
pub struct UserUsecases {
    repository: UserRepository,
}

impl UserUsecases {
    /// Cria uma nova instância de [`UserUsecases`] a partir de uma conexão postgres já
    /// estabelecida.
    pub fn new(conn: PgPool) -> Self {
        let repository = UserRepository::from(conn);
        Self { repository }
    }

    /// Retorna uma lista com todos os usuários.
    pub async fn get_users(&self) -> Result<Users, GetUsersError> {
        let result = self
            .repository
            .get_users()
            .await
            .map_err(GetUsersError::DbQuery)?;
        let mut users = Users::empty();
        for row in result {
            let user = User::from_row(&row).map_err(|e| {
                GetUsersError::UserParsing(ModelParseError::from_err_and_input(e, row))
            })?;
            users.push(user);
        }
        Ok(users)
    }

    /// Retorna o usuário especificado pelo id.
    pub async fn get_user_by_id(&self, id: &str) -> Result<User, GetUserByIdError> {
        let id = id
            .parse()
            .map_err(|_| GetUserByIdError::InvalidId(id.into()))?;
        let pg_row = self
            .repository
            .get_user_by_id(id)
            .await
            .map_err(GetUserByIdError::DbQuery)?
            .ok_or(GetUserByIdError::NotFound(id))?;
        User::from_row(&pg_row).map_err(|e| {
            GetUserByIdError::UserParsing(ModelParseError::from_err_and_input(e, pg_row))
        })
    }
}

/// Possíveis erros para retorno de [`UserUsecases::get_users`].
pub enum GetUsersError {
    /// Quando o erro ocorre na execução da query.
    DbQuery(DbOperationError),
    /// Quando o erro ocorre na deserialização da [`PgRow`].
    UserParsing(ModelParseError<User, PgRow>),
}

impl Into<HttpResponse> for GetUsersError {
    fn into(self) -> HttpResponse {
        match self {
            Self::DbQuery(err) => err.into(),
            Self::UserParsing(err) => err.into(),
        }
    }
}

/// Possíveis erros para o retorno de [`UserUsecases::get_user_by_id`].
pub enum GetUserByIdError {
    /// Quando o id é inválido.
    InvalidId(String),
    /// Quando usuário de id especificado não existe.
    NotFound(i64),
    /// Quando o erro ocorre na execução da query.
    DbQuery(DbOperationError),
    /// Quando o erro ocorre na deserialização da [`PgRow`].
    UserParsing(ModelParseError<User, PgRow>),
}

impl Into<HttpResponse> for GetUserByIdError {
    fn into(self) -> HttpResponse {
        let s = self.to_string();
        match self {
            Self::InvalidId(_) => HttpResponse::BadRequest().body(s),
            Self::NotFound(_) => HttpResponse::NotFound().body(s),
            Self::DbQuery(err) => err.into(),
            Self::UserParsing(err) => err.into(),
        }
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for GetUserByIdError {
    fn to_string(&self) -> String {
        match self {
            Self::InvalidId(id) => format!("`{}` não é reconhecido como um id válido.", id),
            Self::NotFound(id) => format!("O usuário de id `{}` não foi encontrado.", id),
            Self::DbQuery(err) => err.to_string(),
            Self::UserParsing(err) => err.to_string(),
        }
    }
}
