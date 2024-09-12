use std::collections::HashMap;

use winit::event_loop::EventLoop;

use crate::{error, interpreter::Interpreter, parser::Expr};

pub mod libsv2;
// pub mod nerd;
// pub mod skui;
// pub mod nerd_struct_test;

pub enum LibState {
    // NerdState(nerd::NerdState),
    // String(String),
    // EventLoop(EventLoop<()>), // Replace with your actual EventLoop type
    // Add more variants as needed
}

pub struct Library {
    pub functions: HashMap<String, BuiltinFunction>,

    pub state: LibState,
}

pub type BuiltinFunction = fn(&mut Interpreter, Vec<Expr>) -> Result<Expr,  error::ParseError>;

pub type LibFunctions = HashMap<String, BuiltinFunction>;
