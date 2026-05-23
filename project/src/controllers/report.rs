use super::_utils as utils;
use crate::{not_implemented, usecases::reports::ReportsUsecase};
use actix_web::{HttpResponse, web};

const REPORTS_ENDPOINT: &str = "/reports";
const REPORT_BY_ID_ENDPOINT: &str = "/reports/{reportId}";

/// Controller para os reports.
pub struct ReportController {}

impl ReportController {
    /// Retorna a url de [`ReportController::get_reports`].
    pub fn get_reports_endpoint() -> &'static str {
        REPORTS_ENDPOINT
    }

    /// Retorna a url de [`ReportController::get_report_by_id`].
    pub fn get_report_by_id_endpoint() -> &'static str {
        REPORT_BY_ID_ENDPOINT
    }

    /// Retorna a url de [`ReportController::post_report`].
    pub fn post_report_endpoint() -> &'static str {
        REPORTS_ENDPOINT
    }

    /// Retorna um json content os `report_view`s.
    pub async fn get_reports(usecase: web::Data<ReportsUsecase>) -> HttpResponse {
        utils::log_and_normalize(usecase.get_reports().await, Self::get_reports_endpoint())
    }

    /// Retorn o report associado ao `reportId`.
    pub async fn get_report_by_id(_report_id: web::Path<String>) -> HttpResponse {
        utils::log_and_normalize(not_implemented!(), Self::get_report_by_id_endpoint())
    }

    /// Public um novo report no banco de dados.
    pub async fn post_report(usecase: web::Data<ReportsUsecase>) -> HttpResponse {
        utils::log_and_normalize(usecase.post_report().await, Self::get_reports_endpoint())
    }
}
