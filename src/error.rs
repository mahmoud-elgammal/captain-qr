//! Custom error types for cqr

use std::path::PathBuf;
use thiserror::Error;

/// Result type alias for cqr operations
pub type Result<T> = std::result::Result<T, QrError>;

/// Custom error types with helpful messages
#[derive(Error, Debug)]
pub enum QrError {
    #[error("Failed to generate QR code: {0}")]
    QrGeneration(String),

    #[error("Failed to encode data: data too large for QR code")]
    DataTooLarge,

    #[error("Failed to write to file '{path}': {source}")]
    FileWrite {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to read file '{path}': {source}")]
    FileRead {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Invalid color format '{0}'. Use hex format like #RRGGBB or #RGB")]
    InvalidColor(String),

    #[error("Failed to decode QR code from image: {0}")]
    DecodeError(String),

    #[error("No QR code found in image")]
    NoQrCodeFound,

    #[error("Failed to parse CSV: {0}")]
    CsvError(String),

    #[error("Failed to parse JSON: {0}")]
    JsonError(String),

    #[error("Invalid date/time format: {0}")]
    InvalidDateTime(String),

    #[error("Image processing error: {0}")]
    ImageError(String),

    #[error("Interactive mode error: {0}")]
    InteractiveError(String),
}

impl From<qrcode::types::QrError> for QrError {
    fn from(err: qrcode::types::QrError) -> Self {
        QrError::QrGeneration(err.to_string())
    }
}

impl From<image::ImageError> for QrError {
    fn from(err: image::ImageError) -> Self {
        QrError::ImageError(err.to_string())
    }
}

impl From<csv::Error> for QrError {
    fn from(err: csv::Error) -> Self {
        QrError::CsvError(err.to_string())
    }
}

impl From<serde_json::Error> for QrError {
    fn from(err: serde_json::Error) -> Self {
        QrError::JsonError(err.to_string())
    }
}

impl From<dialoguer::Error> for QrError {
    fn from(err: dialoguer::Error) -> Self {
        QrError::InteractiveError(err.to_string())
    }
}
