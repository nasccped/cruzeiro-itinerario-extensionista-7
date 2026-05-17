use super::generic_date_time::GenericDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

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
