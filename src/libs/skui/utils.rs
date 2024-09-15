pub fn hex_to_rgba(hex: &str) -> Result<[u8; 4], &'static str> {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 {
        return Err("Invalid hex format");
    }

    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid red value")?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid green value")?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid blue value")?;

    Ok([r, g, b, 255]) // Full opacity (255)
}
