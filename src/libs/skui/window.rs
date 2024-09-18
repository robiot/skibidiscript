use ggez::conf::WindowMode;
use ggez::graphics::{self, Canvas, Color, DrawMode, Mesh};
use ggez::{Context, ContextBuilder, GameResult};

use crate::libs::skui::{SkuiApp, WindowInfo};
use crate::{error, interpreter::Interpreter, parser::Expr};

use std::time::Duration;

use super::load_skui_state;

// All functions
pub fn create_window_builtin(
    itp: &mut Interpreter,
    _args: Vec<Expr>,
) -> Result<Expr, error::ParseError> {
    let state = load_skui_state(itp)?;

    let window_info = WindowInfo {
        width: 800,
        height: 600,
        title: "Skui Window".to_string(),
    };

    // Initialize ggez context without starting the event loop
    let (ctx, events_loop) = ContextBuilder::new("skui_app", "Elliot")
        .window_mode(WindowMode::default().dimensions(800.0, 600.0))
        .build()
        .expect("Could not create ggez context");

    println!("Window created");

    events_loop.run(move |mut event, _window_target, control_flow | {
        // let ctx = &mut ctx;

        // if ctx.quit_requested {
        //     ctx.continuing = false;
        // }
        // if !ctx.continuing {
        //     *control_flow = ControlFlow::Exit;
        //     return;
        // }
    });

    state.app = Some(SkuiApp {
        window_info: Some(window_info),
        square_pos: [100.0, 100.0], // Initial square position
        ctx: Some(ctx),
    });

    Ok(Expr::Boolean(true))
}

pub fn pump_events_builtin(
    itp: &mut Interpreter,
    _args: Vec<Expr>,
) -> Result<Expr, error::ParseError> {
    let state = super::load_skui_state(itp)?;

    // let event_loop = if let Some(event_loop) = &mut state.event_loop {
    //     event_loop
    // } else {
    //     return Ok(Expr::Boolean(true));
    // };

    let app = if let Some(app) = &mut state.app {
        app
    } else {
        return Ok(Expr::Boolean(true));
    };

    // let status = event_loop.pump_app_events(Some(Duration::ZERO), app);

    // if let PumpStatus::Exit(_) = status {
    //     Ok(Expr::StringLiteral("exit".to_string()))
    // } else {
    //     Ok(Expr::StringLiteral("ok".to_string()))
    // }

    Ok(Expr::StringLiteral("ok".to_string()))
}

pub fn fill_screen_builtin(
    itp: &mut Interpreter,
    args: Vec<Expr>,
) -> Result<Expr, error::ParseError> {
    let colorhex = itp.expr_to_string(itp.consume_argument(&args, 1, 0)?)?;

    let line = itp.line;

    let state = super::load_skui_state(itp)?;

    let app = state.app.as_mut().ok_or(error::ParseError::GeneralError {
        line: line,
        message: "App not initialized".to_string(),
    })?;

    let ctx = app.ctx.as_mut().ok_or(error::ParseError::GeneralError {
        line: line,
        message: "Context not initialized".to_string(),
    })?;

    let mut canvas = graphics::Canvas::from_frame(ctx, Color::CYAN);

    println!("colorhex: {}", colorhex);
    // Draw code here...
    canvas.finish(ctx).unwrap();

    Ok(Expr::StringLiteral("ok".to_string()))
}
