pub mod lexer;
pub mod parser;
pub mod ast;
pub mod vm;
pub mod jit;
pub mod executor;

pub use lexer::Lexer;
pub use parser::Parser;
pub use ast::*;
pub use vm::VM;
pub use executor::Executor;
pub use jit::{JIT, HotspotDetector};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let source = "let x = 42;";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        assert!(!tokens.is_empty());
    }

    #[test]
    fn test_parser() {
        let source = "int x = 10";
        let mut parser = Parser::new(source);
        let program = parser.parse();
        assert!(program.is_ok());
    }

    #[test]
    fn test_executor() {
        let source = "let x = 5 + 3;";
        let mut parser = Parser::new(source);
        if let Ok(program) = parser.parse() {
            let mut executor = Executor::new();
            let result = executor.execute(program);
            assert!(result.is_ok());
        }
    }
}
