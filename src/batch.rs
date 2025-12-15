//! Batch QR code generation from CSV/JSON files

use crate::cli::OutputFormat;
use crate::error::{QrError, Result};
use crate::renderer::{render_to_png, render_to_svg_file, RenderConfig};
use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

/// Record from batch input file
#[derive(Debug, Deserialize)]
pub struct BatchRecord {
    /// Filename for the output (without extension)
    pub filename: String,
    /// Data to encode in QR code
    pub data: String,
}

/// Process a batch file and generate QR codes
pub fn process_batch(
    input_path: &PathBuf,
    output_dir: &PathBuf,
    format: &OutputFormat,
    config: &RenderConfig,
) -> Result<usize> {
    // Create output directory if it doesn't exist
    fs::create_dir_all(output_dir).map_err(|e| QrError::FileWrite {
        path: output_dir.clone(),
        source: e,
    })?;

    // Read records from file
    let records = read_batch_file(input_path)?;
    let total = records.len();

    // Setup progress bar
    let pb = ProgressBar::new(total as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("█▓░"),
    );

    // Generate QR codes
    for record in &records {
        let extension = match format {
            OutputFormat::Png | OutputFormat::Terminal | OutputFormat::Base64 => "png",
            OutputFormat::Svg => "svg",
        };

        let output_path = output_dir.join(format!("{}.{}", record.filename, extension));

        match format {
            OutputFormat::Png | OutputFormat::Terminal | OutputFormat::Base64 => {
                render_to_png(&record.data, &output_path, config)?;
            }
            OutputFormat::Svg => {
                render_to_svg_file(&record.data, &output_path, config)?;
            }
        }

        pb.inc(1);
    }

    pb.finish_with_message("Done!");
    Ok(total)
}

/// Read batch records from CSV or JSON file
fn read_batch_file(path: &PathBuf) -> Result<Vec<BatchRecord>> {
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "csv" => read_csv(path),
        "json" => read_json(path),
        _ => Err(QrError::CsvError(
            "Unsupported file format. Use .csv or .json".to_string(),
        )),
    }
}

/// Read records from CSV file
fn read_csv(path: &PathBuf) -> Result<Vec<BatchRecord>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut records = Vec::new();

    for result in reader.deserialize() {
        let record: BatchRecord = result?;
        records.push(record);
    }

    Ok(records)
}

/// Read records from JSON file
fn read_json(path: &PathBuf) -> Result<Vec<BatchRecord>> {
    let content = fs::read_to_string(path).map_err(|e| QrError::FileRead {
        path: path.clone(),
        source: e,
    })?;

    let records: Vec<BatchRecord> = serde_json::from_str(&content)?;
    Ok(records)
}
