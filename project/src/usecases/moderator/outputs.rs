use crate::models::moderator::{ModeratorInsertionReturnType, ModeratorView};
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

/// Tipo de retorno para [`super::ModeratorUsecase::get_moderators`].
#[derive(Serialize, Deserialize)]
pub struct GetModeratorsOutput {
    moderators: Vec<ModeratorView>,
}

impl From<Vec<ModeratorView>> for GetModeratorsOutput {
    fn from(value: Vec<ModeratorView>) -> Self {
        Self { moderators: value }
    }
}

impl From<GetModeratorsOutput> for HttpResponse {
    fn from(value: GetModeratorsOutput) -> Self {
        HttpResponse::Ok().json(value)
    }
}

/// Tipo de retorno para [`super::ModeratorUsecase::post_moderator`].
pub struct PostModeratorOutput(i32);

impl TryFrom<ModeratorInsertionReturnType> for PostModeratorOutput {
    type Error = super::errors::PostModeratorError;
    fn try_from(value: ModeratorInsertionReturnType) -> Result<Self, Self::Error> {
        if value.is_ok() {
            Ok(Self(value.user_id))
        } else {
            Err(Self::Error::from(value))
        }
    }
}

impl From<PostModeratorOutput> for HttpResponse {
    fn from(value: PostModeratorOutput) -> Self {
        HttpResponse::Created().body(format!("O usuário `{}` agora é um moderador!", value.0))
    }
}
