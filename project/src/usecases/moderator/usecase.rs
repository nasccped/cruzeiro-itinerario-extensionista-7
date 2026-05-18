use super::{
    errors::{DeleteModeratorError, GetModeratorsError, PostModeratorError},
    outputs::{DeleteModeratorOutput, GetModeratorsOutput, PostModeratorOutput},
};
use crate::{
    models::{
        error::ModelParseError,
        moderator::{
            ModeratorDeletionReturnType, ModeratorInsertionReturnType, ModeratorView,
            PostModeratorBody,
        },
    },
    repositories::{ModeratorRepository, error::RepositoryError},
};
use sqlx::{FromRow, PgPool, postgres::PgRow};

pub struct ModeratorUsecase {
    repo: ModeratorRepository,
}

impl ModeratorUsecase {
    /// Cria um novo [`ModeratorUsecase`] a partir de uma [`PgPool`].
    pub fn new(conn: PgPool) -> Self {
        let repo = ModeratorRepository::new(conn);
        Self { repo }
    }

    /// Aciona a conexão do [`PgPool`] e obtém o [`GetModeratorsOutput`]. Caso falhe, retorna
    /// [`GetModeratorsError`].
    pub async fn get_moderators(&self) -> Result<GetModeratorsOutput, GetModeratorsError> {
        let rows = try_get_moderators(&self.repo).await?;
        let mut moderators = Vec::new();
        for row in rows {
            let moderator = try_row_to_moderator_view(row)?;
            moderators.push(moderator);
        }
        Ok(GetModeratorsOutput::from(moderators))
    }

    /// Aciona a conexão do [`PgPool`] e obtém o [`PostModeratorOutput`]. Caso falhe, retorna
    /// [`PostModeratorError`].
    pub async fn post_moderator(
        &self,
        body: String,
    ) -> Result<PostModeratorOutput, PostModeratorError> {
        let body: PostModeratorBody = serde_json::from_str(&body)
            .map_err(|_| PostModeratorError::InvalidBody(body.clone()))?;
        let row = try_post_moderator(&self.repo, body.user_id).await?;
        let result = ModeratorInsertionReturnType::from_row(&row)
            .map_err(|e| ModelParseError::from_err_and_input(e, row))?;
        PostModeratorOutput::try_from(result)
    }

    /// Aciona a conexão do [`PgPool`] e remove o moderador especificado pelo `user_id`.
    pub async fn delete_moderator(
        &self,
        user_id: String,
    ) -> Result<DeleteModeratorOutput, DeleteModeratorError> {
        let id = user_id
            .parse()
            .map_err(|_| DeleteModeratorError::InvalidId(user_id))?;
        let result = self.repo.delete_moderator(id).await.map(|row| {
            ModeratorDeletionReturnType::from_row(&row)
                .map_err(|e| ModelParseError::from_err_and_input(e, row))
        })?;
        let model = result?;
        DeleteModeratorOutput::try_from(model)
    }
}

type TryGetModeratorsOutput = Result<Vec<PgRow>, RepositoryError>;
type TryRowToModeratorView = Result<ModeratorView, ModelParseError<ModeratorView, PgRow>>;
type TryPostModeratorOutput = Result<PgRow, RepositoryError>;

async fn try_get_moderators(repo: &ModeratorRepository) -> TryGetModeratorsOutput {
    repo.get_moderators_view().await
}

async fn try_post_moderator(repo: &ModeratorRepository, id: i32) -> TryPostModeratorOutput {
    repo.post_moderator(id).await
}

fn try_row_to_moderator_view(row: PgRow) -> TryRowToModeratorView {
    ModeratorView::from_row(&row).map_err(|e| ModelParseError::from_err_and_input(e, row))
}
