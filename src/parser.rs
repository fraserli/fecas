use crate::lexer::{Lexer, Token, TokenType};

use anyhow::{anyhow, bail, Context, Result};
use num::BigRational;

use std::iter::Peekable;
use std::ops::Neg;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

#[derive(Debug)]
pub struct Expression {
    pub lhs: Atom,
    pub operations: Vec<(Operator, Box<Expression>)>,
}

#[derive(Debug)]
pub enum Atom {
    Number(BigRational),
    Expr(Box<Expression>),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Power,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let lexer = lexer.peekable();
        Self { lexer }
    }

    pub fn parse(&mut self) -> Result<Expression> {
        let expr = self.parse_expression(0)?;

        if let Some(token) = self.lexer.next() {
            bail!("trailing input: {}", token.value);
        }

        Ok(expr)
    }

    fn eat(&mut self, ttype: TokenType) -> Result<Token> {
        if let Some(token) = self.lexer.next() {
            if token.ttype == ttype {
                Ok(token)
            } else {
                Err(anyhow!(
                    "expected {:?} but got {:?} '{}' at byte {}",
                    ttype,
                    token.ttype,
                    token.value,
                    token.pos,
                ))
            }
        } else {
            Err(anyhow!("unexpected EOF"))
        }
    }

    fn try_eat(&mut self, ttype: TokenType) -> Option<Token> {
        self.lexer.next_if(|t| t.ttype == ttype)
    }

    fn parse_expression(&mut self, min_prec: usize) -> Result<Expression> {
        let mut operations = Vec::new();

        let lhs = self.parse_atom()?;

        while let Some(token) = self.lexer.peek() && let Some(op) = token.to_binary_operator() {
            let (prec, assoc) = op.properties();

            if prec < min_prec {
                break;
            } else {
                self.lexer.next();
            }

            let next_min_prec = match assoc {
                Associativity::Left => prec + 1,
                Associativity::Right => prec,
            };

            let rhs = Box::new(self.parse_expression(next_min_prec)?);

            operations.push((op, rhs));
        }

        Ok(Expression { lhs, operations })
    }

    fn parse_atom(&mut self) -> Result<Atom> {
        if self.try_eat(TokenType::Plus).is_some() {
            Ok(self.parse_atom()?)
        } else if self.try_eat(TokenType::Minus).is_some() {
            Ok(-self.parse_atom()?)
        } else if self.try_eat(TokenType::OpeningParen).is_some() {
            let expr = self.parse_expression(0)?;
            self.eat(TokenType::ClosingParen)?;
            Ok(Atom::Expr(Box::new(expr)))
        } else {
            let number = self.parse_number()?;
            Ok(Atom::Number(number))
        }
    }

    fn parse_number(&mut self) -> Result<BigRational> {
        let token = self.eat(TokenType::Number)?;
        let f: f64 = token
            .value
            .parse()
            .with_context(|| format!("invalid number literal: '{}'", token.value))?;
        Ok(BigRational::from_float(f).unwrap())
    }
}

enum Associativity {
    Left,
    Right,
}

impl Neg for Expression {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let lhs = -self.lhs;
        let operations = self
            .operations
            .into_iter()
            .map(|(op, expr)| (op, Box::new(-*expr)))
            .collect();

        Expression { lhs, operations }
    }
}

impl Neg for Atom {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Number(number) => Self::Number(-number),
            Self::Expr(expression) => Self::Expr(Box::new(-*expression)),
        }
    }
}

impl Operator {
    fn properties(&self) -> (usize, Associativity) {
        use Associativity::*;
        use Operator::*;
        match self {
            Addition | Subtraction => (0, Left),
            Multiplication => (1, Left),
            Division => (1, Left),
            Power => (2, Right),
        }
    }
}

impl Token<'_> {
    fn to_binary_operator(&self) -> Option<Operator> {
        use Operator::*;
        use TokenType::*;
        match self.ttype {
            Plus => Some(Addition),
            Minus => Some(Subtraction),
            Asterisk => Some(Multiplication),
            ForwardSlash => Some(Division),
            Caret => Some(Power),
            _ => None,
        }
    }
}
