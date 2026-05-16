#![allow(clippy::from_over_into)]
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, prelude::Type};
use time::OffsetDateTime;

/// Tipo que representa um [`Vec`] (lista) de usuários.
#[derive(Serialize, Deserialize)]
pub struct Users {
    users: Vec<User>,
}

impl Users {
    /// Cria uma entidade [`Users`] vazia.
    pub fn empty() -> Self {
        Self { users: Vec::new() }
    }

    /// Adiciona um novo usuário ao final do vetor de usuários.
    pub fn push(&mut self, value: User) {
        self.users.push(value);
    }
}

impl FromIterator<User> for Users {
    fn from_iter<T: IntoIterator<Item = User>>(iter: T) -> Self {
        Self {
            users: Vec::from_iter(iter),
        }
    }
}

impl Into<HttpResponse> for Users {
    fn into(self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

/// Tipo que representa um usuário.
#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    /// Id do usuário.
    id: i32,
    /// Nome do usuário.
    user_name: Box<str>,
    /// E-mail do usuário.
    user_mail: Box<str>,
    /// Última data de modificação.
    latest_change: Option<OffsetDateTime>,
    /// Status do usuário.
    #[sqlx(rename = "current_status")]
    status: UserStatus,
}

impl Into<HttpResponse> for User {
    fn into(self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

/// Status de usuário.
#[repr(i32)]
#[derive(Default, Serialize, Deserialize, Type)]
#[sqlx(type_name = "INT4")]
pub enum UserStatus {
    /// Disponível.
    #[default]
    Available = 1,
    /// Suspenso.
    Suspended = 2,
}
