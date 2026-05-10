use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Tipo que representa um [`Vec`] (lista) de usuários.
#[derive(Serialize, Deserialize)]
pub struct Users(Vec<User>);

impl FromIterator<User> for Users {
    fn from_iter<T: IntoIterator<Item = User>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

/// Tipo que representa um usuário.
#[derive(Serialize, Deserialize)]
pub struct User {
    /// Id do usuário.
    id: i32,
    /// Nome do usuário.
    name: Box<str>,
    /// E-mail do usuário.
    mail: UserMail,
    /// Última data de modificação.
    latest_modify: SystemTime,
    /// Status do usuário.
    status: UserStatus,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: 0,
            name: "Nome Fantasia".into(),
            mail: UserMail {
                local: "fantasia".into(),
                dominium: "mail.com".into(),
            },
            latest_modify: SystemTime::now(),
            status: UserStatus::default(),
        }
    }
}

/// Estrutura capaz de representar o e-mail de um usuário.
#[derive(Serialize, Deserialize)]
pub struct UserMail {
    /// Parte local do e-mail.
    local: Box<str>,
    /// Domínio do e-mail.
    dominium: Box<str>,
}

/// Status de usuário.
#[derive(Default, Serialize, Deserialize)]
pub enum UserStatus {
    /// Disponível.
    #[default]
    Available,
    /// Suspenso.
    Suspended,
}
