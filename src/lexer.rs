use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,
    pos: usize,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub ttype: TokenType,
    pub value: &'a str,
    pub pos: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Identifier,
    Number,
    Plus,
    Minus,
    Asterisk,
    ForwardSlash,
    Caret,
    OpeningParen,
    ClosingParen,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let chars = source.chars().peekable();
        Self {
            source,
            chars,
            pos: 0,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        use TokenType::*;

        while let Some(c) = self.chars.next_if(|c| c.is_whitespace()) {
            self.source = &self.source[c.len_utf8()..];
        }

        if let Some(c) = self.chars.next() {
            let (ttype, len) = match c {
                '+' => (Plus, 1),
                '-' => (Minus, 1),
                '*' => (Asterisk, 1),
                '/' => (ForwardSlash, 1),
                '^' => (Caret, 1),
                '(' => (OpeningParen, 1),
                ')' => (ClosingParen, 1),
                _ => {
                    if c.is_ascii_digit() || c == '.' {
                        let len = count_char_bytes_while(&mut self.chars, |c| {
                            c.is_ascii_digit() || *c == '.'
                        });
                        (Number, c.len_utf8() + len)
                    } else {
                        let len = count_char_bytes_while(&mut self.chars, |c| {
                            !(c.is_whitespace() || c.is_ascii_punctuation())
                        });
                        (Identifier, c.len_utf8() + len)
                    }
                }
            };

            let value = &self.source[..len];
            let pos = self.pos;

            self.source = &self.source[len..];
            self.pos += len;

            Some(Token { ttype, value, pos })
        } else {
            None
        }
    }
}

fn count_char_bytes_while<F>(chars: &mut Peekable<Chars>, func: F) -> usize
where
    F: FnOnce(&char) -> bool + Copy,
{
    let mut bytes = 0;

    while let Some(c) = chars.next_if(func) {
        bytes += c.len_utf8();
    }

    bytes
}
