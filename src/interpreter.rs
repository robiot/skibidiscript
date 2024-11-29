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
    pub classes: HashMap<String, ClassDefinition>,
    pub instances: HashMap<String, Instance>, // Instance ID, new Class()
    pub libs: HashMap<String, Library>,

    // Live runtime info
    pub current_instance: Option<String>, // id of the current instance
    pub line: usize,
}

#[derive(Debug)]
pub struct ClassDefinition {
    pub functions: HashMap<String, Vec<Stmt>>, // Method name to statements
}

#[derive(Debug)]
pub struct Instance {
    pub variables: HashMap<String, Expr>,
    class_name: String,
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
            classes: HashMap::new(),
            instances: HashMap::new(),
            libs: HashMap::new(),
            current_instance: None,
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
            Stmt::Class {
                name,
                functions,
                line,
            } => {
                self.line = line;

                // Create a new ClassDefinition
                let class_definition = ClassDefinition {
                    functions: functions
                        .iter()
                        .filter_map(|method| {
                            // If the method is a Function statement, store it
                            if let Stmt::Function {
                                name: method_name,
                                body,
                                ..
                            } = method
                            {
                                Some((method_name.clone(), body.clone()))
                            } else {
                                None // Ignore non-function statements
                            }
                        })
                        .collect::<HashMap<String, Vec<Stmt>>>(),
                };

                // Store the class definition in the classes HashMap
                self.classes.insert(name, class_definition);

                Ok(ControlFlow::None)
            }
            Stmt::Function { name, body, line } => {
                self.line = line;
                self.functions.insert(name, body);
                Ok(ControlFlow::None)
            }
            Stmt::VariableAssign {
                name,
                object: _,
                value,
                line,
            } => {
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
                    libs::nerd::LIBRARY_NAME => {
                        self.libs.insert(library, libs::nerd::load_nerd_library());
                    }
                    libs::skui::LIBRARY_NAME => {
                        self.libs.insert(library, libs::skui::load_skui_library());
                    }
                    libs::apel::LIBRARY_NAME => {
                        self.libs.insert(library, libs::apel::load_apel_library());
                    }
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
            _ => Err(error::ParseError::GeneralError {
                line: self.line,
                message: "Unsupported statement".to_string(),
            }),
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
            Expr::List(values) => {
                let mut evaluated_values = Vec::new();
                for value in values {
                    evaluated_values.push(self.evaluate_expression(value)?);
                }

                Ok(Expr::List(evaluated_values))
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
            Expr::NewInstance {
                class_name,
                args: _,
            } => {
                // Look up the class definition
                let class_def = self.classes.get(&class_name).ok_or_else(|| {
                    error::ParseError::GeneralError {
                        line: self.line,
                        message: format!("Unknown class: {}", class_name),
                    }
                })?;

                // Create a new unique instance ID
                let instance_id = format!(
                    "{}_{}_{}",
                    class_name,
                    self.instances.len(),
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_nanos()
                );

                // Run the init __edge__ function
                if let Some(body) = class_def.functions.get("__edge__") {
                    for stmt in body.clone() {
                        let controlflow = self.execute_statement(stmt)?;

                        match controlflow {
                            ControlFlow::Return(value) => return Ok(value),
                            _ => {}
                        }
                    }
                } else {
                    return Err(error::ParseError::UnknownFunction {
                        name: "__edge__".to_string(),
                        line: self.line,
                    });
                }

                // Create and store the instance
                let variables = HashMap::new();

                let instance = Instance {
                    variables,
                    class_name: class_name.clone(),
                };

                self.instances.insert(instance_id.clone(), instance);

                // Return the instance ID as an expression
                Ok(Expr::Instance {
                    class_name: class_name.clone(),
                    instance_id,
                })
            }
            _ => Err(error::ParseError::GeneralError {
                line: self.line,
                message: format!("Unsupported expression {:?}", expr),
            }),
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

    // pub fn execute_statements(&mut self, statements: Vec<Stmt>) -> Result<Expr, error::ParseError> {
    //     for stmt in statements {
    //         match self.execute_statement(stmt)? {
    //             ControlFlow::Return(value) => {
    //                 return Ok(value);
    //             }
    //             _ => {}
    //         }
    //     }

    //     Ok(Expr::Number(0))
    // }

    pub fn execute_user_function(
        &mut self,
        name: String,
        _args: Vec<Expr>,
    ) -> Result<Expr, error::ParseError> {
        if let Some(body) = self.functions.get(&name) {
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
        if let Some(object) = object {
            // gives the value of variables if defined, cant use this sorgy
            // let object = self.evaluate_expression(*object)?;

            match *object {
                Expr::Ident(ref obj_name) if obj_name == "self" => {
                    // Ensure we're in a method context
                    let current_instance = self.current_instance.clone().ok_or_else(|| {
                        error::ParseError::GeneralError {
                            line: self.line,
                            message: "Cannot use 'self' outside of a method context".to_string(),
                        }
                    })?;

                    // Lookup the current instance
                    let _instance = self.instances.get_mut(&current_instance).ok_or_else(|| {
                        error::ParseError::GeneralError {
                            line: self.line,
                            message: "Current instance not found".to_string(),
                        }
                    })?;

                    // TODO: needs to get the function from the class definition somehow, currently no way to reverse lookup
                    // check claude, I starred a thing there
                }
                Expr::Ident(object) => {
                    let object_name = object.clone();
                    
                    // We could evaluate expression first, but idc
                    if let Some(value) = self.variables.get(&object_name) {
                        match value {
                            Expr::Instance {
                                class_name,
                                instance_id,
                            } => {
                                let class_def = self.classes.get(class_name).ok_or_else(|| {
                                    error::ParseError::GeneralError {
                                        line: self.line,
                                        message: format!("Unknown class: {}", class_name),
                                    }
                                })?;

                                let func = class_def.functions.get(&name).ok_or_else(|| {
                                    error::ParseError::GeneralError {
                                        line: self.line,
                                        message: format!("Unknown function: {} on object {}", name, object_name),
                                    }
                                })?;

                                for stmt in func.clone() {
                                    let controlflow = self.execute_statement(stmt)?;
            
                                    match controlflow {
                                        ControlFlow::Return(value) => return Ok(value),
                                        _ => {}
                                    }
                                }

                                return Ok(Expr::Number(0));
                            }
                            _ => {
                                return Err(error::ParseError::GeneralError {
                                    line: self.line,
                                    message: "Object calls of variables are only supported for instance variables."
                                        .to_string(),
                                });
                            }
                        }
                    }

                    // If it isnt a variable, it must be a library function

                    let lib = self.libs.get_mut(&object_name).ok_or_else(|| {
                        error::ParseError::GeneralError {
                            line: self.line,
                            message: format!("Unknown library: {}", object_name),
                        }
                    })?;

                    let func = lib.functions.get(&name).ok_or_else(|| {
                        error::ParseError::GeneralError {
                            line: self.line,
                            message: format!("Unknown function: {} on object {}", name, object_name),
                        }
                    })?;

                    return func(self, args.clone());
                }
                _ => {
                    return Err(error::ParseError::GeneralError {
                        line: self.line,
                        message: "Object calls of other types than IDENT are not supported"
                            .to_string(),
                    });
                }
            }
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
