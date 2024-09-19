use pixels::Pixels;
use winit::event_loop::EventLoop;

use crate::error;

use super::{SkuiApp, SkuiState, WindowInfo};

pub fn hex_to_rgba(hex: &str, line: usize) -> Result<[u8; 4], error::ParseError> {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 {
        return Err(error::ParseError::GeneralError {
            line,
            message: "Invalid hex color".to_string(),
        });
    }

    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| error::ParseError::GeneralError {
        line,
        message: "Invalid red value in hex color".to_string(),
    })?;

    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| error::ParseError::GeneralError {
        line,
        message: "Invalid green value in hex color".to_string(),
    })?;

    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| error::ParseError::GeneralError {
        line,
        message: "Invalid blue value in hex color".to_string(),
    })?;

    Ok([r, g, b, 255]) // Full opacity (255)
}

pub fn get_app<'a>(state: &'a mut SkuiState) -> Result<&'a mut SkuiApp, error::ParseError> {
    state.app.as_mut().ok_or_else(|| {
        error::ParseError::Other(
            "App is not initialized. Have you created a window first?".to_string(),
        )
    })
}
