use crate::ast::{Token};

use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug)]
pub struct Lexer<'a> {
  source: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
  pub fn new(source: &'a str) -> Self {
    Lexer {
      source: source.chars().peekable(),
    }
  }

  pub fn set_source(&mut self, source: &'a str) {
    self.source = source.chars().peekable();
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Token;

  fn next(&mut self) -> Option<Token> {
    let c = self.source.next();
    match c {
      Some('0'...'9') => {
        let mut num = c.unwrap().to_string();
        // FIX THIS AT SOME POINT.
        for ch in self.source.to_owned().take_while(|ch| ch.is_numeric()) {
          num.push(ch);
        }
        Some(Token::Num(num.parse::<i32>().unwrap()))
      },
      Some('+')      => Some(Token::Add),
      Some('-')      => Some(Token::Sub),
      Some('*')      => Some(Token::Mul),
      Some('/')      => Some(Token::Div),
      Some('(')      => Some(Token::LParen),
      Some(')')      => Some(Token::RParen),
      None           => Some(Token::Eof),
      Some(' ')      => self.next(),
      Some('\n')     => self.next(),
      Some('\t')     => self.next(),
      Some(_)        => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_num_token() {
    let mut lexer = Lexer::new("345671");
    assert_eq!(Token::Num(345671), lexer.next().unwrap());
  }

  #[test]
  fn test_add() {
    let mut lexer = Lexer::new("+");
    assert_eq!(Token::Add, lexer.next().unwrap());
  }

  #[test]
  fn test_sub() {
    let mut lexer = Lexer::new("-");
    assert_eq!(Token::Sub, lexer.next().unwrap());
  }

  #[test]
  fn test_mul() {
    let mut lexer = Lexer::new("*");
    assert_eq!(Token::Mul, lexer.next().unwrap());
  }

  #[test]
  fn test_div() {
    let mut lexer = Lexer::new("/");
    assert_eq!(Token::Div, lexer.next().unwrap());
  }

  #[test]
  fn test_parens() {
    let mut lexer = Lexer::new("()");
    assert_eq!(Token::LParen, lexer.next().unwrap());
    assert_eq!(Token::RParen, lexer.next().unwrap());
  }
}