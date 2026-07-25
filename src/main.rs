#![allow(dead_code, unused_variables)]

use compiler_prototype::tokenize::Lexer;
use compiler_prototype::{grammar::*, parser::Parser};
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
        match stmt {
            Stmt::Assign { id, expr } => Self::check_expr(expr),
            _ => {}
        }
    }

    fn check_expr(expr: &Expr) {}
}

fn main() -> std::io::Result<()> {
    let file = File::open("src.txt")?; 
    let mut buffer = String::new();
    {
        let mut reader = BufReader::new(file);
        reader.read_to_string(&mut buffer)?;
    }
    

    let mut lexer = Lexer::new(buffer);

    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser {
        tokens: tokens.clone(),
        pos: 0,
    };
    let stmts = parser.parse_input();

    for token in tokens {
        println!("{:?}", token);
    }

    for stmt in stmts {
        println!("{:?}", stmt);
    }

    Ok(())
}
