use famt::{Lexer, Parser, Executor};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: famt <script.famt>");
        std::process::exit(1);
    }

    let filename = &args[1];

    // Read the file
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file {}: {}", filename, err);
            std::process::exit(1);
        }
    };

    // Lexing
    println!("=== Lexing ===");
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();
    println!("Tokens: {:?}", tokens.len());

    // Parsing
    println!("\n=== Parsing ===");
    let mut parser = Parser::new(&source);
    let program = match parser.parse() {
        Ok(prog) => {
            println!("Functions: {}", prog.functions.len());
            println!("Statements: {}", prog.statements.len());
            prog
        }
        Err(err) => {
            eprintln!("Parse error: {}", err);
            std::process::exit(1);
        }
    };

    // Execution
    println!("\n=== Execution ===");
    let mut executor = Executor::new();
    match executor.execute(program) {
        Ok(result) => {
            println!("Program completed successfully");
            println!("Result: {:?}", result);
        }
        Err(err) => {
            eprintln!("Execution error: {}", err);
            std::process::exit(1);
        }
    }
}
