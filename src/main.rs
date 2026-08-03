use compiler_prototype::analyzer::SemanticAnalyzer;
use compiler_prototype::codegen::CodeGen;
use compiler_prototype::parser;
use compiler_prototype::tokenize::{Lexer, Token};
use inkwell::context::Context;
use std::error::Error;
use std::io::BufReader;
use std::{fs::File, io::Read};

#[allow(unused)]
fn print_tokens(tokens: &Vec<Token>, source: &str) {
    for token in tokens {
        println!("token '{}' at {}", token.lexeme(source), token.span.start())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("src.txt")?;
    let mut source = String::new();
    {
        let mut reader = BufReader::new(file);
        reader.read_to_string(&mut source)?;
    }

    let mut lexer = Lexer::new(&source);

    match lexer.tokenize() {
        Ok(tokens) => {
            //print_tokens(&tokens, &source);

            let mut parser = parser::Parser::new(tokens, &source);
            match parser.parse_program() {
                Ok(program) => {
                    //println!("{:?}", program);
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
                        eprintln!("Parser error: {err}");
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
