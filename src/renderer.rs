//! QR code rendering to various formats: PNG, SVG, terminal, base64

use crate::error::{QrError, Result};
use image::{ImageBuffer, Rgb, Rgba};
use image::imageops::{overlay, resize, FilterType};
use qrcode::render::unicode;
use qrcode::{EcLevel, QrCode};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// RGB color parsed from hex string
#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Parse color from hex string like #RRGGBB or #RGB
    pub fn from_hex(hex: &str) -> Result<Self> {
        let hex = hex.trim_start_matches('#');
        
        let (r, g, b) = match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1], 16).map_err(|_| QrError::InvalidColor(hex.to_string()))? * 17;
                let g = u8::from_str_radix(&hex[1..2], 16).map_err(|_| QrError::InvalidColor(hex.to_string()))? * 17;
                let b = u8::from_str_radix(&hex[2..3], 16).map_err(|_| QrError::InvalidColor(hex.to_string()))? * 17;
                (r, g, b)
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| QrError::InvalidColor(hex.to_string()))?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| QrError::InvalidColor(hex.to_string()))?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| QrError::InvalidColor(hex.to_string()))?;
                (r, g, b)
            }
            _ => return Err(QrError::InvalidColor(hex.to_string())),
        };
        
        Ok(Color { r, g, b })
    }
    
    /// Convert to image crate Rgb type (unused now but kept for API completeness, optional)
    #[allow(dead_code)]
    pub fn to_rgb(&self) -> Rgb<u8> {
        Rgb([self.r, self.g, self.b])
    }
}

/// Configuration for QR code rendering
pub struct RenderConfig {
    pub size: u32,
    pub quiet_zone: u32,
    pub fg_color: Color,
    pub bg_color: Color,
    pub gradient_color: Option<Color>,
    pub logo: Option<PathBuf>,
    pub ec_level: EcLevel,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            size: 512,
            quiet_zone: 2,
            fg_color: Color { r: 0, g: 0, b: 0 },
            bg_color: Color { r: 255, g: 255, b: 255 },
            gradient_color: None,
            logo: None,
            ec_level: EcLevel::M,
        }
    }
}

/// Create QR code with specified error correction level
fn create_qr_code(data: &str, ec_level: EcLevel) -> Result<QrCode> {
    QrCode::with_error_correction_level(data.as_bytes(), ec_level)
        .map_err(|e| QrError::QrGeneration(e.to_string()))
}

/// Render QR code to terminal using Unicode block characters
pub fn render_to_terminal(data: &str, config: &RenderConfig) -> Result<String> {
    let code = create_qr_code(data, config.ec_level)?;
    let image = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .quiet_zone(config.quiet_zone > 0)
        .build();
    Ok(image)
}

/// Create QR code image buffer with support for gradient and logo
fn create_qr_image(data: &str, config: &RenderConfig) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let code = create_qr_code(data, config.ec_level)?;
    let module_count = code.width();
    
    // Calculate module size to fit desired image size
    let total_modules = module_count + (config.quiet_zone as usize * 2);
    let module_size = config.size / total_modules as u32;
    let actual_size = module_size * total_modules as u32;
    
    let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = 
        ImageBuffer::from_pixel(actual_size, actual_size, 
            Rgba([config.bg_color.r, config.bg_color.g, config.bg_color.b, 255]));
    
    let colors = code.to_colors();
    let quiet_offset = config.quiet_zone * module_size;
    
    for (i, color) in colors.iter().enumerate() {
        if *color == qrcode::Color::Dark {
            let qr_x = (i % module_count) as u32;
            let qr_y = (i / module_count) as u32;
            let px_x = qr_x * module_size + quiet_offset;
            let px_y = qr_y * module_size + quiet_offset;
            
            // Calculate gradient color if enabled
            let draw_color = if let Some(grad_end) = config.gradient_color {
                let progress = (px_x as f32 + px_y as f32) / (actual_size as f32 * 2.0);
                let r = (config.fg_color.r as f32 + (grad_end.r as f32 - config.fg_color.r as f32) * progress) as u8;
                let g = (config.fg_color.g as f32 + (grad_end.g as f32 - config.fg_color.g as f32) * progress) as u8;
                let b = (config.fg_color.b as f32 + (grad_end.b as f32 - config.fg_color.b as f32) * progress) as u8;
                Rgba([r, g, b, 255])
            } else {
                Rgba([config.fg_color.r, config.fg_color.g, config.fg_color.b, 255])
            };

            for dy in 0..module_size {
                for dx in 0..module_size {
                    img.put_pixel(px_x + dx, px_y + dy, draw_color);
                }
            }
        }
    }

    // Overlay logo if present
    if let Some(logo_path) = &config.logo {
        let logo_img = image::open(logo_path).map_err(|e| QrError::FileWrite {
             path: logo_path.clone(),
             source: std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to load logo: {}", e)),
        })?;

        // Calculate logo size (e.g., 20% of QR size)
        let logo_size = actual_size / 5;
        let logo_rgba = logo_img.to_rgba8();
        let resized_logo = resize(&logo_rgba, logo_size, logo_size, FilterType::Lanczos3);

        // Calculate position to center
        let center_x = (actual_size - resized_logo.width()) / 2;
        let center_y = (actual_size - resized_logo.height()) / 2;

        overlay(&mut img, &resized_logo, center_x.into(), center_y.into());
    }
    
    Ok(img)
}

/// Render QR code to PNG image
pub fn render_to_png(data: &str, output_path: &PathBuf, config: &RenderConfig) -> Result<()> {
    let img = create_qr_image(data, config)?;
    img.save(output_path).map_err(|e| QrError::FileWrite {
        path: output_path.clone(),
        source: std::io::Error::new(std::io::ErrorKind::Other, e.to_string()),
    })?;
    Ok(())
}

/// Render QR code to SVG string
/// Render QR code to SVG string
pub fn render_to_svg(data: &str, config: &RenderConfig) -> Result<String> {
    use base64::{engine::general_purpose::STANDARD, Engine};
    use image::codecs::png::PngEncoder;
    use image::ImageEncoder;

    let code = create_qr_code(data, config.ec_level)?;
    let module_count = code.width();
    
    let total_modules = module_count + (config.quiet_zone as usize * 2);
    let module_size = config.size / total_modules as u32;
    let actual_size = module_size * total_modules as u32;
    
    let fg_hex = format!("#{:02x}{:02x}{:02x}", config.fg_color.r, config.fg_color.g, config.fg_color.b);
    let bg_hex = format!("#{:02x}{:02x}{:02x}", config.bg_color.r, config.bg_color.g, config.bg_color.b);
    
    let mut svg = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    svg.push_str(&format!(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {size} {size}" width="{size}" height="{size}">"#, size = actual_size));
    
    // Gradient definitions
    let fill_attr = if let Some(grad_end) = config.gradient_color {
        let grad_hex = format!("#{:02x}{:02x}{:02x}", grad_end.r, grad_end.g, grad_end.b);
        svg.push_str(r#"<defs><linearGradient id="grad" x1="0%" y1="0%" x2="100%" y2="100%">"#);
        svg.push_str(&format!(r#"<stop offset="0%" style="stop-color:{};stop-opacity:1" />"#, fg_hex));
        svg.push_str(&format!(r#"<stop offset="100%" style="stop-color:{};stop-opacity:1" />"#, grad_hex));
        svg.push_str(r#"</linearGradient></defs>"#);
        "url(#grad)".to_string()
    } else {
        fg_hex
    };

    svg.push_str(&format!(r#"<rect width="100%" height="100%" fill="{}"/>"#, bg_hex));

    let colors = code.to_colors();
    let quiet_offset = config.quiet_zone * module_size;
    
    for (i, color) in colors.iter().enumerate() {
        if *color == qrcode::Color::Dark {
            let x = (i % module_count) as u32 * module_size + quiet_offset;
            let y = (i / module_count) as u32 * module_size + quiet_offset;
            svg.push_str(&format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}"/>"#,
                x, y, module_size, module_size, fill_attr
            ));
            svg.push('\n');
        }
    }

    // Logo support
    if let Some(logo_path) = &config.logo {
        let logo_img = image::open(logo_path).map_err(|e| QrError::FileWrite {
             path: logo_path.clone(),
             source: std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to load logo: {}", e)),
        })?;

        // Calculate logo size (20% of QR)
        let logo_size = actual_size / 5;
        let logo_rgba = logo_img.to_rgba8();
        let resized_logo = resize(&logo_rgba, logo_size, logo_size, FilterType::Lanczos3);
        
        // Encode resized logo to PNG base64
        let mut buffer = Vec::new();
        let encoder = PngEncoder::new(&mut buffer);
        encoder.write_image(
            resized_logo.as_raw(),
            logo_size,
            logo_size,
            image::ColorType::Rgba8,
        ).map_err(|e| QrError::ImageError(e.to_string()))?;
        
        let b64_logo = STANDARD.encode(&buffer);
        let logo_uri = format!("data:image/png;base64,{}", b64_logo);

        let center_x = (actual_size - logo_size) / 2;
        let center_y = (actual_size - logo_size) / 2;

        svg.push_str(&format!(
            r#"<image x="{}" y="{}" width="{}" height="{}" href="{}" />"#,
            center_x, center_y, logo_size, logo_size, logo_uri
        ));
    }

    svg.push_str("</svg>");
    Ok(svg)
}

/// Render QR code to SVG file
pub fn render_to_svg_file(data: &str, output_path: &PathBuf, config: &RenderConfig) -> Result<()> {
    let svg = render_to_svg(data, config)?;
    let mut file = File::create(output_path).map_err(|e| QrError::FileWrite {
        path: output_path.clone(),
        source: e,
    })?;
    file.write_all(svg.as_bytes()).map_err(|e| QrError::FileWrite {
        path: output_path.clone(),
        source: e,
    })?;
    Ok(())
}

/// Render QR code to base64 encoded PNG string
pub fn render_to_base64(data: &str, config: &RenderConfig) -> Result<String> {
    use base64::{engine::general_purpose::STANDARD, Engine};
    
    let img = create_qr_image(data, config)?;
    let (width, height) = img.dimensions();

    // Encode to PNG in memory
    let mut buffer = Vec::new();
    {
        use image::codecs::png::PngEncoder;
        use image::ImageEncoder;
        let encoder = PngEncoder::new(&mut buffer);
        encoder.write_image(
            img.as_raw(),
            width,
            height,
            image::ColorType::Rgba8,
        )?;
    }
    
    // Convert to base64
    let b64 = STANDARD.encode(&buffer);
    Ok(format!("data:image/png;base64,{}", b64))
}
