use std::collections::HashMap;

use crate::{error, interpreter::Interpreter, parser::Expr};

use super::{LibFunctions, LibState, Library};

// constant of library name

pub const LIBRARY_NAME: &str = "nerd";

pub fn load_nerd_library() -> Library {
    let mut functions: LibFunctions = HashMap::new();
    functions.insert("randInt".to_string(), rand_int_builtin);

    // Add more functions as needed

    Library { functions, state: LibState::None }
}

fn rand_int_builtin(itp: &mut Interpreter, args: Vec<Expr>) -> Result<Expr, error::ParseError> {
    let min = itp.expr_to_number(itp.consume_argument(&args, 2, 0)?)?;
    let max = itp.expr_to_number(itp.consume_argument(&args, 2, 1)?)?;

    // let states = let LibState::NerdState(state) = state else {
    //     return Err(error::ParseError::GeneralError {
    //         line: itp.line,
    //         message: "Invalid state".to_string(),
    //     });
    // };

    // state.

    if min > max {
        return Err(error::ParseError::GeneralError {
            line: itp.line,
            message: "min cannot be greater than max".to_string(),
        });
    }

    use rand::Rng;
    let mut rng = rand::thread_rng();

    Ok(Expr::Number(rng.gen_range(min..=max)))
}
