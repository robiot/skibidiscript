use std::collections::HashMap;

use crate::{error, interpreter::Interpreter, parser::Expr};

pub mod nerd;
pub mod skui;

#[derive(Debug)]
pub enum LibState {
    SkuiState(skui::SkuiState),
    None,
}

pub struct Library {
    pub functions: HashMap<String, BuiltinFunction>,
    pub state: LibState,
}

pub type BuiltinFunction =
    fn(&mut Interpreter, Vec<Expr>) -> Result<Expr, error::ParseError>;

pub type LibFunctions = HashMap<String, BuiltinFunction>;

pub fn get_lib_state<'a>(itp: &'a mut Interpreter, lib_name: &str) -> &'a mut LibState {
    let lib = itp.libs.get_mut(lib_name).unwrap();
    &mut lib.state
}
