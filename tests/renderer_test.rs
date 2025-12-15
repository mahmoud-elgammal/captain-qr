use cqr::renderer::{RenderConfig, render_to_svg, render_to_base64, Color, render_to_terminal};

#[test]
fn test_render_to_svg() {
    let data = "Hello SVG";
    let config = RenderConfig::default();
    
    let svg = render_to_svg(data, &config).expect("Failed to render SVG");
    
    assert!(svg.starts_with("<?xml"));
    assert!(svg.contains("<svg"));
    assert!(svg.contains("rect"));
    // Default color is black (0,0,0) so fill="#000000" should be there for modules
    // Background defaults to white "#ffffff"
    assert!(svg.contains("fill=\"#000000\""));
    assert!(svg.contains("fill=\"#ffffff\""));
}

#[test]
fn test_render_to_base64() {
    let data = "Hello Base64";
    let config = RenderConfig {
        size: 256,
        ..RenderConfig::default()
    };
    
    let b64 = render_to_base64(data, &config).expect("Failed to render Base64");
    
    assert!(b64.starts_with("data:image/png;base64,"));
    assert!(b64.len() > 100);
}

#[test]
fn test_render_to_terminal() {
    let data = "Hello Terminal";
    let config = RenderConfig::default();
    
    let term = render_to_terminal(data, &config).expect("Failed to render Terminal");
    
    // Terminal output uses unicode block characters, just check it's not empty
    assert!(!term.is_empty());
}

#[test]
fn test_render_config_custom() {
    let data = "Hello Config";
    let config = RenderConfig {
        size: 300,
        quiet_zone: 4,
        fg_color: Color { r: 255, g: 0, b: 0 }, // Red
        bg_color: Color { r: 0, g: 0, b: 255 }, // Blue
        ..RenderConfig::default()
    };
    
    let svg = render_to_svg(data, &config).expect("Failed to render SVG");
    
    assert!(svg.contains("fill=\"#ff0000\"")); // Red foreground
    assert!(svg.contains("fill=\"#0000ff\"")); // Blue background
}
