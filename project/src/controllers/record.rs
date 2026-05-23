use super::_utils as utils;
use crate::usecases::records::RecordUsecase;
use actix_web::{HttpResponse, web};

/// Controller para os records.
pub struct RecordController {}

impl RecordController {
    /// Retorna a url de [`RecordController::get_records`].
    pub fn get_records_endpoint() -> &'static str {
        "/records"
    }

    /// Obtém a lista de records registrada no banco de dados.
    pub async fn get_records(usecase: web::Data<RecordUsecase>) -> HttpResponse {
        utils::log_and_normalize(usecase.get_records().await, Self::get_records_endpoint())
    }
}
