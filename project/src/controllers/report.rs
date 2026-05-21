use super::_utils as utils;
use actix_web::HttpResponse;

const REPORTS_ENDPOINT: &str = "/reports";

/// Controller para os reports.
pub struct ReportController {}

impl ReportController {
    /// Retorna a url de [`ReportController::get_reports`].
    pub fn get_reports_endpoint() -> &'static str {
        REPORTS_ENDPOINT
    }

    pub async fn get_reports() -> HttpResponse {
        let err: Result<HttpResponse, HttpResponse> =
            Err(HttpResponse::NotImplemented().body("Operação ainda não implementada"));
        utils::log_and_normalize(err, Self::get_reports_endpoint())
    }
}
