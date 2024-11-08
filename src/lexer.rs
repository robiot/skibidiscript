// lexer.rs
use std::collections::HashMap;

use crate::error;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Core
    Cookable,
    Cook,
    Is,
    Skibidi,
    Sigma,
    NewLine,
    Ohio,
    Suspect,
    Then,
    Do,
    Slay,
    Rizz,
    Blud,
    Ghost,
    Ick,
    Gyatt,
    Goon,
    In,

    // Classes
    Pookie,
    SelfKeyword,
    New,

    // General
    Ident(String),
    Number(i64),
    StringLiteral(String),
    
    // Args
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Comma,
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    GreaterThan,
    LessThan,
    Dot,
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    keywords: HashMap<String, Token>,
    pub line: usize, // Track the current line number
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("cookable".into(), Token::Cookable);
        keywords.insert("cook".into(), Token::Cook);
        keywords.insert("is".into(), Token::Is);
        keywords.insert("skibidi".into(), Token::Skibidi);
        keywords.insert("sigma".into(), Token::Sigma);
        keywords.insert("ohio".into(), Token::Ohio);
        keywords.insert("sus".into(), Token::Suspect);
        keywords.insert("then".into(), Token::Then);
        keywords.insert("do".into(), Token::Do);
        keywords.insert("slay".into(), Token::Slay);
        keywords.insert("rizz".into(), Token::Rizz);
        keywords.insert("blud".into(), Token::Blud);
        keywords.insert("ghost".into(), Token::Ghost);
        keywords.insert("ick".into(), Token::Ick);
        keywords.insert("gyatt".into(), Token::Gyatt);
        keywords.insert("goon".into(), Token::Goon);
        keywords.insert("in".into(), Token::In);

        keywords.insert("pookie".into(), Token::Pookie);
        // keywords.insert("self".into(), Token::SelfKeyword);
        keywords.insert("new".into(), Token::New);


        Lexer {
            input: input.chars().collect(),
            position: 0,
            keywords,
            line: 1,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            self.position += 1;
            Some(self.input[self.position - 1])
        } else {
            None
        }
    }

    fn peek_char(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    fn peek_next_char(&self) -> Option<char> {
        // Temporarily advance the position by one character to peek ahead
        let next_position = self.position + 1;

        if next_position < self.input.len() {
            Some(self.input[next_position])
        } else {
            None
        }
    }

    pub fn next_token(&mut self) -> Result<Token, error::ParseError> {
        self.skip_whitespace();

        match self.next_char() {
            Some('"') => Ok(self.read_string()),
            Some('(') => Ok(Token::LeftParen),
            Some(')') => Ok(Token::RightParen),
            Some('[') => Ok(Token::LeftBracket),
            Some(']') => Ok(Token::RightBracket),
            Some(',') => Ok(Token::Comma),
            Some('.') => Ok(Token::Dot),
            Some('\n') => Ok(Token::NewLine),
            // meth operators
            Some('+') => Ok(Token::Plus),
            Some('-') => Ok(Token::Minus),
            Some('*') => Ok(Token::Star),
            Some('/') => Ok(Token::Slash),
            Some('>') => Ok(Token::GreaterThan),
            Some('<') => Ok(Token::LessThan),
            Some(ch) if ch.is_alphabetic() || ch == '_' => Ok(self.read_identifier_or_keyword(ch)),
            Some(ch) if ch.is_digit(10) => Ok(self.read_number(ch)),
            None => Ok(Token::EOF),
            Some(ch) => match ch {
                _ => Err(error::ParseError::LexerUnexpectedChar {
                    found: ch.to_string(),
                    line: self.line,
                }),
            },
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() {
                if ch == '\n' {
                    self.line += 1; // Increment the line count for newlines
                }
                self.next_char(); // Skip whitespace characters
            } else if ch == '-' {
                // Peek at the next character
                if let Some(next_ch) = self.peek_next_char() {
                    if next_ch == '-' {
                        self.next_char(); // Skip the '-' character
                        self.next_char(); // Skip the next '-' character
                        self.skip_comment(); // Skip the rest of the comment
                    } else {
                        // It's a minus sign, not the start of a comment
                        break;
                    }
                } else {
                    // It's a minus sign, not the start of a comment
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        while let Some(ch) = self.next_char() {
            if ch == '\n' {
                self.line += 1;
                break;
            }
        }
    }

    fn read_string(&mut self) -> Token {
        let mut result = String::new();
        while let Some(ch) = self.next_char() {
            if ch == '"' {
                break;
            } else {
                result.push(ch);
            }
        }
        Token::StringLiteral(result)
    }

    fn read_identifier_or_keyword(&mut self, first_char: char) -> Token {
        let mut result = String::new();
        result.push(first_char);

        while let Some(ch) = self.peek_char() {
            if ch.is_alphanumeric() || ch == '_' {
                result.push(ch);
                self.next_char();
            } else {
                break;
            }
        }

        self.keywords
            .get(&result)
            .cloned()
            .unwrap_or(Token::Ident(result))
    }

    fn read_number(&mut self, first_digit: char) -> Token {
        let mut number = first_digit.to_string();
        while let Some(ch) = self.peek_char() {
            if ch.is_digit(10) {
                number.push(ch);
                self.next_char();
            } else {
                break;
            }
        }
        Token::Number(number.parse().unwrap())
    }
}
