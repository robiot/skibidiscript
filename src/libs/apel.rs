use std::collections::HashMap;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

use crate::{error, interpreter::Interpreter, parser::Expr};

use super::{LibFunctions, LibState, Library};

// constant of library name

pub const LIBRARY_NAME: &str = "apel";

pub fn load_apel_library() -> Library {
    let mut functions: LibFunctions = HashMap::new();
    functions.insert("spawnTheApel".to_string(), spawn_apel_builtin);

    // Add more functions as needed
    Library {
        functions,
        state: LibState::None,
    }
}

fn spawn_apel_builtin(itp: &mut Interpreter, args: Vec<Expr>) -> Result<Expr, error::ParseError> {
    let secretnum = itp.expr_to_number(itp.consume_argument(&args, 1, 0)?)?;

    if secretnum != 420 {
        return Err(error::ParseError::GeneralError {
            line: itp.line,
            message: "arg must be secret number".to_string(),
        });
    }

    let apple_art = r#"
    ,--./,-.
   / #      \
  |          |
   \        /  
    `._,._,'
    "#;

    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut x_dir = 1;
    let mut y_dir = 1;
    let width = 40;
    let height = 10;

    loop {
        // Clear the terminal using ANSI escape codes
        print!("\x1B[2J\x1B[H");
        io::stdout().flush().unwrap();

        // Move the apple
        x_pos += x_dir;
        y_pos += y_dir;

        // Bounce off the walls
        if x_pos <= 0 || x_pos >= width {
            x_dir *= -1;
        }
        if y_pos <= 0 || y_pos >= height {
            y_dir *= -1;
        }

        // Print the apple at the new position
        for _ in 0..y_pos {
            println!();
        }
        for _ in 0..x_pos {
            print!(" ");
        }
        println!("{}", apple_art);

        // Sleep to control the frame rate
        sleep(Duration::from_millis(100));
    }
    Ok(Expr::Number(0))
}
