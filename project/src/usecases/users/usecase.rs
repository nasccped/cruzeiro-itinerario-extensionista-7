use super::{GetUserByIdError, GetUserByIdOutput, GetUsersError, GetUsersOutput, PostUserOutput};
use crate::{
    models::{
        error::ModelParseError,
        user::{CreateUserModel, User},
    },
    repositories::{UserRepository, error::RepositoryError},
    usecases::users::errors::PostUserError,
};
use sqlx::{
    FromRow,
    postgres::{PgPool, PgRow},
};

/// Casos de uso para as operações com os usuários.
pub struct UserUsecase {
    repository: UserRepository,
}

impl UserUsecase {
    /// Cria uma nova instância de [`UserUsecase`] a partir de uma conexão postgres já
    /// estabelecida.
    pub fn new(conn: PgPool) -> Self {
        let repository = UserRepository::from(conn);
        Self { repository }
    }

    /// Retorna uma lista com todos os usuários.
    pub async fn get_users(&self) -> Result<GetUsersOutput, GetUsersError> {
        let result = try_get_users(&self.repository).await?;
        let mut users = Vec::new();
        for row in result {
            let user = try_row_to_user(row)?;
            users.push(user);
        }
        Ok(GetUsersOutput::from(users))
    }

    /// Retorna o usuário especificado pelo id.
    pub async fn get_user_by_id(&self, id: &str) -> Result<GetUserByIdOutput, GetUserByIdError> {
        let id = id
            .parse()
            .map_err(|_| GetUserByIdError::InvalidId(id.to_string()))?;
        let row = try_get_user_by_id(&self.repository, id)
            .await?
            .ok_or(GetUserByIdError::NotFound(id))?;
        try_row_to_user(row)
            .map(GetUserByIdOutput::from)
            .map_err(GetUserByIdError::from)
    }

    /// Adiciona um novo usuário ao banco de dados.
    pub async fn post_user(&self, body: String) -> Result<PostUserOutput, PostUserError> {
        let invalid_name = |name: &str| PostUserError::InvalidName(name.to_string());
        let invalid_mail = |mail: &str| PostUserError::InvalidMail(mail.to_string());
        let model: CreateUserModel =
            serde_json::from_str(&body).map_err(|_| PostUserError::InvalidBody(body))?;
        if !model.name_is_valid() {
            return Err(invalid_name(model.name()));
        } else if !model.email_is_valid() {
            return Err(invalid_mail(model.mail()));
        }
        let name = model.name().to_string();
        let mail = model.mail().to_string();
        post_user_or_err(&self.repository, model)
            .await
            .map(|_| PostUserOutput)
            .map_err(|e| {
                PostUserError::from(e)
                    .with_name_violation(name)
                    .with_mail_violation(mail)
            })
    }
}

type TryGetUserOutput = Result<Vec<PgRow>, RepositoryError>;
type TryGetUserByIdOutput = Result<Option<PgRow>, RepositoryError>;
type TryPostUserOutput = Result<(), RepositoryError>;
type TryRowToUserOutput = Result<User, ModelParseError<User, PgRow>>;

/// Tenta obter os usuários do banco de dados.
async fn try_get_users(repo: &UserRepository) -> TryGetUserOutput {
    repo.get_users().await
}

/// Tenta obter um usuário (especificado pelo id) do banco de dados.
async fn try_get_user_by_id(repo: &UserRepository, id: i64) -> TryGetUserByIdOutput {
    repo.get_user_by_id(id).await
}

/// Tenta converter uma [`PgRow`] para [`User`].
fn try_row_to_user(row: PgRow) -> TryRowToUserOutput {
    User::from_row(&row).map_err(|e| ModelParseError::from_err_and_input(e, row))
}

async fn post_user_or_err(repo: &UserRepository, model: CreateUserModel) -> TryPostUserOutput {
    repo.post_user(model).await
}
