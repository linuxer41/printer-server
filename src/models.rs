use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct PrinterInfo {
    pub name: String,
    pub driver: String,
    pub is_default: bool,
}

#[derive(Deserialize)]
pub struct PrintParams {
    pub url: String,
    pub printer: Option<String>,
}