use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};

/// View para um record.
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct RecordView {
    /// Id do record.
    id: i32,
    /// Status do record.
    status: RecordStatus,
    /// Postal Address Code (Código de Endereço Postal - CEP).
    pac: String,
    /// Nome da rua.
    street_name: String,
    /// Bairro.
    neighborhood: String,
    /// Local (cidade - estado)
    locale: String,
    /// Número de reports em aberto.
    open_reports: i64,
    /// Número de reports cancelados.
    canceled_reports: i64,
    /// Número de reports suspensos.
    suspended_reports: i64,
}

/// Status possíveis para um record.
#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "record_status", rename_all = "lowercase")]
pub enum RecordStatus {
    /// Aberto.
    Open,
    /// Suspenso.
    Suspended,
    /// Cancelado.
    Canceled,
    /// Fechado.
    Closed,
}
