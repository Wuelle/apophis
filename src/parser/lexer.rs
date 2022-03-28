use super::position::{Position, Positioned};
use super::token::Token;
use crate::debug::annotation::InputLocation;
use crate::debug::error::{Error, ErrorVariant};
use std::collections::VecDeque;
use std::iter::{Enumerate, Peekable};

/// A parser for right-linear (not sure if thats the right word) grammars
pub struct Lexer<'a> {
    buffer: &'a str,
}

impl<'a> Lexer<'a> {
    /// Create a new Lexer from an input buffer
    pub fn new(buffer: &'a str) -> Self {
        Self { buffer: buffer }
    }

    fn read_next_token(&'a self, pos: &mut usize) -> Result<Option<Positioned<Token<'a>>>, Error> {
        let token = match self.peek(pos) {
            Some(c) => match c {
                'a'..='z' | 'A'..='Z' | '_' => {
                    let ident = self
                        .consume_while(pos, |c: &char| c.is_alphanumeric() || c == &'_')
                        .unwrap();
                    let token = match *ident.as_inner() {
                        "forall" => ident.map(Token::Forall),
                        "true" => ident.map(Token::True),
                        "false" => ident.map(Token::False),
                        ident_str => ident.map(Token::Ident(ident_str)),
                    };
                    Some(token)
                }
                ' ' => self.consume(pos).map(|o| o.map(Token::Indent)),
                '=' => match self.take(pos, "=>") {
                    Ok(read) => Some(read.map(Token::Implication)),
                    Err(e) => unimplemented!(),
                },
                '(' => self.consume(pos).map(|o| o.map(Token::OpeningParen)),
                ')' => self.consume(pos).map(|o| o.map(Token::ClosingParen)),
                x => {
                    return Err(Error::new(
                        ErrorVariant::UnexpectedCharacter {
                            found: x,
                            expected: None,
                        },
                        InputLocation::Pos(*pos),
                    ));
                }
            },
            None => None,
        };

        // Following a token, there can be an arbitrary number of whitespaces
        self.consume_while(pos, |c: &char| c.is_whitespace());

        Ok(token)
    }

    pub fn tokenize(&'a self) -> Result<Vec<Positioned<Token<'a>>>, Error> {
        let mut pos = 0;
        let mut tokens = vec![];
        while let Some(token) = self.read_next_token(&mut pos)? {
            tokens.push(token)
        }

        Ok(tokens)
    }

    /// Return a single character and advance the reader position
    pub fn consume(&'a self, pos: &mut usize) -> Option<Positioned<char>> {
        let c = self.buffer.chars().nth(*pos)?;
        let res = Positioned::new(c, Position::Pos(*pos));
        *pos += 1;
        Some(res)
    }

    /// Return a single character and advance the reader position
    ///
    /// # Panic
    /// Panics if the end position is outside the buffer range
    pub fn consume_exact(&'a self, pos: &mut usize, n: usize) -> Positioned<&'a str> {
        let end = *pos + n;
        let res = Positioned::new(&self.buffer[*pos..end], Position::Span(*pos, end));
        *pos += n;
        res
    }

    /// Read as long as the read character satisfies the given predicate
    /// and advance the reader position accordingly
    pub fn consume_while<P: 'static>(
        &self,
        pos: &mut usize,
        predicate: P,
    ) -> Option<Positioned<&str>>
    where
        P: FnMut(&char) -> bool,
    {
        let len = self.buffer.chars().skip(*pos).take_while(predicate).count();
        if len == 0 {
            None
        } else {
            Some(self.consume_exact(pos, len))
        }
    }

    /// Return a single character without advancing the reader position
    pub fn peek(&self, pos: &usize) -> Option<char> {
        self.buffer.chars().nth(*pos)
    }

    /// Read exactly n characters without advancing the reader position
    pub fn peek_exact(&self, pos: usize, n: usize) -> &str {
        &self.buffer[pos..pos + n]
    }

    pub fn take(&self, pos: &mut usize, expected: &str) -> Result<Positioned<()>, Error> {
        let initial_pos = *pos;
        for expected_char in expected.chars() {
            let read = self.consume(pos);

            match read {
                Some(found) => {
                    let found_c = found.into_inner();
                    if found_c != expected_char {
                        return Err(Error::new(
                            ErrorVariant::UnexpectedCharacter {
                                found: found_c,
                                expected: Some(vec![expected_char]),
                            },
                            InputLocation::Pos(*pos),
                        ));
                    }
                }
                None => {
                    return Err(Error::new(
                        ErrorVariant::UnexpectedEndOfInput,
                        InputLocation::Pos(*pos),
                    ));
                }
            }
        }
        Ok(Positioned::new((), Position::Span(initial_pos, *pos)))
    }
}
