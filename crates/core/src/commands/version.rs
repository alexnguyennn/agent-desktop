use crate::AppError;
use serde_json::{Value, json};

pub fn execute() -> Result<Value, AppError> {
    Ok(json!({
        "version": crate::BUILD_VERSION,
        "target": std::env::consts::ARCH,
        "os": std::env::consts::OS,
    }))
}
