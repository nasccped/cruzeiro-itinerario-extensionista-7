#![allow(clippy::from_over_into)]
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, prelude::Type};
use time::OffsetDateTime;

/// Tipo que representa um usuário.
#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    /// Id do usuário.
    id: i32,
    /// Nome do usuário.
    user_name: String,
    /// E-mail do usuário.
    user_mail: String,
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

/// Model para criação de um novo usuário no banco de dados (mesmo de [`User`] mas sem
/// [`User::id`], [`User::latest_change`] e [`User::status`]).
#[derive(Serialize, Debug)]
pub struct CreateUserModel {
    user_name: String,
    user_mail: String,
}

impl<'de> Deserialize<'de> for CreateUserModel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct TempModel {
            user_name: String,
            user_mail: String,
        }
        let temp = TempModel::deserialize(deserializer)?;
        let user_name = normalize_str_field(temp.user_name);
        let user_mail = normalize_str_field(temp.user_mail.to_lowercase());
        Ok(Self {
            user_name,
            user_mail,
        })
    }
}

impl CreateUserModel {
    /// Retorna o nome do usuário.
    pub fn name(&self) -> &str {
        &self.user_name
    }

    /// Retorna o e-mail do usuário.
    pub fn mail(&self) -> &str {
        &self.user_mail
    }

    /// Retorna se o nome do user model é válido.
    pub fn name_is_valid(&self) -> bool {
        !self.user_name.is_empty() && self.user_name.chars().any(|c| c.is_ascii_alphabetic())
    }

    /// Retorna se o e-mail do user model é válido.
    pub fn email_is_valid(&self) -> bool {
        let mut parts = self.user_mail.split("@");
        let mut current_part: &str;
        match parts.next() {
            Some(part) => current_part = part,
            None => return false,
        }
        if current_part
            .chars()
            .any(|c| c != '.' && !c.is_ascii_alphanumeric())
        {
            return false;
        }
        match parts.next() {
            Some(part) => current_part = part,
            None => return false,
        }
        let len = current_part.len();
        if current_part.chars().enumerate().any(|(i, c)| {
            (i == 0 && !c.is_ascii_alphabetic())
                || (i == len - 1 && !c.is_ascii_alphabetic())
                || (c != '.' && !c.is_alphanumeric())
        }) {
            return false;
        }
        parts.next().is_none()
    }
}

/// Status de usuário.
#[repr(i32)]
#[derive(Default, Serialize, Deserialize, Type, Debug)]
#[sqlx(type_name = "INT4")]
pub enum UserStatus {
    /// Disponível.
    #[default]
    Available = 1,
    /// Suspenso.
    Suspended = 2,
}

/// Normaliza os campos relacionados ao usuário.
fn normalize_str_field(s: impl AsRef<str>) -> String {
    s.as_ref().split_whitespace().collect::<Vec<_>>().join(" ")
}
