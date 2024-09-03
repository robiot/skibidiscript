// interpreter.rs
use crate::{
    error,
    parser::{Expr, Stmt},
};
use std::collections::HashMap;

pub struct Interpreter {
    pub variables: HashMap<String, i64>,
    pub functions: HashMap<String, Vec<Stmt>>,
    pub line: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            functions: HashMap::new(),
            line: 0,
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), error::ParseError> {
        for stmt in statements {
            self.execute_statement(stmt)?;
        }

        Ok(())
    }

    pub fn execute_statement(&mut self, stmt: Stmt) -> Result<(), error::ParseError> {
        match stmt {
            Stmt::Function { name, body, line } => {
                self.line = line;

                self.functions.insert(name, body);
            }
            Stmt::VariableAssign { name, value, line } => {
                self.line = line;

                let evaluated = self.evaluate_expression(value)?;
                self.variables.insert(name, evaluated);
            }
            Stmt::While {
                condition,
                body,
                line,
            } => {
                self.line = line;

                while self.evaluate_expression(condition.clone())? != 0 {
                    for stmt in &body {
                        self.execute_statement(stmt.clone())?;
                    }
                }
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
                line,
            } => {
                self.line = line;

                if self.evaluate_expression(condition)? != 0 {
                    for stmt in then_branch {
                        self.execute_statement(stmt)?;
                    }
                } else if let Some(else_branch) = else_branch {
                    for stmt in else_branch {
                        self.execute_statement(stmt)?;
                    }
                }
            }
            Stmt::Expression {
                value: expr,
                line,
            } => {
                self.line = line;
                self.evaluate_expression(expr)?;
            }
            Stmt::Return {
                value: expr,
                line,
            } => {
                self.line = line;
                let _value = self.evaluate_expression(expr);
                // Return logic here if needed
            }
        }

        Ok(())
    }

    pub fn evaluate_expression(&mut self, expr: Expr) -> Result<i64, error::ParseError> {
        match expr {
            Expr::Ident(name) => Ok(*self.variables.get(&name).unwrap_or(&0)),
            Expr::Number(value) => Ok(value),
            Expr::StringLiteral(_string) => Ok(0), // Handle strings differently if neded
            Expr::Boolean(value) => {
                if value {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
            Expr::FunctionCall { name, args } => self.execute_function_call(name, args),
            Expr::BinOp { left, op, right } => {
                let left_val = self.evaluate_expression(*left)?;
                let right_val = self.evaluate_expression(*right)?;
                match op.as_str() {
                    "+" => Ok(left_val + right_val),
                    "-" => Ok(left_val - right_val),
                    "*" => Ok(left_val * right_val),
                    "/" => Ok(left_val / right_val),
                    "==" => {
                        if left_val == right_val {
                            Ok(1)
                        } else {
                            Ok(0)
                        }
                    }
                    _ => Err(error::ParseError::GeneralError {
                        line: 0,
                        message: format!("Unknown operator: {}", op),
                    }),
                }
            }
        }
    }

    pub fn execute_function_call(
        &mut self,
        name: String,
        args: Vec<Expr>,
    ) -> Result<i64, error::ParseError> {
        match name.as_str() {
            "nerd.randInt" => {
                if args.len() != 2 {
                    panic!("randInt expects 2 arguments");
                }
                let min = self.evaluate_expression(args[0].clone())?;
                let max = self.evaluate_expression(args[1].clone())?;
                if min > max {
                    panic!("randInt min should be less than max");
                }
                use rand::Rng;
                let mut rng = rand::thread_rng();

                Ok(rng.gen_range(min..=max))
            }
            "yap" => {
                for arg in args {
                    print!("{}", self.evaluate_expression(arg)?);
                }
                println!();

                Ok(0)
            }
            "yapask" => {
                for arg in args {
                    print!("{}", self.evaluate_expression(arg)?);
                }

                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                Ok(input.trim().parse::<i64>().unwrap_or(0))
            }
            _ => {
                // Clone the function body out of the borrowing context to avoid conflicts
                let body = match self.functions.get(&name) {
                    Some(body) => body.clone(),
                    None => Err(error::ParseError::UnknownFunction {
                        name,
                        line: self.line,
                    })?,
                };
                for stmt in body {
                    self.execute_statement(stmt.clone())?;
                }

                Ok(0)
            }
        }
    }
}
