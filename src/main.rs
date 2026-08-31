use clap::Parser;
#[allow(unused)]
use compiler_prototype::analyzer::SemanticAnalyzer;
use compiler_prototype::cli::CompilerArgs;
#[allow(unused)]
use compiler_prototype::codegen::CodeGen;
use compiler_prototype::file::SourceFile;
use compiler_prototype::parser;
use compiler_prototype::tokenize::Lexer;
#[allow(unused)]
use inkwell::context::Context;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // let args = CompilerArgs::parse();
    // for now only process one file
    // let file = SourceFile::new(args.inputs.first().unwrap())?;
    let file = SourceFile::new("src.txt")?;
    let mut lexer = Lexer::new(&file);

    match lexer.tokenize() {
        Ok(tokens) => {
            //print_tokens(&tokens, &source);

            let mut parser = parser::Parser::new(tokens, &file);
            match parser.parse_program() {
                Ok(program) => {
                    // println!("{:#?}", program);
                    // /*
                    let mut analyzer = SemanticAnalyzer::default();
                    match analyzer.check_program(&program) {
                        Ok(semantic_info) => {
                            // /*
                            let ctx = Context::create();
                            let mut code_generator = CodeGen::new(&ctx, &semantic_info);
                            code_generator.compile_program(&program)?;
                            // */
                        }
                        Err(errors) => {
                            for err in errors {
                                eprintln!("Semantic error: {err}");
                            }
                            return Ok(());
                        }
                    }
                    // */
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
