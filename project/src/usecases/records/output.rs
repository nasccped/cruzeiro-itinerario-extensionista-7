use crate::models::record::RecordView;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

/// Output [`Ok`] retornado por [`super::RecordUsecase::get_records`].
#[derive(Serialize, Deserialize)]
pub struct GetRecordsOutput {
    records: Vec<RecordView>,
}

impl From<GetRecordsOutput> for HttpResponse {
    fn from(value: GetRecordsOutput) -> Self {
        Self::Ok().json(value)
    }
}

impl From<Vec<RecordView>> for GetRecordsOutput {
    fn from(value: Vec<RecordView>) -> Self {
        Self { records: value }
    }
}
