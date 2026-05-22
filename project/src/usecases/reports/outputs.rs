use crate::models::report::ReportView;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

/// Output para [`super::usecase::ReportsUsecase::get_reports`].
#[derive(Serialize, Deserialize)]
pub struct GetReportsOutput {
    reports: Vec<ReportView>,
}

impl From<GetReportsOutput> for HttpResponse {
    fn from(value: GetReportsOutput) -> Self {
        Self::Ok().json(value)
    }
}

impl From<Vec<ReportView>> for GetReportsOutput {
    fn from(value: Vec<ReportView>) -> Self {
        Self { reports: value }
    }
}
