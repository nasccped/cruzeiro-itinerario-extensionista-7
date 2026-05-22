use super::_utils as utils;
use crate::usecases::reports::ReportsUsecase;
use actix_web::{HttpResponse, web};

const REPORTS_ENDPOINT: &str = "/reports";

/// Controller para os reports.
pub struct ReportController {}

impl ReportController {
    /// Retorna a url de [`ReportController::get_reports`].
    pub fn get_reports_endpoint() -> &'static str {
        REPORTS_ENDPOINT
    }

    /// Retorna um json content os `report_view`s.
    pub async fn get_reports(usecase: web::Data<ReportsUsecase>) -> HttpResponse {
        utils::log_and_normalize(usecase.get_reports().await, Self::get_reports_endpoint())
    }
}
