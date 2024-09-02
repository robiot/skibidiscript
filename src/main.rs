// main.rs
mod lexer;
mod parser;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <script>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let source = fs::read_to_string(filename).expect("Could not read file");

    let mut lexer = Lexer::new(&source);
    let mut parser = Parser::new(&mut lexer);
    let statements = parser.parse();

    let mut interpreter = Interpreter::new();
    interpreter.interpret(statements);

    let mew_body = match interpreter.functions.get("mew") {
        Some(body) => body.clone(),
        None => panic!("No mew function found. Stop edging and fix this problem."),
    };
    for stmt in mew_body {
        interpreter.execute_statement(stmt.clone());
    }
}