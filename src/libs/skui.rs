use std::collections::HashMap;

use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};

use crate::{error, interpreter::Interpreter, parser::Expr};

use super::{LibFunctions, Library};

pub fn load_skui_library() -> Library {
    let mut functions: LibFunctions = HashMap::new();
    functions.insert("createWindow".to_string(), create_window);

    Library { functions }
}

fn create_window(itp: &mut Interpreter, args: Vec<Expr>) -> Result<Expr, error::ParseError> {
    let width = itp.expr_to_number(itp.consume_argument(&args, 2, 0)?)?;
    let height = itp.expr_to_number(itp.consume_argument(&args, 2, 1)?)?;

    // Ensure dimensions are valid
    if width <= 0 || height <= 0 {
        return Err(error::ParseError::GeneralError {
            line: itp.line,
            message: "Window dimensions must be greater than zero.".to_string(),
        });
    }

    println!(
        "Creating window with width: {} and height: {}",
        width, height
    );

    // Create and run the ggez window
    run_ggez_window(width as f32, height as f32).unwrap();

    Ok(Expr::Boolean(true))
}

// Implement a basic ggez game state
struct GameState;

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Game update logic here (e.g., player movement, zombie spawning)
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            100.0,
            2.0,
            Color::WHITE,
        )?;
        canvas.draw(&circle, Vec2::new(0.0, 380.0));

        canvas.finish(ctx)?;
        Ok(())
    }
}

fn run_ggez_window(width: f32, height: f32) -> GameResult<()> {
    let (mut ctx, event_loop) = ContextBuilder::new("skui_game", "Elliot")
        .window_mode(ggez::conf::WindowMode::default().dimensions(width, height))
        .build()
        .expect("Failed to build ggez context");

    let state = GameState;
    event::run(ctx, event_loop, state)
}
