// interpreter.rs
use crate::{
    error,
    lexer::Token,
    libs::{self, Library},
    parser::{Expr, Stmt},
};
use {std::collections::HashMap, std::io::Write};

pub struct Interpreter {
    pub variables: HashMap<String, Expr>,
    pub functions: HashMap<String, Vec<Stmt>>,
    pub libs: HashMap<String, Library>,
    pub line: usize,
}

#[derive(Debug, PartialEq)]
enum ControlFlow {
    Continue,
    Return(Expr),
    None,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            functions: HashMap::new(),
            libs: HashMap::new(),
            line: 0,
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), error::ParseError> {
        for stmt in statements {
            self.execute_statement(stmt)?;
        }

        Ok(())
    }
    
    fn execute_statement(&mut self, stmt: Stmt) -> Result<ControlFlow, error::ParseError> {
        match stmt {
            Stmt::Function { name, body, line } => {
                self.line = line;
                self.functions.insert(name, body);
                Ok(ControlFlow::None)
            }
            Stmt::VariableAssign { name, value, line } => {
                self.line = line;
                let evaluated = self.evaluate_expression(value)?;
                self.variables.insert(name, evaluated);
                Ok(ControlFlow::None)
            }
            Stmt::While {
                condition,
                body,
                line,
            } => {
                self.line = line;

                while self.evaluate_expression(condition.clone())? != Expr::Boolean(false) {
                    for stmt in &body {
                        match self.execute_statement(stmt.clone())? {
                            ControlFlow::Continue => break, // Skip to the next iteration
                            ControlFlow::Return(value) => return Ok(ControlFlow::Return(value)),
                            ControlFlow::None => {} // Continue executing the next statement
                        }
                    }
                }

                Ok(ControlFlow::None)
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
                line,
            } => {
                self.line = line;

                if self.evaluate_expression(condition)? != Expr::Boolean(false) {
                    for stmt in then_branch {
                        match self.execute_statement(stmt)? {
                            ControlFlow::Continue => return Ok(ControlFlow::Continue),
                            ControlFlow::Return(value) => return Ok(ControlFlow::Return(value)),
                            ControlFlow::None => {} // Continue executing the next statement
                        }
                    }
                } else if let Some(else_branch) = else_branch {
                    for stmt in else_branch {
                        match self.execute_statement(stmt)? {
                            ControlFlow::Continue => return Ok(ControlFlow::Continue),
                            ControlFlow::Return(value) => return Ok(ControlFlow::Return(value)),
                            ControlFlow::None => {} // Continue executing the next statement
                        }
                    }
                }

                Ok(ControlFlow::None)
            }
            Stmt::Expression { value: expr, line } => {
                self.line = line;
                self.evaluate_expression(expr)?;
                Ok(ControlFlow::None)
            }
            Stmt::Import { library, line } => {
                self.line = line;

                match library.as_str() {
                    // "nerd" => {
                    //     self.libs.insert(library, libs::nerd::load_nerd_library());
                    // }
                    // "skui" => {
                    //     self.libs.insert(library, libs::skui::load_skui_library());
                    // }
                    _ => {
                        return Err(error::ParseError::GeneralError {
                            line: self.line,
                            message: format!("Unknown library gyatt import: {}", library),
                        });
                    }
                }

                Ok(ControlFlow::None)
            }
            Stmt::Return { value, line } => {
                self.line = line;
                let return_value = self.evaluate_expression(value)?;
                Ok(ControlFlow::Return(return_value))
            }
            Stmt::Continue { line } => {
                self.line = line;
                Ok(ControlFlow::Continue)
            }
            // _ => Err(error::ParseError::GeneralError {
            //     line: self.line,
            //     message: "Unsupported statement".to_string(),
            // }),
        }
    }

    pub fn evaluate_expression(&mut self, expr: Expr) -> Result<Expr, error::ParseError> {
        match expr {
            Expr::Ident(name) => {
                if let Some(value) = self.variables.get(&name) {
                    Ok(value.clone())
                } else {
                    Err(error::ParseError::UnknownVariable {
                        name,
                        line: self.line,
                    })
                }
            }
            Expr::Number(value) => Ok(Expr::Number(value)),
            Expr::StringLiteral(string) => Ok(Expr::StringLiteral(string)), // Handle strings differently if neded
            Expr::Boolean(value) => Ok(Expr::Boolean(value)),
            // Expr::None => Ok(Expr::None),
            Expr::FunctionCall { name, object, args } => {
                self.execute_function_call(name, object, args)
            }
            Expr::BinOp { left, op, right } => {
                let left_val = self.evaluate_expression(*left)?;
                let right_val = self.evaluate_expression(*right)?;

                // Handle different types of binary operations
                match op {
                    Token::Plus => match (left_val.clone(), right_val.clone()) {
                        (Expr::Number(l), Expr::Number(r)) => Ok(Expr::Number(l + r)),
                        (Expr::StringLiteral(l), Expr::StringLiteral(r)) => {
                            Ok(Expr::StringLiteral(l + r.as_str()))
                        }
                        _ => Err(error::ParseError::TypeError {
                            expected: Expr::Number(-1),
                            found: Expr::BinOp {
                                left: Box::new(left_val),
                                op,
                                right: Box::new(right_val),
                            },
                            line: self.line,
                        }),
                    },
                    Token::Minus => {
                        if let (Expr::Number(l), Expr::Number(r)) =
                            (left_val.clone(), right_val.clone())
                        {
                            Ok(Expr::Number(l - r))
                        } else {
                            Err(error::ParseError::TypeError {
                                expected: Expr::Number(-1),
                                found: Expr::BinOp {
                                    left: Box::new(left_val),
                                    op,
                                    right: Box::new(right_val),
                                },
                                line: self.line,
                            })
                        }
                    }
                    Token::Star => {
                        if let (Expr::Number(l), Expr::Number(r)) =
                            (left_val.clone(), right_val.clone())
                        {
                            Ok(Expr::Number(l * r))
                        } else {
                            Err(error::ParseError::TypeError {
                                expected: Expr::Number(-1),
                                found: Expr::BinOp {
                                    left: Box::new(left_val),
                                    op,
                                    right: Box::new(right_val),
                                },
                                line: self.line,
                            })
                        }
                    }
                    Token::Slash => {
                        if let (Expr::Number(l), Expr::Number(r)) =
                            (left_val.clone(), right_val.clone())
                        {
                            if r == 0 {
                                Err(error::ParseError::DivisionByZero { line: self.line })
                            } else {
                                Ok(Expr::Number(l / r))
                            }
                        } else {
                            Err(error::ParseError::TypeError {
                                expected: Expr::Number(-1),
                                found: Expr::BinOp {
                                    left: Box::new(left_val),
                                    op,
                                    right: Box::new(right_val),
                                },
                                line: self.line,
                            })
                        }
                    }
                    Token::Rizz => match (left_val.clone(), right_val.clone()) {
                        (Expr::Number(l), Expr::Number(r)) => Ok(Expr::Boolean(l == r)),
                        (Expr::StringLiteral(l), Expr::StringLiteral(r)) => {
                            Ok(Expr::Boolean(l == r.as_str()))
                        }
                        _ => Err(error::ParseError::TypeError {
                            expected: Expr::Boolean(false),
                            found: Expr::BinOp {
                                left: Box::new(left_val),
                                op,
                                right: Box::new(right_val),
                            },
                            line: self.line,
                        }),
                    },
                    Token::GreaterThan => match (left_val.clone(), right_val.clone()) {
                        (Expr::Number(l), Expr::Number(r)) => Ok(Expr::Boolean(l > r)),
                        _ => Err(error::ParseError::TypeError {
                            expected: Expr::Boolean(false),
                            found: Expr::BinOp {
                                left: Box::new(left_val),
                                op,
                                right: Box::new(right_val),
                            },
                            line: self.line,
                        }),
                    },
                    Token::LessThan => match (left_val.clone(), right_val.clone()) {
                        (Expr::Number(l), Expr::Number(r)) => Ok(Expr::Boolean(l < r)),
                        _ => Err(error::ParseError::TypeError {
                            expected: Expr::Boolean(false),
                            found: Expr::BinOp {
                                left: Box::new(left_val),
                                op,
                                right: Box::new(right_val),
                            },
                            line: self.line,
                        }),
                    },
                    // Add other operators as needed
                    _ => Err(error::ParseError::GeneralError {
                        line: self.line,
                        message: format!("Unsupported operator: {:?}", op),
                    }),
                }
            }
        }
    }

    pub fn expr_to_string(&mut self, expr: Expr) -> Result<String, error::ParseError> {
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

    pub fn expr_to_number(&mut self, expr: Expr) -> Result<i64, error::ParseError> {
        match self.evaluate_expression(expr.clone())? {
            Expr::Number(value) => Ok(value),
            _ => Err(error::ParseError::TypeError {
                expected: Expr::Number(-1),
                found: expr.clone(),
                line: self.line,
            }),
        }
    }

    // let args: Vec<Expr> = vec![/* some expressions */];

    // // Define validators corresponding to each argument
    // let validators = vec![
    //     |expr| itp.expr_to_number(expr),  // First validator: expects a number
    //     |expr| itp.expr_to_number(expr),  // Second validator: expects a number
    //     |expr| itp.expr_to_string(expr),  // Third validator: expects a string
    // ];

    // let parsed_args = itp.consume_arguments(&args, &validators)?;

    // // Pattern match and destructure the result into a tuple
    // let (width, height, title) = match parsed_args.as_slice() {
    //     [width, height, title] => (*width as u32, *height as u32, title.clone()),
    // };
    // pub fn consume_arguments<T>(
    //     args: &[Expr], 
    //     validators: &[impl Fn(&Expr) -> Result<T, error::ParseError>]
    // ) -> Result<Vec<T>, error::ParseError> {
    //     if args.len() != validators.len() {
    //         return Err(error::ParseError::ArgumentMismatch {
    //             found: args.len(),
    //             expected: validators.len(),
    //             line: self.line,
    //         });
    //     }
    
    //     // Use iterator to apply validators sequentially
    //     args.iter()
    //         .zip(validators.iter())
    //         .map(|(arg, validator)| validator(arg))
    //         .collect::<Result<Vec<T>, _>>()  // Collect into a `Result<Vec<T>, ParseError>`
    // }

    // consume_argument(&args, 1)
    // Helper function to consume arguments
    pub fn consume_argument(
        &self,
        args: &Vec<Expr>,
        expected: usize,
        index: usize,
    ) -> Result<Expr, error::ParseError> {
        if index >= args.len() || args.len() != expected {
            return Err(error::ParseError::ArgumentMismatch {
                found: args.len(),
                expected,
                line: self.line,
            });
        }

        let arg = args.get(index).unwrap().clone();

        Ok(arg)
    }

    pub fn execute_user_function(
        &mut self,
        name: String,
        _args: Vec<Expr>,
    ) -> Result<Expr, error::ParseError> {
        if let Some(body) = self.functions.get(&name) {
            // for stmt in body.clone() {
            //     if let Stmt::Return { value, line } = stmt {
            //         self.line = line;

            //         return Ok(self.evaluate_expression(value)?);
            //     } else {
            //         self.execute_statement(stmt)?;
            //     }
            // }

            for stmt in body.clone() {
                let controlflow = self.execute_statement(stmt)?;

                match controlflow {
                    ControlFlow::Return(value) => return Ok(value),
                    _ => {}
                }
            }

            Ok(Expr::Number(0)) // Default return value for functions
        } else {
            Err(error::ParseError::UnknownFunction {
                name,
                line: self.line,
            })
        }
    }

    pub fn execute_function_call(
        &mut self,
        name: String,
        object: Option<Box<Expr>>,
        args: Vec<Expr>,
    ) -> Result<Expr, error::ParseError> {
        // Just library functions, for now
        if let Some(object) = object {
            // check that object is ident

            let object = if let Expr::Ident(object) = *object {
                object
            } else {
                return Err(error::ParseError::GeneralError {
                    line: self.line,
                    message: "Object calls of other types than IDENT are not supported".to_string(),
                });
            };

            // check if object exisrt in the libs
            let lib = if let Some(lib) = self.libs.get(&object) {
                Ok(lib)
            } else {
                Err(error::ParseError::GeneralError {
                    line: self.line,
                    message: format!("Unknown library: {}", object),
                })
            }?;

            let func = if let Some(function) = lib.functions.get(&name) {
                Ok(function)
            } else {
                return Err(error::ParseError::GeneralError {
                    line: self.line,
                    message: format!("Unknown function: {} on object {}", name, object),
                });
            }?;

            return func(self, args.clone());
        }

        // normal function call
        match name.as_str() {
            // convert to number
            "aura" => {
                let arg = self.consume_argument(&args, 1, 0)?;

                let number = match self.evaluate_expression(arg.clone())? {
                    Expr::Number(value) => value,
                    Expr::StringLiteral(value) => {
                        value
                            .parse::<i64>()
                            .map_err(|_| error::ParseError::GeneralError {
                                line: self.line,
                                message: format!("Failed to parse {} as number", value),
                            })?
                    }
                    _ => {
                        return Err(error::ParseError::TypeError {
                            expected: Expr::Number(-1),
                            found: arg,
                            line: self.line,
                        })
                    }
                };

                Ok(Expr::Number(number))
            }
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
                // expect only one argument

                let question = self.expr_to_string(self.consume_argument(&args, 1, 0)?)?;

                print!("{}", question);

                std::io::stdout().flush().expect("Failed to flush stdout");

                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                Ok(Expr::StringLiteral(input.trim().to_string()))
            }
            _ => self.execute_user_function(name, args),
        }
    }
}
