// lexer.rs
use std::collections::HashMap;

use crate::error;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Cookable,
    Cook,
    Is,
    Skibbity,
    Sigma,
    Ohio,
    // Nerd,
    // RandInt,
    Suspect,
    Then,
    Slay,
    Rizz,
    Blud,
    Ick,
    Ident(String),
    Number(i64),
    StringLiteral(String),
    LeftParen,
    RightParen,
    Comma,
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
        keywords.insert("skibbity".into(), Token::Skibbity);
        keywords.insert("sigma".into(), Token::Sigma);
        keywords.insert("ohio".into(), Token::Ohio);
        // keywords.insert("nerd".into(), Token::Nerd);
        // keywords.insert("randInt".into(), Token::RandInt);
        keywords.insert("suspect".into(), Token::Suspect);
        keywords.insert("then".into(), Token::Then);
        keywords.insert("slay".into(), Token::Slay);
        keywords.insert("rizz".into(), Token::Rizz);
        keywords.insert("blud".into(), Token::Blud);
        keywords.insert("ick".into(), Token::Ick);

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

    pub fn next_token(&mut self) -> Result<Token, error::ParseError> {
        self.skip_whitespace();

        println!("next_token: {:?}", self.peek_char());
        match self.next_char() {
            Some('\n') => {
                println!("Newline detected");
                self.line += 1;  // Increment line number for each newline character
                self.next_token() // Recursively call to get the next meaningful token
            },
            Some('"') => Ok(self.read_string()),
            Some('(') => Ok(Token::LeftParen),
            Some(')') => Ok(Token::RightParen),
            Some(',') => Ok(Token::Comma),
            Some(ch) if ch.is_alphabetic() => Ok(self.read_identifier_or_keyword(ch)),
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
                self.next_char();
            } else {
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
