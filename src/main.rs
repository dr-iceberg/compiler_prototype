#![allow(dead_code, unused_variables)]

use compiler_prototype::grammar::*;
use compiler_prototype::tokenize::Lexer;
use std::error::Error;
use std::io::BufReader;
use std::{fs::File, io::Read};

struct SemanticAnalyzer {
    stmts: Vec<Stmt>,
    id_table: Vec<String>,
}

impl SemanticAnalyzer {
    fn check_program(&self) {
        for stmt in self.stmts.iter() {
            SemanticAnalyzer::check_stmt(stmt);
        }
    }

    fn generate_id_table(&self) {}

    fn check_stmt(stmt: &Stmt) {
        #[allow(clippy::single_match)]
        match stmt {
            Stmt::Assign { id, expr } => Self::check_expr(expr),
            _ => {}
        }
    }

    fn check_expr(expr: &Expr) {}
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("src.txt")?;
    let mut buffer = String::new();
    {
        let mut reader = BufReader::new(file);
        reader.read_to_string(&mut buffer)?;
    }

    let mut lexer = Lexer::new(buffer);

    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(errors) => {
            for err in errors {
                eprintln!("Lexer error: {err}");
            }
            return Ok(());
        }
    };

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
