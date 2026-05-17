use crate::models::user::User;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

/// Tipo de retorno para quando [`super::UserUsecase::get_users`] opera com sucesso.
#[derive(Serialize, Deserialize)]
pub struct GetUsersOutput {
    users: Vec<User>,
}

impl From<Vec<User>> for GetUsersOutput {
    fn from(value: Vec<User>) -> Self {
        Self { users: value }
    }
}

impl From<GetUsersOutput> for HttpResponse {
    fn from(val: GetUsersOutput) -> Self {
        HttpResponse::Ok().json(val)
    }
}

/// Tipo de retorno para quando [`super::UserUsecase::get_user_by_id`] opera com sucesso.
#[derive(Serialize, Deserialize)]
pub struct GetUserByIdOutput {
    user: User,
}

impl From<User> for GetUserByIdOutput {
    fn from(value: User) -> Self {
        Self { user: value }
    }
}

impl From<GetUserByIdOutput> for HttpResponse {
    fn from(val: GetUserByIdOutput) -> Self {
        HttpResponse::Ok().json(val)
    }
}

/// Tipo de retorno para quando [`super::UserUsecase::post_user`] opera com sucesso.
pub struct PostUserOutput;

impl From<PostUserOutput> for HttpResponse {
    #[allow(unused_variables)]
    fn from(value: PostUserOutput) -> Self {
        HttpResponse::Ok().body("Usuário adicionado com sucesso!")
    }
}
