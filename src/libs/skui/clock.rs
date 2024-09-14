
use super::{get_lib_state, LibState};
use crate::libs::skui::{SkuiApp, WindowInfo, LIBRARY_NAME};
use crate::{error, interpreter::Interpreter, parser::Expr};

use std::time::Duration;

use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::pump_events::EventLoopExtPumpEvents;

pub fn clock_tick_builtin(itp: &mut Interpreter, args: Vec<Expr>) -> Result<Expr, error::ParseError> {
    let state = super::load_skui_state(itp)?;

    let event_loop = if let Some(event_loop) = &mut state.event_loop {
        event_loop
    } else {
        return Ok(Expr::Boolean(true));
    };

    Ok(Expr::Boolean(true))
}
