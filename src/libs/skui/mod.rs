use super::{get_lib_state, LibFunctions, LibState, Library};
use crate::{error, interpreter::Interpreter};

use std::collections::HashMap;

use ggez::Context;

pub mod clock;
pub mod window;
pub mod utils;

pub const LIBRARY_NAME: &str = "skui";

#[derive(Debug)]
struct WindowInfo {
    width: u32,
    height: u32,
    title: String,
}

#[derive(Default, Debug)]
pub struct SkuiApp {
    window_info: Option<WindowInfo>,
    square_pos: [f32; 2], // Position of the square on the screen
    ctx: Option<Context>, // GGEZ context for rendering
}

#[derive(Default, Debug)]
pub struct SkuiState {
    app: Option<SkuiApp>,
    clock: Option<clock::Clock>,
}

// https://www.reddit.com/r/rust/comments/1dnaase/rust_and_winit_0303/
pub fn load_skui_library() -> Library {
    let mut functions: LibFunctions = HashMap::new();
    functions.insert("createWindow".to_string(), window::create_window_builtin);
    functions.insert("pumpEvents".to_string(), window::pump_events_builtin);

    // Clock tick
    functions.insert(
        "setFramesPerSkibidi".to_string(),
        clock::clock_set_fps_builtin,
    );
    functions.insert("clockEdge".to_string(), clock::clock_tick_builtin);

    // Draw
    functions.insert("goonScreen".to_string(), window::fill_screen_builtin);

    Library {
        functions,
        state: LibState::SkuiState(SkuiState {
            app: None,
            clock: None,
        }),
    }
}

pub fn load_skui_state(itp: &mut Interpreter) -> Result<&mut SkuiState, error::ParseError> {
    let line = itp.line;
    let libstate = get_lib_state(itp, LIBRARY_NAME);

    let state = if let LibState::SkuiState(state) = libstate {
        state
    } else {
        return Err(error::ParseError::GeneralError {
            line,
            message: "Invalid state".to_string(),
        });
    };

    Ok(state)
}

//
// The state of the skui/ winit wrapper
//
