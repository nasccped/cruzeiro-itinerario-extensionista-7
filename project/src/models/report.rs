use super::generic_date_time::GenericDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, prelude::Type};

/// View para o report registrado no banco de dados.
#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct ReportView {
    id: i32,
    owner: String,
    #[sqlx(rename = "timestamp")]
    open_at: GenericDateTime,
    record: i32,
    status: ReportStatus,
}

/// Possíveis status  para os reports.
#[derive(Serialize, Deserialize, Type, Debug)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase", type_name = "report_status")]
enum ReportStatus {
    Open,
    Suspended,
    Canceled,
}
