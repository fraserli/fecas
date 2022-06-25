#![feature(let_chains, once_cell)]

mod compute;
mod lexer;
mod parser;

use fraction::{BigInt, DynaFraction};
use lexer::Lexer;
use parser::Parser;

use anyhow::Result;

use std::io::Write;

fn main() -> Result<()> {
    loop {
        print!("> ");
        std::io::stdout().flush()?;

        let mut line = String::new();
        let bytes = std::io::stdin().read_line(&mut line)?;

        if bytes == 0 {
            println!("\nExiting...");
            break;
        } else if !line.trim().is_empty() {
            let lexer = Lexer::new(&line);
            let mut parser = Parser::new(lexer);

            match parser.parse() {
                Ok(expr) => {
                    let result = expr.compute();
                    let a = DynaFraction::from(BigInt::from(10).pow(24));
                    println!("    = {:.24}", (result * a.clone()).round() / a);
                }
                Err(error) => println!("    {:#}", error),
            }
        }
    }

    Ok(())
}
