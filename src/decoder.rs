//! QR code decoder from image files

use crate::error::{QrError, Result};
use rxing::{
    BarcodeFormat, DecodeHintType, DecodeHintValue, Luma8LuminanceSource, LuminanceSource,
};
use std::collections::HashMap;
use std::path::PathBuf;

/// Decoded QR code result
pub struct DecodedQr {
    pub content: String,
}

/// Decode QR code from an image file or URL
pub fn decode(input: &str) -> Result<DecodedQr> {
    // Check if input is URL
    let img = if input.starts_with("http://") || input.starts_with("https://") {
        let resp = reqwest::blocking::get(input).map_err(|e| QrError::FileRead {
            path: PathBuf::from(input),
            source: std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to download URL: {e}"),
            ),
        })?;

        let bytes = resp.bytes().map_err(|e| QrError::FileRead {
            path: PathBuf::from(input),
            source: std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to read bytes: {e}"),
            ),
        })?;

        image::load_from_memory(&bytes).map_err(|e| QrError::ImageError(e.to_string()))?
    } else {
        let path = PathBuf::from(input);
        image::open(&path).map_err(|e| QrError::FileRead {
            path: path.clone(),
            source: std::io::Error::new(std::io::ErrorKind::Other, e.to_string()),
        })?
    };

    // Convert to grayscale
    let gray = img.to_luma8();
    let (width, height) = gray.dimensions();

    // Create luminance source for rxing
    let source = Luma8LuminanceSource::new(gray.into_raw(), width, height);

    // Setup decode hints for QR codes
    let mut hints = HashMap::new();
    hints.insert(
        DecodeHintType::POSSIBLE_FORMATS,
        DecodeHintValue::PossibleFormats(vec![BarcodeFormat::QR_CODE].into_iter().collect()),
    );
    hints.insert(DecodeHintType::TRY_HARDER, DecodeHintValue::TryHarder(true));

    // Get the matrix and decode
    let matrix = source.get_matrix();

    // Decode
    let result = rxing::helpers::detect_in_luma_with_hints(matrix, width, height, None, &mut hints)
        .map_err(|e| QrError::DecodeError(format!("{e:?}")))?;

    Ok(DecodedQr {
        content: result.getText().to_string(),
    })
}
