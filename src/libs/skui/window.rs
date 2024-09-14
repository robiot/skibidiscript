use crate::libs::skui::{SkuiApp, WindowInfo};
use crate::{error, interpreter::Interpreter, parser::Expr};

use std::time::Duration;

use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::pump_events::{EventLoopExtPumpEvents, PumpStatus};

// All functions
pub fn create_window_builtin(
    itp: &mut Interpreter,
    args: Vec<Expr>,
) -> Result<Expr, error::ParseError> {
    let width = itp.expr_to_number(itp.consume_argument(&args, 3, 0)?)? as u32;
    let height = itp.expr_to_number(itp.consume_argument(&args, 3, 1)?)? as u32;
    let title = itp.expr_to_string(itp.consume_argument(&args, 3, 2)?)?;

    // Ensure dimensions are valid
    if width <= 0 || height <= 0 {
        return Err(error::ParseError::GeneralError {
            line: itp.line,
            message: "Window dimensions must be greater than zero.".to_string(),
        });
    }

    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    event_loop.set_control_flow(ControlFlow::Wait);

    let app = SkuiApp::new(WindowInfo {
        width,
        height,
        title,
    });

    let state = super::load_skui_state(itp)?;

    state.app = Some(app);
    state.event_loop = Some(event_loop);
  
    Ok(Expr::Boolean(true))
}

pub fn pump_events_builtin(
    itp: &mut Interpreter,
    _args: Vec<Expr>,
) -> Result<Expr, error::ParseError> {
    let state = super::load_skui_state(itp)?;

    let event_loop = if let Some(event_loop) = &mut state.event_loop {
        event_loop
    } else {
        return Ok(Expr::Boolean(true));
    };

    let app = if let Some(app) = &mut state.app {
        app
    } else {
        return Ok(Expr::Boolean(true));
    };

    let status = event_loop.pump_app_events(Some(Duration::ZERO), app);

    if let PumpStatus::Exit(_) = status {
        Ok(Expr::StringLiteral("exit".to_string()))
    } else {
        Ok(Expr::StringLiteral("ok".to_string()))
    }
}
