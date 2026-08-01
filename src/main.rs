#![allow(dead_code, unused_variables)]

use compiler_prototype::analyzer::SemanticAnalyzer;
use compiler_prototype::codegen::CodeGen;
use compiler_prototype::parser;
use compiler_prototype::tokenize::Lexer;
use inkwell::context::Context;
use std::error::Error;
use std::io::BufReader;
use std::{fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("src.txt")?;
    let mut buffer = String::new();
    {
        let mut reader = BufReader::new(file);
        reader.read_to_string(&mut buffer)?;
    }

    let mut lexer = Lexer::new(buffer);

    match lexer.tokenize() {
        Ok(tokens) => {
            let mut parser = parser::Parser::new(tokens);
            match parser.parse_program() {
                Ok(program) => {
                    println!("{:?}", program);
                    let analyzer = SemanticAnalyzer::default();
                    match analyzer.check_program(&program) {
                        Ok(_) => {
                            let ctx = Context::create();
                            let mut code_generator = CodeGen::new(&ctx);
                            code_generator.compile_program(&program)?;
                        }
                        Err(err) => {
                            eprintln!("Semantic error: {err}");
                        }
                    }
                }
                Err(errors) => {
                    for err in errors {
                        eprintln!("Lexer error: {err}");
                    }
                    return Ok(());
                }
            }
        }
        Err(errors) => {
            for err in errors {
                eprintln!("Lexer error: {err}");
            }
            return Ok(());
        }
    };

    Ok(())
}
