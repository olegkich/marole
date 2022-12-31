
use std::{char, ops::{Index, RangeTo}};
#[derive(Debug, PartialEq)]
enum Kind {
  NumberToken,
  WhitespaceToken,
  PlusToken,
  MinusToken,
  MultiplyToken,
  DivideToken,
  OpenParenToken,
  ClosedParenToken,
  BadToken
}

#[derive(Debug)]
pub struct SyntaxToken {
  kind: Kind,
  _position: usize,
  text: String,
}

pub struct Lexer {
  _text: String,
  _position: usize
}

impl Lexer {
  pub fn init(text: String) -> Self {
    Self {_text: text, _position: 0}
  }

  pub fn run(&mut self) -> Vec<SyntaxToken> {
    let mut tokens: Vec<SyntaxToken> = vec!();
    
    while self.current() != '\0' {
      
      let token = self.next_token();
      
      if token.kind == Kind::BadToken {
        panic!("{}", "Bad token type.")
      }

      tokens.push(token);
    };

    tokens
  }

  fn current(&self) -> char {
    if self._position >= self._text.len() {return '\0';};
    return self._text.as_bytes()[self._position] as char;
  }

  fn next(&mut self) {
    self._position = self._position + 1;
  }

  fn next_token(&mut self) -> SyntaxToken {
    // <numbers>
    // + - * / ( )
    // <whitespace>

    match self.current() {
      _ if self.current().is_numeric() => {
        
        let start = self._position;

        while self.current().is_numeric() {
          self.next();
        }

        let len = self._position.saturating_sub(start);
        let text: String = self._text.chars().skip(start).take(len).collect();
  
        return SyntaxToken {kind: Kind::NumberToken, _position: start, text: text};}
      _ if self.current().is_whitespace() => {
        
        let start = self._position;

        while self.current().is_whitespace() {
          self.next();
        }
  
        let len = start.saturating_sub(self._position);
        let text = self._text.chars().skip(start).take(len).collect();
  
        return SyntaxToken {kind: Kind::WhitespaceToken, _position: start, text};}

      _ if self.current().eq(&'+') => {self.next();
        return SyntaxToken {kind: Kind::PlusToken, _position: self._position - 1, text: "+".to_string()}},

      _ if self.current().eq(&'-') => {self.next();
        return SyntaxToken {kind: Kind::MinusToken, _position: self._position - 1, text: "-".to_string()}},

      _ if self.current().eq(&'*') => {self.next();
        return SyntaxToken {kind: Kind::MultiplyToken, _position: self._position - 1, text: "*".to_string()}},

      _ if self.current().eq(&'/') => {
        self.next();
        return SyntaxToken {kind: Kind::DivideToken, _position: self._position - 1, text: "/".to_string()}},

      _ if self.current().eq(&'(') => {
        self.next();
        return SyntaxToken {kind: Kind::OpenParenToken, _position: self._position - 1, text: "(".to_string()}
      },

      _ if self.current().eq(&')') => {self.next();
        return SyntaxToken {kind: Kind::ClosedParenToken, _position: self._position - 1, text: ")".to_string()}},

      _ => return SyntaxToken {kind: Kind::BadToken, _position: 0, text: "BAD".to_string()}
    }
  }
}






// if self.current().eq(&'+') {
//   return (SyntaxToken {kind: Kind::PlusToken, _position: self._position + 1, text: "+".to_string()});      
// };

// if self.current().eq(&'-') {
//   return (SyntaxToken {kind: Kind::MinusToken, _position: self._position + 1, text: "-".to_string()});      
// };

// if self.current().eq(&'*') {
//   return (SyntaxToken {kind: Kind::MultiplyToken, _position: self._position + 1, text: "*".to_string()});      
// };

// if self.current().eq(&'/') {
//   return (SyntaxToken {kind: Kind::DivideToken, _position: self._position + 1, text: "/".to_string()});      
// };

// if self.current().eq(&'(') {
//   return (SyntaxToken {kind: Kind::OpenParenToken, _position: self._position + 1, text: "(".to_string()});      
// };

// if self.current().eq(&')') {
//   return (SyntaxToken {kind: Kind::ClosedParenToken, _position: self._position + 1, text: ")".to_string()});      
// };