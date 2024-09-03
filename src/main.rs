// main.rs
mod interpreter;
mod lexer;
mod parser;
mod error;

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

    let mut parser = match Parser::new(&mut lexer){
        Ok(parser) => parser,
        Err(e) => {
            println!("parser init error: {}", e);

            return;
        }
    };

    let statements = match parser.parse() {
        Ok(statements) => statements,
        Err(e) => {
            println!("parser parse error: {}", e);

            return;
        }
    };

    let mut interpreter = Interpreter::new();
    interpreter.interpret(statements);

    // Run the mew function, which is the main function
    let mew_body = match interpreter.functions.get("mew") {
        Some(body) => body.clone(),
        None => {
            println!("error: No mew function found. Stop edging and fix this problem.");

            return;
        },
    };
    for stmt in mew_body {
        interpreter.execute_statement(stmt.clone());
    }
}
