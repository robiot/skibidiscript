// parser.rs
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
    FunctionCall {
        name: String,
        object: Option<Box<Expr>>,
        args: Vec<Expr>,
    },
    _BinOp {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
}

#[derive(Clone, Debug)]
pub enum Stmt {
    Function {
        name: String,
        body: Vec<Stmt>,
        line: usize,
    },
    VariableAssign {
        name: String,
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
            // println!("current_token: {:?}", self.current_token);
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Stmt, error::ParseError> {
        match self.current_token {
            Token::Cookable => self.parse_function(),
            Token::Ident(_) => self.parse_variable_assign_or_expression(),
            Token::Cook => self.parse_cook_statement(),
            Token::Gyatt => self.parse_import_statement(),
            Token::Skibidi => self.parse_while(),
            Token::Suspect => self.parse_if(),
            Token::Blud => self.parse_return(),
            _ => Err(error::ParseError::UnknownUnexpectedToken {
                found: self.current_token.clone(),
                line: self.lexer.line,
            }),
        }
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
        let name = if let Token::Ident(ident) = &self.current_token {
            Ok(ident.clone())
        } else {
            Err(error::ParseError::Other("Expected identifier".into()))
        }?;

        self.next_token()?;

        if self.current_token == Token::Is {
            self.next_token()?;

            let value = self.parse_expression()?;
           
            Ok(Stmt::VariableAssign {
                name,
                value,
                line: self.lexer.line,
            })
        } else {
            let expr = self.parse_expression()?;

            Ok(Stmt::Expression {
                value: expr,
                line: self.lexer.line,
            })
        }
    }

    fn parse_expression(&mut self) -> Result<Expr, error::ParseError> {
        match self.current_token.clone() {  // Clone here to avoid borrowing `self`
            Token::Cook => {
                self.expect_token(Token::Cook)?;
                let function_call = self.parse_expression()?;
                Ok(function_call)
            }
            Token::Ident(ref ident) => {
                let mut expr = Expr::Ident(ident.clone());
                self.next_token()?; // Now you can mutate `self` safely.
    
                while self.current_token == Token::Dot {
                    self.next_token()?; // Move past the dot.
    
                    if let Token::Ident(method_name) = &self.current_token.clone() { // Clone here to avoid borrow
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
    
                            self.expect_token(Token::RightParen)?; // Now you can mutate `self` safely.
    
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
    
                    self.expect_token(Token::RightParen)?; // Now you can mutate `self` safely.
    
                    Ok(Expr::FunctionCall {
                        name: ident.clone(),
                        object: None,
                        args,
                    })
                } else {
                    Ok(expr)
                }
            }
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
        }    
    }

    fn parse_cook_statement(&mut self) -> Result<Stmt, error::ParseError> {
        // We've encountered 'cook', so advance the token.
        self.expect_token(Token::Cook)?;

        // The next token should be the identifier (function name).
        let _function_name = if let Token::Ident(ident) = &self.current_token {
            ident.clone()
        } else {
            return Err(error::ParseError::GeneralError {
                line: self.lexer.line,
                message: format!(
                    "Expected a function name after 'cook', but found: {:?}",
                    self.current_token
                ),
            });
        };

        // Parse the function call expression.
        let function_call = self.parse_expression()?; // This will handle the parsing of the function call.

        // We assume that this function call is the entire statement.
        Ok(Stmt::Expression {
            value: function_call,
            line: self.lexer.line,
        })
    }

    fn parse_import_statement(&mut self) -> Result<Stmt, error::ParseError> {
        self.expect_token(Token::Gyatt)?; // Move past 'gyatt'
    
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

    fn parse_if(&mut self) -> Result<Stmt, error::ParseError> {
        self.expect_token(Token::Suspect)?;
        self.expect_token(Token::LeftParen)?;
        let condition = self.parse_expression()?;
        self.expect_token(Token::RightParen)?;
        self.expect_token(Token::Then)?;
        let mut then_branch = Vec::new();

        while self.current_token != Token::Ick
            && self.current_token != Token::Slay
            && self.current_token != Token::EOF
        {
            then_branch.push(self.parse_statement()?);
        }

        let else_branch = if self.current_token == Token::Ick {
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
}
