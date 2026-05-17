use super::_utils as utils;
use crate::usecases::moderator::ModeratorUsecase;
use actix_web::HttpResponse;
use actix_web::web;

const MODERATORS_ENDPOINT: &str = "/moderators";

pub struct ModeratorController {}

impl ModeratorController {
    /// Retorna o endpoint para [`ModeratorController::get_moderators`].
    pub fn get_moderators_endpoint() -> &'static str {
        MODERATORS_ENDPOINT
    }

    /// Retorna o endpoint para [`ModeratorController::post_moderator`].
    pub fn post_moderator_endpoint() -> &'static str {
        MODERATORS_ENDPOINT
    }

    /// Retorna uma lista contendo todos os moderadores.
    pub async fn get_moderators(usecase: web::Data<ModeratorUsecase>) -> HttpResponse {
        utils::log_and_normalize(
            usecase.get_moderators().await,
            Self::get_moderators_endpoint(),
        )
    }

    /// Tenta inserir um novo moderador na tabela de moderadores.
    pub async fn post_moderator(
        usecase: web::Data<ModeratorUsecase>,
        body: String,
    ) -> HttpResponse {
        utils::log_and_normalize(
            usecase.post_moderator(body).await,
            Self::post_moderator_endpoint(),
        )
    }
}
