use std::{borrow::Cow, str::FromStr};

pub struct Lexer<'src> {
    src: &'src str,
    cursor: usize,
    line: u64,
    column: u64,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            src,
            cursor: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn next(&mut self) -> Token {
        self.skip_whitespace();

        let c = match self.next_char() {
            Some(c) => c,
            _ => return Token::eof(self.line, self.column),
        };

        match c {
            'i' => {
                if let Some(c) = self.peek_char() {
                    match c {
                        'f' => Token::simple(self.line, self.column, TokenKind::If),
                        'n' => Token::simple(self.line, self.column, TokenKind::In),
                        _ => {
                            self.retreat(1);
                            self.consume_identifier()
                        }
                    }
                } else {
                    self.retreat(1);
                    self.consume_identifier()
                }
            }
            'l' => {
                if self.next_is("oop") {
                    self.cursor += 3;
                    Token::simple(self.line, self.column, TokenKind::Loop)
                } else if self.next_is("et") {
                    self.cursor += 2;
                    Token::simple(self.line, self.column, TokenKind::Let)
                } else {
                    self.retreat(1);
                    self.consume_identifier()
                }
            }
            ':' => Token::simple(self.line, self.column, TokenKind::Colon),
            ';' => Token::simple(self.line, self.column, TokenKind::Semicolon),
            '(' => Token::simple(self.line, self.column, TokenKind::LeftParen),
            ')' => Token::simple(self.line, self.column, TokenKind::RightParen),
            '=' => Token::simple(self.line, self.column, TokenKind::Equals),
            ',' => Token::simple(self.line, self.column, TokenKind::Comma),
            c if c.is_numeric() => {
                self.retreat(1);
                self.consume_number()
            }
            _ => {
                self.retreat(1);
                self.skip_whitespace();
                self.consume_identifier()
            }
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let next = self.src[self.cursor..].chars().next();
        self.column += 1;
        self.cursor += 1;
        next
    }

    fn peek_char(&self) -> Option<char> {
        self.src[self.cursor + 1..].chars().next()
    }

    fn next_is(&self, s: &str) -> bool {
        &self.src[self.cursor..self.cursor + s.len()] == s
    }

    fn retreat(&mut self, count: usize) {
        self.cursor -= count;
    }

    fn consume_identifier(&mut self) -> Token {
        let identifier_end = self.src[self.cursor..]
            .find(|c: char| !(c.is_alphanumeric() || c == '_'))
            .unwrap_or_else(|| self.src.len());

        let identifier = Token::identifier(
            self.line,
            self.column,
            &self.src[self.cursor..self.cursor + identifier_end],
        );
        self.cursor += identifier_end;
        self.column += identifier_end as u64;

        identifier
    }

    fn consume_number(&mut self) -> Token {
        const BASES: [char; 6] = ['x', 'y', 'z', 's', 'o', 'b'];
        let number_end = self.src[self.cursor..]
            .find(|c: char| !(c.is_numeric() || BASES.contains(&c) || c == '.'))
            .unwrap_or_else(|| self.src.len());

        let possible_number = &self.src[self.cursor..self.cursor + number_end];
        self.cursor += number_end;
        self.line += number_end as u64;
        if possible_number.matches('.').count() >= 1 {
            return Token::error(
                self.line,
                self.column,
                "too many dots in floating-point literal".into(),
            );
        }

        for base in &BASES {
            if possible_number.matches(*base).count() >= 1 {
                return Token::error(
                    self.line,
                    self.column,
                    format!("Too many '{}' in integer literal", base).into(),
                );
            }
        }

        let number = Number::from_str(possible_number);
        match number {
            Ok(number) => Token::number(self.line, self.column, number),
            Err(e) => {
                return Token::error(
                    self.line,
                    self.column,
                    format!("Error when parsing integer literal: {:?}", e).into(),
                )
            }
        }
    }

    fn skip_whitespace(&mut self) {
        let mut to_skip = 0;
        let mut src_iter = self.src[self.cursor..].chars();
        while let Some(c) = src_iter.next() {
            if c.is_whitespace() {
                to_skip += 1;
                if c == '\n' {
                    self.line += 1;
                    self.column = 1;
                } else {
                    self.column += 1;
                }
            } else {
                break;
            }
        }

        self.cursor += to_skip;
    }
}

#[derive(Debug)]
pub struct Token<'lexer> {
    line: u64,
    column: u64,
    kind: TokenKind<'lexer>,
}

impl<'lexer> Token<'lexer> {
    pub fn kind(&self) -> &TokenKind<'lexer> {
        &self.kind
    }

    pub fn is_identifier(&self) -> bool {
        matches!(self.kind, TokenKind::Identifier(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self.kind, TokenKind::Number(_))
    }

    pub fn identifier_name(&self) -> Option<&'lexer str> {
        match self.kind {
            TokenKind::Identifier(ident) => Some(ident),
            _ => None,
        }
    }

    fn eof(line: u64, column: u64) -> Self {
        Self {
            line,
            column,
            kind: TokenKind::Eof,
        }
    }

    fn identifier(line: u64, column: u64, name: &'lexer str) -> Self {
        Self {
            line,
            column,
            kind: TokenKind::Identifier(name),
        }
    }

    fn number(line: u64, column: u64, number: Number) -> Self {
        Self {
            line,
            column,
            kind: TokenKind::Number(number),
        }
    }

    fn error(line: u64, column: u64, msg: Cow<'static, str>) -> Self {
        Self {
            line,
            column,
            kind: TokenKind::Error(msg),
        }
    }

    fn simple(line: u64, column: u64, kind: TokenKind<'lexer>) -> Self {
        Self { line, column, kind }
    }
}

#[derive(PartialEq, Debug)]
pub enum TokenKind<'lexer> {
    Eof,
    Error(Cow<'static, str>),

    If,
    While,
    Loop,
    For,
    In,

    Let,

    Colon,
    Semicolon,
    LeftParen,
    RightParen,
    Comma,
    Equals,

    Identifier(&'lexer str),
    Number(Number),
}

#[derive(PartialEq, Debug)]
pub enum Number {
    /// Produced by literals that could be either floats or integers, i.e. 0, 3
    Untyped(u64),
    /// Produced by literals that are definitely integers, i.e. 0s13, 0b11, 0y10
    Integer(u64),
    /// Produced by literals that are definitely floats, i.e. 3.14
    Float(f64),
}

#[derive(Debug)]
pub enum NumberParseError {
    InvalidBase,
    InvalidInteger,
}

impl FromStr for Number {
    type Err = NumberParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let float = f64::from_str(s);
        if let Ok(f) = float {
            if f == f.floor() {
                return Ok(Number::Untyped(f as u64));
            }
            return Ok(Number::Float(f));
        }

        let mut radix = 10;
        if s.starts_with("0") {
            let base = match s.chars().nth(1) {
                Some(base) => base,
                _ => return Ok(Number::Untyped(0)),
            };

            match base {
                'x' => radix = 16,
                'y' => radix = 14,
                'z' => radix = 12,
                'o' => radix = 8,
                'b' => radix = 2,
                's' => radix = 13,
                _ => return Err(NumberParseError::InvalidBase),
            }
        }

        let int = u64::from_str_radix(s, radix).map_err(|_| NumberParseError::InvalidInteger);
        Ok(Number::Integer(int?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let src = "let tuple : (i8, second: i16, i32) = (3, 4, 5);";
        let mut lexer = Lexer::new(src);
        let next = lexer.next();

        assert_eq!(next.kind(), &TokenKind::Let);

        let next = lexer.next();
        assert!(next.is_identifier());
        assert_eq!(next.identifier_name(), Some("tuple"));

        assert_eq!(lexer.next().kind(), &TokenKind::Colon);
        assert_eq!(lexer.next().kind(), &TokenKind::LeftParen);
        let next = lexer.next();
        assert!(next.is_identifier());
        assert_eq!(lexer.next().kind(), &TokenKind::Comma);
        assert!(lexer.next().is_identifier());
        assert_eq!(lexer.next().kind(), &TokenKind::Colon);
        assert!(lexer.next().is_identifier());
        assert_eq!(lexer.next().kind(), &TokenKind::Comma);
        assert!(lexer.next().is_identifier());
        assert_eq!(lexer.next().kind(), &TokenKind::RightParen);
        assert_eq!(lexer.next().kind(), &TokenKind::Equals);
        assert_eq!(lexer.next().kind(), &TokenKind::LeftParen);
        assert!(lexer.next().is_number());
        assert_eq!(lexer.next().kind(), &TokenKind::Comma);
        assert!(lexer.next().is_number());
        assert_eq!(lexer.next().kind(), &TokenKind::Comma);
        assert!(lexer.next().is_number());
        assert_eq!(lexer.next().kind(), &TokenKind::RightParen);
        assert_eq!(lexer.next().kind(), &TokenKind::Semicolon);
    }
}
