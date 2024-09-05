// interpreter.rs
use crate::{
    error,
    parser::{Expr, Stmt},
};
use {std::collections::HashMap, std::io::Write};

pub struct Interpreter {
    pub variables: HashMap<String, Expr>,
    pub functions: HashMap<String, Vec<Stmt>>,
    // pub libs: HashMap<String, Vec<Stmt>>,
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

                while self.evaluate_expression(condition.clone())? != Expr::Number(0) {
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

                if self.evaluate_expression(condition)? != Expr::Number(0) {
                    for stmt in then_branch {
                        self.execute_statement(stmt)?;
                    }
                } else if let Some(else_branch) = else_branch {
                    for stmt in else_branch {
                        self.execute_statement(stmt)?;
                    }
                }
            }
            Stmt::Expression { value: expr, line } => {
                self.line = line;
                self.evaluate_expression(expr)?;
            }
            Stmt::Return { value: expr, line } => {
                self.line = line;
                let _value = self.evaluate_expression(expr);
                // Return logic here if needed
            }
        }

        Ok(())
    }

    pub fn evaluate_expression(&mut self, expr: Expr) -> Result<Expr, error::ParseError> {
        match expr {
            Expr::Ident(name) => Ok(self
                .variables
                .get(&name)
                .unwrap_or(&Expr::Number(0))
                .clone()),
            Expr::Number(value) => Ok(Expr::Number(value)),
            Expr::StringLiteral(string) => Ok(Expr::StringLiteral(string)), // Handle strings differently if neded
            Expr::Boolean(value) => Ok(Expr::Boolean(value)),
            Expr::FunctionCall { name, object, args } => self.execute_function_call(name, object, args),
            Expr::_BinOp { left, op, right } => {
                let left_val = self.evaluate_expression(*left)?;
                let right_val = self.evaluate_expression(*right)?;
                match (left_val, right_val) {
                    (Expr::Number(left_val), Expr::Number(right_val)) => match op.as_str() {
                        "+" => Ok(Expr::Number(left_val + right_val)),
                        "-" => Ok(Expr::Number(left_val - right_val)),
                        "*" => Ok(Expr::Number(left_val * right_val)),
                        "/" => Ok(Expr::Number(left_val / right_val)),
                        "==" => Ok(Expr::Boolean(left_val == right_val)),
                        _ => Err(error::ParseError::GeneralError {
                            line: 0,
                            message: format!("Unknown operator: {}", op),
                        }),
                    },
                    (Expr::StringLiteral(left_val), Expr::StringLiteral(right_val)) => {
                        if op == "+" {
                            Ok(Expr::StringLiteral(left_val + &right_val))
                        } else {
                            Err(error::ParseError::GeneralError {
                                line: 0,
                                message: format!("Invalid operator for strings: {}", op),
                            })
                        }
                    }
                    _ => Err(error::ParseError::GeneralError {
                        line: 0,
                        message: "Type mismatch in binary operation".to_string(),
                    }),
                }
            }
        }
    }

    fn expr_to_string(&mut self, expr: Expr) -> Result<String, error::ParseError> {
        match self.evaluate_expression(expr)? {
            Expr::Number(value) => Ok(value.to_string()),
            Expr::StringLiteral(value) => Ok(value),
            Expr::Boolean(value) => Ok(if value {
                "sigma".to_string()
            } else {
                "ohio".to_string()
            }),
            _ => Ok("".to_string()), // Return an empty string for other types
        }
    }

    pub fn execute_function_call(
        &mut self,
        name: String,
        object: Option<Box<Expr>>,
        args: Vec<Expr>,
    ) -> Result<Expr, error::ParseError> {
        if let Some(_object) = object {
            return Err(error::ParseError::GeneralError {
                line: self.line,
                message: "Object calls are not supported".to_string(),
            });
        }

        match name.as_str() {
            // "nerd.randInt" => {
            //     if args.len() != 2 {
            //         panic!("randInt expects 2 arguments");
            //     }
            //     let min = self.evaluate_expression(args[0].clone())?;
            //     let max = self.evaluate_expression(args[1].clone())?;
            //     if min > max {
            //         panic!("randInt min should be less than max");
            //     }
            //     use rand::Rng;
            //     let mut rng = rand::thread_rng();

            //     Ok(rng.gen_range(min..=max))
            // }
            "yap" => {
                let mut output = String::new();
                for arg in args {
                    output.push_str(&self.expr_to_string(arg)?);
                    output.push_str(" ");
                }

                println!("{}", output);

                Ok(Expr::Boolean(true)) // Return value for function calls
            }
            "attemptrizz" => {
                let mut output = String::new();

                for arg in args {
                    output.push_str(&self.expr_to_string(arg)?);
                }

                print!("{}", output);

                std::io::stdout().flush().expect("Failed to flush stdout");

                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                Ok(Expr::StringLiteral(input.trim().to_string()))
            }
            _ => {
                if let Some(body) = self.functions.get(&name) {
                    for stmt in body.clone() {
                        self.execute_statement(stmt)?;
                    }
                    Ok(Expr::Number(0)) // Default return value for functions
                } else {
                    Err(error::ParseError::UnknownFunction {
                        name,
                        line: self.line,
                    })
                }
            }
        }
    }
}
