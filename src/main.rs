#![feature(let_chains, once_cell)]

mod compute;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;

use anyhow::Result;
use num::ToPrimitive;

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
                    let s = if result.is_integer() {
                        result.to_string()
                    } else {
                        result.to_f64().unwrap().to_string()
                    };
                    println!("    = {s}")
                }
                Err(error) => println!("    {:#}", error),
            }
        }
    }

    Ok(())
}
