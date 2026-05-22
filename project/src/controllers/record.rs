use super::_utils as utils;
use actix_web::HttpResponse;

/// Controller para os records.
pub struct RecordController {}

impl RecordController {
    /// Retorna a url de [`RecordController::get_records`].
    pub fn get_records_endpoint() -> &'static str {
        "/records"
    }

    /// Obtém a lista de records registrada no banco de dados.
    pub async fn get_records() -> HttpResponse {
        let err: Result<HttpResponse, HttpResponse> =
            Err(HttpResponse::NotImplemented().body("Funcionalidade ainda não implementada!"));
        utils::log_and_normalize(err, Self::get_records_endpoint())
    }
}
