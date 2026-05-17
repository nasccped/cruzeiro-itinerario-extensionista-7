use crate::models::moderator::ModeratorView;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

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
