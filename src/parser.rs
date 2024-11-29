// parser.rs

// todo: (refactor) use a shared function for running function statements and checking control flow.

use crate::{
    error,
    lexer::{Lexer, Token},
};

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Ident(String),
    Number(i64),
    StringLiteral(String),
    Boolean(bool),
    // None,
    List(Vec<Expr>),
    FunctionCall {
        name: String,
        object: Option<Box<Expr>>,
        args: Vec<Expr>,
    },
    BinOp {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    ObjectValue {
        object: Option<Box<Expr>>,
        name: String,
    },
    NewInstance {
        class_name: String,
        args: Vec<Expr>,
    },
    Instance {
        class_name: String,
        instance_id: String,
    }
}

#[derive(Clone, Debug)]
pub enum Stmt {
    Class {
        name: String,
        functions: Vec<Stmt>,
        line: usize,
    },
    Function {
        name: String,
        body: Vec<Stmt>,
        line: usize,
    },
    VariableAssign {
        name: String,
        object: Option<Box<Expr>>,
        value: Expr,
        line: usize,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
        line: usize,
    },
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
        line: usize,
    },
    Expression {
        value: Expr,
        line: usize,
    },
    Return {
        value: Expr,
        line: usize,
    },
    Import {
        library: String,
        line: usize,
    },
    Continue {
        line: usize,
    },
    ForLoop {
        iterator: String,
        collection: Expr,
        body: Vec<Stmt>,
        line: usize,
    },
}

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Result<Self, error::ParseError> {
        let current_token = lexer.next_token()?;

        Ok(Parser {
            lexer,
            current_token,
        })
    }

    fn next_token(&mut self) -> Result<(), error::ParseError> {
        self.current_token = self.lexer.next_token()?;

        Ok(())
    }

    fn expect_token(&mut self, expected: Token) -> Result<(), error::ParseError> {
        if self.current_token == expected {
            self.next_token()?;

            Ok(())
        } else {
            Err(error::ParseError::UnexpectedToken {
                expected: expected.clone(),
                found: self.current_token.clone(),
                line: self.lexer.line,
            })
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, error::ParseError> {
        let mut statements = Vec::new();
        while self.current_token != Token::EOF {
            // ...debug

            let stmt = self.parse_statement()?;
            statements.push(stmt);

            // println!("current token: {:#?}", statements);
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Stmt, error::ParseError> {
        match self.current_token {
            Token::Pookie => self.parse_class(),
            Token::Cookable => self.parse_function(),
            Token::Ident(_) => self.parse_variable_assign_or_expression(),
            Token::Cook => self.parse_cook_statement(),
            Token::Gyatt => self.parse_import_statement(),
            Token::Skibidi => self.parse_while(),
            Token::Goon => self.parse_for_loop(),
            Token::Suspect => self.parse_if(),
            Token::Blud => self.parse_return(),
            Token::Ghost => self.parse_continue(),
            _ => Err(error::ParseError::UnknownUnexpectedToken {
                found: self.current_token.clone(),
                line: self.lexer.line,
            }),
        }
    }

    fn parse_class(&mut self) -> Result<Stmt, error::ParseError> {
        self.expect_token(Token::Pookie)?;

        let name = if let Token::Ident(ident) = &self.current_token {
            ident.clone()
        } else {
            return Err(error::ParseError::GeneralError {
                line: self.lexer.line,
                message: "Expected class name".to_string(),
            });
        };

        self.next_token()?;
        self.expect_token(Token::LeftParen)?;
        self.expect_token(Token::RightParen)?;

        let mut functions = Vec::new();
        let mut has_init = false;

        while self.current_token != Token::Slay && self.current_token != Token::EOF {
            let method = self.parse_function()?;
            if let Stmt::Function {
                name: ref method_name,
                ..
            } = method
            {
                if method_name == "__edge__" {
                    has_init = true;
                }
            }
            functions.push(method);
        }

        if !has_init {
            return Err(error::ParseError::GeneralError {
                line: self.lexer.line,
                message: "Class must have an __edge__ method".to_string(),
            });
        }

        self.expect_token(Token::Slay)?;

        Ok(Stmt::Class {
            name,
            functions,
            line: self.lexer.line,
        })
    }

    fn parse_function(&mut self) -> Result<Stmt, error::ParseError> {
        self.expect_token(Token::Cookable)?;
        let name = if let Token::Ident(ident) = &self.current_token {
            Ok(ident.clone())
        } else {
            Err(error::ParseError::GeneralError {
                line: self.lexer.line,
                message: format!(
                    "found token: {:?}, expected function name",
                    self.current_token.clone()
                ),
            })
        }?;

        self.next_token()?;
        self.expect_token(Token::LeftParen)?;
        self.expect_token(Token::RightParen)?;

        let mut body = Vec::new();
        while self.current_token != Token::Slay && self.current_token != Token::EOF {
            body.push(self.parse_statement()?);
        }

        self.expect_token(Token::Slay)?;

        Ok(Stmt::Function {
            name,
            body,
            line: self.lexer.line,
        })
    }

    fn parse_variable_assign_or_expression(&mut self) -> Result<Stmt, error::ParseError> {
        let (object, name) = match &self.current_token {
            Token::Ident(ident) => {
                let name = ident.clone();

                self.next_token()?;

                // Expect a dot after 'self'
                if self.current_token == Token::Dot {
                    self.next_token()?; // Move past dot

                    // Get the property name
                    let property_name = if let Token::Ident(ident) = &self.current_token {
                        ident.clone()
                    } else {
                        return Err(error::ParseError::GeneralError {
                            line: self.lexer.line,
                            message: "Expected identifier after 'self.'".to_string(),
                        });
                    };

                    self.next_token()?;

                    // if let Token::Ident(variable_name) = &self.current_token.clone() {
                    //     self.next_token()?;

                    //     // Handle function calls
                    //     expr = Expr::ObjectValue {
                    //         name: variable_name.clone(),
                    //         object: Some(Box::new(expr)),
                    //     };
                    // } else {
                    //     return Err(error::ParseError::UnexpectedToken {
                    //         line: self.lexer.line,
                    //         expected: Token::Ident("variable name after .".into()),
                    //         found: self.current_token.clone(),
                    //     });
                    // }

                    (Some(Box::new(Expr::Ident(name))), property_name)
                } else {
                    (None, name)
                }
            }
            _ => return Err(error::ParseError::Other("Expected identifier".into())),
        };

        if self.current_token == Token::Is {
            self.next_token()?;

            // 1HERE: FAULT
            let value = self.parse_expression()?;

            match object {
                Some(obj) => Ok(Stmt::VariableAssign {
                    name,
                    object: Some(obj),
                    value,
                    line: self.lexer.line,
                }),
                None => Ok(Stmt::VariableAssign {
                    name,
                    object: None,
                    value,
                    line: self.lexer.line,
                }),
            }
        } else {
            let expr = self.parse_expression()?;

            Ok(Stmt::Expression {
                value: expr,
                line: self.lexer.line,
            })
        }
    }

    fn parse_expression(&mut self) -> Result<Expr, error::ParseError> {
        self.parse_expression_with_precedence(0)
        // self.parse_primary()
    }

    fn parse_expression_with_precedence(
        &mut self,
        min_precedence: u8,
    ) -> Result<Expr, error::ParseError> {
        // Start by parsing the primary expression (the base)
        let mut left_expr = self.parse_primary()?;

        // While the current token is an operator and its precedence is higher than min_precedence
        while let Some(op_precedence) = self.get_precedence(&self.current_token) {
            if op_precedence < min_precedence {
                break;
            }

            // Clone the operator token and move past it
            let op_token = self.current_token.clone();

            self.next_token()?;

            // Parse the right-hand side expression with the appropriate precedence
            let right_expr = self.parse_expression_with_precedence(op_precedence + 1)?;

            // Combine left and right expressions into a BinOp node
            left_expr = Expr::BinOp {
                left: Box::new(left_expr),
                op: op_token,
                right: Box::new(right_expr),
            };
        }

        Ok(left_expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, error::ParseError> {
        let thing = match self.current_token.clone() {
            Token::Mew => {
                self.next_token()?;
                if let Token::Ident(class_name) = &self.current_token {
                    let class_name = class_name.clone();
                    self.next_token()?;
                    self.expect_token(Token::LeftParen)?;

                    let mut args = Vec::new();
                    while self.current_token != Token::RightParen {
                        args.push(self.parse_expression()?);
                        if self.current_token == Token::Comma {
                            self.next_token()?;
                        }
                    }

                    self.expect_token(Token::RightParen)?;
                    Ok(Expr::NewInstance { class_name, args })
                } else {
                    Err(error::ParseError::GeneralError {
                        line: self.lexer.line,
                        message: "Expected class name after 'new'".to_string(),
                    })
                }
            }
            Token::Minus => {
                // Move past the minus sign
                self.next_token()?;

                // Parse the next expression as the value of the negative number
                let value = self.parse_primary()?;

                match value {
                    Expr::Number(num) => Ok(Expr::Number(-num)),
                    _ => Err(error::ParseError::GeneralError {
                        line: self.lexer.line,
                        message: format!("Expected a number after '-', found {:?}", value),
                    }),
                }
            }
            Token::LeftBracket => {
                self.next_token()?;

                let mut elements = Vec::new();

                while self.current_token != Token::RightBracket {
                    let element = self.parse_expression()?;
                    elements.push(element);

                    // If the next token is a comma, skip it
                    if self.current_token == Token::Comma {
                        self.next_token()?;
                    } else {
                        break;
                    }
                }

                self.expect_token(Token::RightBracket)?;

                Ok(Expr::List(elements))
            }
            // Clone here to avoid borrowing self
            Token::Cook => {
                self.expect_token(Token::Cook)?;
                let function_call = self.parse_expression()?;
                Ok(function_call)
            }
            Token::Ident(ref ident) => {
                // Handle idents with dots
                let mut expr = Expr::Ident(ident.clone());
                self.next_token()?;
            
                // Handle dot notation (object functions/properties)
                while self.current_token == Token::Dot {
                    self.next_token()?;
            
                    if let Token::Ident(objectname) = &self.current_token.clone() {
                        self.next_token()?;
            
                        // Handle function calls
                        if self.current_token == Token::LeftParen {
                            self.next_token()?; // Move past '('.
                            let mut args = Vec::new();
            
                            if self.current_token != Token::RightParen {
                                args.push(self.parse_expression()?);
                                while self.current_token == Token::Comma {
                                    self.next_token()?;
                                    args.push(self.parse_expression()?);
                                }
                            }
            
                            self.expect_token(Token::RightParen)?;
            
                            expr = Expr::FunctionCall {
                                name: objectname.clone(),
                                object: Some(Box::new(expr)),
                                args,
                            };
                        } else {
                            expr = Expr::ObjectValue {
                                name: objectname.clone(),
                                object: Some(Box::new(expr)),
                            };
                        }
                    } else {
                        return Err(error::ParseError::UnexpectedToken {
                            line: self.lexer.line,
                            expected: Token::Ident("variable name after .".into()),
                            found: self.current_token.clone(),
                        });
                    }
                }
            
                // Add this block to handle standalone function calls
                if self.current_token == Token::LeftParen {
                    self.next_token()?; // Move past '('.
                    let mut args = Vec::new();
            
                    if self.current_token != Token::RightParen {
                        args.push(self.parse_expression()?);
                        while self.current_token == Token::Comma {
                            self.next_token()?;
                            args.push(self.parse_expression()?);
                        }
                    }
            
                    self.expect_token(Token::RightParen)?;
            
                    expr = Expr::FunctionCall {
                        name: ident.clone(),
                        object: None,
                        args,
                    };
                }
            
                Ok(expr)
            },
            Token::Number(num) => {
                let value = num;
                self.next_token()?;

                Ok(Expr::Number(value))
            }
            Token::StringLiteral(ref string) => {
                let value = string.clone();
                self.next_token()?;

                Ok(Expr::StringLiteral(value))
            }
            Token::Sigma => {
                self.next_token()?;

                Ok(Expr::Boolean(true))
            }
            Token::Ohio => {
                self.next_token()?;

                Ok(Expr::Boolean(false))
            }
            _ => Err(error::ParseError::UnknownUnexpectedToken {
                found: self.current_token.clone(),
                line: self.lexer.line,
            }),
        };

        thing
    }

    fn get_precedence(&self, token: &Token) -> Option<u8> {
        match token {
            Token::Plus | Token::Minus => Some(1),
            Token::Star | Token::Slash => Some(2),
            Token::Rizz => Some(0), // Equality operator
            Token::GreaterThan | Token::LessThan => Some(1), // Relational operators
            _ => None,
        }
    }

    fn parse_cook_statement(&mut self) -> Result<Stmt, error::ParseError> {
        // We've encountered 'cook', so advance the token.
        self.expect_token(Token::Cook)?;

        // The next token should be the identifier (function name).
        let ident = if let Token::Ident(ident) = &self.current_token {
            ident.clone()
        } else {
            return Err(error::ParseError::GeneralError {
                line: self.lexer.line,
                message: format!(
                    "Expected a function or object name after 'cook', but found: {:?}",
                    self.current_token
                ),
            });
        };

        // Parse the function call expression.
        // let function_call = self.parse_expression()?; // This will handle the parsing of the function call.

        let function_call = {
            let mut expr = Expr::Ident(ident.clone());
            self.next_token()?; // Now you can mutate self safely.

            while self.current_token == Token::Dot {
                self.next_token()?; // Move past the dot.

                if let Token::Ident(method_name) = &self.current_token.clone() {
                    // Clone here to avoid borrow
                    self.next_token()?; // Move past the method name.

                    // Handle function calls
                    if self.current_token == Token::LeftParen {
                        self.next_token()?; // Move past '('.
                        let mut args = Vec::new();

                        if self.current_token != Token::RightParen {
                            args.push(self.parse_expression()?);
                            while self.current_token == Token::Comma {
                                self.next_token()?;
                                args.push(self.parse_expression()?);
                            }
                        }

                        self.expect_token(Token::RightParen)?; // Now you can mutate self safely.

                        expr = Expr::FunctionCall {
                            name: method_name.clone(),
                            object: Some(Box::new(expr)),
                            args,
                        };
                    } else {
                        return Err(error::ParseError::UnexpectedToken {
                            line: self.lexer.line,
                            expected: Token::LeftParen,
                            found: self.current_token.clone(),
                        });
                    }
                } else {
                    return Err(error::ParseError::UnexpectedToken {
                        line: self.lexer.line,
                        expected: Token::Ident("method name after .".into()),
                        found: self.current_token.clone(),
                    });
                }
            }

            // Handle regular function calls
            if self.current_token == Token::LeftParen {
                self.next_token()?; // Move past '('.
                let mut args = Vec::new();

                if self.current_token != Token::RightParen {
                    args.push(self.parse_expression()?);
                    while self.current_token == Token::Comma {
                        self.next_token()?;
                        args.push(self.parse_expression()?);
                    }
                }

                self.expect_token(Token::RightParen)?;

                Ok(Expr::FunctionCall {
                    name: ident.clone(),
                    object: None,
                    args,
                })
            } else {
                Ok(expr)
            }
        }?;

        // We assume that this function call is the entire statement.
        Ok(Stmt::Expression {
            value: function_call,
            line: self.lexer.line,
        })
    }

    fn parse_import_statement(&mut self) -> Result<Stmt, error::ParseError> {
        self.expect_token(Token::Gyatt)?;

        let lib_name = if let Token::Ident(ident) = &self.current_token {
            ident.clone()
        } else {
            return Err(error::ParseError::GeneralError {
                line: self.lexer.line,
                message: format!(
                    "Expected a library name after 'gyatt', found {:?}",
                    self.current_token
                ),
            });
        };

        self.next_token()?; // Move past the library name

        Ok(Stmt::Import {
            library: lib_name,
            line: self.lexer.line,
        })
    }

    fn parse_while(&mut self) -> Result<Stmt, error::ParseError> {
        self.expect_token(Token::Skibidi)?;
        self.expect_token(Token::LeftParen)?;

        let condition = self.parse_expression()?;

        self.expect_token(Token::RightParen)?;

        self.expect_token(Token::Do)?;

        let mut body = Vec::new();

        while self.current_token != Token::Slay && self.current_token != Token::EOF {
            body.push(self.parse_statement()?);
        }

        self.expect_token(Token::Slay)?;

        Ok(Stmt::While {
            condition,
            body,
            line: self.lexer.line,
        })
    }

    fn parse_for_loop(&mut self) -> Result<Stmt, error::ParseError> {
        self.next_token()?; // Move past 'for'
        self.expect_token(Token::LeftParen)?;

        let iterator = if let Token::Ident(name) = &self.current_token {
            name.clone()
        } else {
            return Err(error::ParseError::GeneralError {
                line: self.lexer.line,
                message: "Expected iterator variable name".to_string(),
            });
        };

        self.next_token()?;
        self.expect_token(Token::In)?;

        let collection = self.parse_expression()?;

        // error if collection is not a list
        if let Expr::List(_) = collection {
        } else {
            return Err(error::ParseError::GeneralError {
                line: self.lexer.line,
                message: "Expected list after 'in'".to_string(),
            });
        }

        self.expect_token(Token::RightParen)?;
        self.expect_token(Token::Do)?;

        let mut body = Vec::new();
        while self.current_token != Token::Slay && self.current_token != Token::EOF {
            body.push(self.parse_statement()?);
        }

        self.expect_token(Token::Slay)?;

        Ok(Stmt::ForLoop {
            iterator,
            collection,
            body,
            line: self.lexer.line,
        })
    }

    fn parse_if(&mut self) -> Result<Stmt, error::ParseError> {
        self.expect_token(Token::Suspect)?;
        self.expect_token(Token::LeftParen)?;

        let condition = self.parse_expression()?;

        self.expect_token(Token::RightParen)?;

        self.expect_token(Token::Then)?;

        let mut then_branch = Vec::new();

        while self.current_token != Token::Cap
            && self.current_token != Token::Slay
            && self.current_token != Token::EOF
        {
            then_branch.push(self.parse_statement()?);
        }

        let else_branch = if self.current_token == Token::Cap {
            self.next_token()?;
            let mut else_stmts = Vec::new();
            while self.current_token != Token::Slay && self.current_token != Token::EOF {
                else_stmts.push(self.parse_statement()?);
            }
            Some(else_stmts)
        } else {
            None
        };

        self.expect_token(Token::Slay)?;

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
            line: self.lexer.line,
        })
    }

    fn parse_return(&mut self) -> Result<Stmt, error::ParseError> {
        self.expect_token(Token::Blud)?;

        let expr = self.parse_expression()?;

        Ok(Stmt::Return {
            value: expr,
            line: self.lexer.line,
        })
    }

    fn parse_continue(&mut self) -> Result<Stmt, error::ParseError> {
        self.expect_token(Token::Ghost)?;

        Ok(Stmt::Continue {
            line: self.lexer.line,
        })
    }
}
