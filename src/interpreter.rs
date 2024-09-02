// interpreter.rs
use crate::parser::{Expr, Stmt};
use std::{borrow::Borrow, collections::HashMap};

pub struct Interpreter {
    pub variables: HashMap<String, i64>,
    pub functions: HashMap<String, Vec<Stmt>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for stmt in statements {
            self.execute_statement(stmt);
        }
    }

    pub fn execute_statement(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Function { name, body } => {
                self.functions.insert(name, body);
            }
            Stmt::VariableAssign { name, value } => {
                let evaluated = self.evaluate_expression(value);
                self.variables.insert(name, evaluated);
            }
            Stmt::While { condition, body } => {
                while self.evaluate_expression(condition.clone()) != 0 {
                    for stmt in &body {
                        self.execute_statement(stmt.clone());
                    }
                }
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if self.evaluate_expression(condition) != 0 {
                    for stmt in then_branch {
                        self.execute_statement(stmt);
                    }
                } else if let Some(else_branch) = else_branch {
                    for stmt in else_branch {
                        self.execute_statement(stmt);
                    }
                }
            }
            Stmt::Expression(expr) => {
                self.evaluate_expression(expr);
            }
            Stmt::Return(expr) => {
                let _value = self.evaluate_expression(expr);
                // Return logic here if needed
            }
        }
    }

    pub fn evaluate_expression(&mut self, expr: Expr) -> i64 {
        match expr {
            Expr::Ident(name) => *self.variables.get(&name).unwrap_or(&0),
            Expr::Number(value) => value,
            Expr::StringLiteral(_string) => 0, // Handle strings differently if neded
            Expr::Boolean(value) => {
                if value {
                    1
                } else {
                    0
                }
            }
            Expr::FunctionCall { name, args } => self.execute_function_call(name, args),
            Expr::BinOp { left, op, right } => {
                let left_val = self.evaluate_expression(*left);
                let right_val = self.evaluate_expression(*right);
                match op.as_str() {
                    "+" => left_val + right_val,
                    "-" => left_val - right_val,
                    "*" => left_val * right_val,
                    "/" => left_val / right_val,
                    "==" => {
                        if left_val == right_val {
                            1
                        } else {
                            0
                        }
                    }
                    _ => panic!("Unknown binary operator: {}", op),
                }
            }
        }
    }

    pub fn execute_function_call(&mut self, name: String, args: Vec<Expr>) -> i64 {
        match name.as_str() {
            "nerd.randInt" => {
                if args.len() != 2 {
                    panic!("randInt expects 2 arguments");
                }
                let min = self.evaluate_expression(args[0].clone());
                let max = self.evaluate_expression(args[1].clone());
                if min > max {
                    panic!("randInt min should be less than max");
                }
                use rand::Rng;
                let mut rng = rand::thread_rng();
                rng.gen_range(min..=max)
            }
            "yap" => {
                for arg in args {
                    print!("{}", self.evaluate_expression(arg));
                }
                println!();
                0
            }
            "yapask" => {
                for arg in args {
                    print!("{}", self.evaluate_expression(arg));
                }
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                input.trim().parse::<i64>().unwrap_or(0)
            }
            _ => {
                // Clone the function body out of the borrowing context to avoid conflicts
                let body = match self.functions.get(&name) {
                    Some(body) => body.clone(),
                    None => panic!("Unknown function: {}", name),
                };
                for stmt in body {
                    self.execute_statement(stmt.clone());
                }
                0
            }
        }
    }
}
