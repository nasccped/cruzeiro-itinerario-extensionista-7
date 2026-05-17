use super::generic_date_time::GenericDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};

/// Tipo que representa uma view de moderador.
#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct ModeratorView {
    /// Id do moderador.
    moderator_id: i32,
    /// Id do usuário.
    user_id: i32,
    /// Nome do usuário.
    name: String,
    /// E-mail do usuário.
    mail: String,
    /// Moderador desde:
    since: GenericDateTime,
}

/// Json esperado para [`crate::usecases::moderator::ModeratorUsecase::post_moderator`].
#[derive(Serialize, Deserialize)]
pub struct PostModeratorBody {
    /// Id do usuário em questão.
    pub user_id: i32,
}

/// Tabela retornada para a chamada sql da função `INSERT_INTO_MODERATORS`.
#[derive(Debug, FromRow)]
pub struct ModeratorInsertionReturnType {
    /// Qual variante foi retornada.
    pub result: ModeratorInsertionVariant,
    /// Id do usuário na tentativa da operação.
    pub user_id: i32,
}

impl ModeratorInsertionReturnType {
    /// Se a operação teve sucesso.
    pub fn is_ok(&self) -> bool {
        self.result.is_ok()
    }
}

#[derive(Type, Debug, PartialEq)]
#[sqlx(type_name = "moderator_insertion_variant", rename_all = "lowercase")]
pub enum ModeratorInsertionVariant {
    /// Não existe usuário com o id especificado.
    NotFound,
    /// Usuário existe mas está suspenso.
    IsSuspended,
    /// Usuário já é um moderador.
    AlreadyModerator,
    /// Operação feita com sucesso.
    Done,
}

impl ModeratorInsertionVariant {
    /// Se a variante expressa sucesso.
    fn is_ok(&self) -> bool {
        *self == ModeratorInsertionVariant::Done
    }
}
