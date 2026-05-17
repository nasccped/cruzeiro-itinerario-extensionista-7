use super::{errors::GetModeratorsError, outputs::GetModeratorsOutput};
use crate::{
    models::{error::ModelParseError, moderator::ModeratorView},
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

    /// Aciona o a conexão do [`PgPool`] e obtém o [`GetModeratorsOutput`]. Caso falhe, retorna
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
}

type TryGetModeratorsOutput = Result<Vec<PgRow>, RepositoryError>;
type TryRowToModeratorView = Result<ModeratorView, ModelParseError<ModeratorView, PgRow>>;

async fn try_get_moderators(repo: &ModeratorRepository) -> TryGetModeratorsOutput {
    repo.get_moderators_view().await
}

fn try_row_to_moderator_view(row: PgRow) -> TryRowToModeratorView {
    ModeratorView::from_row(&row).map_err(|e| ModelParseError::from_err_and_input(e, row))
}
