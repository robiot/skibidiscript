// main.rs
mod error;
mod interpreter;
mod lexer;
mod libs;
mod parser;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // First check that we have a script
    if args.len() < 2 {
        println!("Usage: {} <script>", args[0]);

        return;
    }

    // Read the file
    let filename = &args[1];
    let source = match fs::read_to_string(filename) {
        Ok(source) => source,
        Err(e) => {
            println!("error: Couldn't read file: {}", e);

            return;
        }
    };

    // Initialize the lexer and parser
    let mut lexer = Lexer::new(&source);

    let mut parser = match Parser::new(&mut lexer) {
        Ok(parser) => parser,
        Err(e) => {
            println!("parser init error: {}", e);

            return;
        }
    };

    let statements = match parser.parse() {
        Ok(statements) => statements,
        Err(e) => {
            println!("parse_error: {}", e);

            return;
        }
    };

    println!("statements: {:#?}", statements);

    let mut interpreter = Interpreter::new();
    match interpreter.interpret(statements) {
        Ok(_) => {}
        Err(e) => {
            println!("interpreter: {}", e);

            return;
        }
    };

    // Run the mew function, which is the main function
    match interpreter.execute_user_function("mew".to_string(), vec![]) {
        Ok(_) => {}
        Err(e) => {
            println!("interpreter: {}", e);

            return;
        }
    };
}
