use cqr::renderer::{RenderConfig, render_to_png};
use cqr::decoder::decode;
use tempfile::Builder;

#[test]
fn test_decode_generated_qr() {
    // 1. Generate a QR code
    let data = "https://example.com/test-decode";
    let config = RenderConfig::default();
    
    // 2. Create a temporary file path
    let temp_file = Builder::new()
        .suffix(".png")
        .tempfile()
        .expect("Failed to create temp file");
    let path = temp_file.path().to_path_buf();
    let path_str = path.to_str().unwrap().to_string();

    // 3. Render directly to the file
    render_to_png(data, &path, &config).expect("Failed to render PNG");

    // 4. Decode it using our decoder
    let decoded = decode(&path_str).expect("Failed to decode QR code");

    // 5. Assert content matches
    assert_eq!(decoded.content, data);
}
