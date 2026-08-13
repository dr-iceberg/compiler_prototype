use crate::tokenize::Token;

#[allow(unused)]
fn print_tokens(tokens: &Vec<Token>, source: &str) {
    for token in tokens {
        println!("token '{}' at {}", token.lexeme(source), token.span.start())
    }
}
