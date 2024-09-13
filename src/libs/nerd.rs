use std::collections::HashMap;

use crate::{error, interpreter::Interpreter, libs::get_lib_state, parser::Expr};

use super::{LibFunctions, LibState, Library};

// constant of library name

pub const LIBRARY_NAME: &str = "nerd";

#[derive(Debug, Clone)]
pub struct NerdState {
    pub state_number: i32,
}

pub fn load_nerd_library() -> Library {
    let mut functions: LibFunctions = HashMap::new();
    functions.insert("randInt".to_string(), rand_int_builtin);
    functions.insert("getState".to_string(), get_state);

    // Add more functions as needed

    Library { functions, state: LibState::NerdState(NerdState { state_number: 0 }) }
}

fn rand_int_builtin(itp: &mut Interpreter, args: Vec<Expr>) -> Result<Expr, error::ParseError> {
    let state = get_lib_state(itp, LIBRARY_NAME);

    let real_state = if let LibState::NerdState(state) = state {
        state
    } else {
        return Err(error::ParseError::GeneralError {
            line: itp.line,
            message: "Invalid state".to_string(),
        });
    };

    real_state.state_number += 1;
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

fn get_state(itp: &mut Interpreter, args: Vec<Expr>) -> Result<Expr, error::ParseError> {
    let state = get_lib_state(itp, LIBRARY_NAME);

    let real_state = if let LibState::NerdState(state) = state {
        state
    } else {
        return Err(error::ParseError::GeneralError {
            line: itp.line,
            message: "Invalid state".to_string(),
        });
    };

    Ok(Expr::Number(real_state.state_number as i64))
}
