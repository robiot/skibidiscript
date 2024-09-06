use std::collections::HashMap;

use crate::{error, interpreter::Interpreter, parser::Expr};

pub mod nerd;

pub struct Library {
    pub functions: HashMap<String, BuiltinFunction>,
}

pub type BuiltinFunction = fn(&mut Interpreter, Vec<Expr>) -> Result<Expr,  error::ParseError>;

pub type LibFunctions = HashMap<String, BuiltinFunction>;
