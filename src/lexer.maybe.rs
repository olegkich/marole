
use std::{char, ops::{Index, RangeTo}};
enum Kind {
  NumberToken,
  WhitespaceToken,
  PlusToken,
  MinusToken,
  MultiplyToken,
  DivideToken,
  OpenParenToken,
  ClosedParenToken
}

struct SyntaxToken {
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

  pub fn run(&self) -> Vec<SyntaxToken> {
    let tokens: Vec<SyntaxToken> = vec!();
    
    for _ in self._text {
      let token = next_token();
      tokens.push
    }
  }

  fn current(&self) -> char {
    if self._position >= self._text.len() {return '\0';};
    return self._text.as_bytes()[self._position] as char;
  }

  fn next(&mut self) {
    self._position += 1;
  }

  fn next_token(&mut self) -> Result<SyntaxToken, String> {
    // <numbers>
    // + - * / ( )
    // <whitespace>

    let c = self.current();

    if c.is_numeric() {
      let start = self._position;

      while (c.is_numeric()) {
        self.next();
      }

      let len = start - self._position;
      let text = self._text.chars().skip(start).take(len).collect();

      return Ok(SyntaxToken {kind: Kind::NumberToken, _position: start, text});
    };

    if c.is_whitespace() {
      let start = self._position;

      while c.is_whitespace() {
        self.next();
      }

      let len = start - self._position;
      let text = self._text.chars().skip(start).take(len).collect();

      return Ok(SyntaxToken {kind: Kind::WhitespaceToken, _position: start, text});
    };

    if c.eq(&'+') {
      return Ok(SyntaxToken {kind: Kind::PlusToken, _position: self._position + 1, text: "+".to_string()});      
    };

    if c.eq(&'-') {
      return Ok(SyntaxToken {kind: Kind::MinusToken, _position: self._position + 1, text: "-".to_string()});      
    };

    if c.eq(&'*') {
      return Ok(SyntaxToken {kind: Kind::MultiplyToken, _position: self._position + 1, text: "*".to_string()});      
    };

    if c.eq(&'/') {
      return Ok(SyntaxToken {kind: Kind::DivideToken, _position: self._position + 1, text: "/".to_string()});      
    };

    if c.eq(&'(') {
      return Ok(SyntaxToken {kind: Kind::OpenParenToken, _position: self._position + 1, text: "(".to_string()});      
    };

    if c.eq(&')') {
      return Ok(SyntaxToken {kind: Kind::ClosedParenToken, _position: self._position + 1, text: ")".to_string()});      
    };
    
    
  }
}




