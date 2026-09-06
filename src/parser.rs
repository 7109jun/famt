use crate::lexer::{Lexer, Token, TokenType};
use crate::ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        Parser {
            tokens,
            position: 0,
        }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&self.tokens[self.tokens.len() - 1])
    }

    fn peek(&self, offset: usize) -> &Token {
        self.tokens.get(self.position + offset).unwrap_or(&self.tokens[self.tokens.len() - 1])
    }

    fn advance(&mut self) -> Token {
        let token = self.current().clone();
        if self.position < self.tokens.len() {
            self.position += 1;
        }
        token
    }

    fn expect(&mut self, expected: TokenType) -> Result<Token, String> {
        if std::mem::discriminant(&self.current().token_type) == std::mem::discriminant(&expected) {
            Ok(self.advance())
        } else {
            Err(format!(
                "Expected {:?}, got {:?} at line {}",
                expected, self.current().token_type, self.current().line
            ))
        }
    }

    fn skip_newlines(&mut self) {
        while matches!(self.current().token_type, TokenType::Newline) {
            self.advance();
        }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut functions = Vec::new();
        let mut statements = Vec::new();

        self.skip_newlines();

        while !matches!(self.current().token_type, TokenType::Eof) {
            if matches!(self.current().token_type, TokenType::Func) {
                functions.push(self.parse_function()?);
            } else if matches!(self.current().token_type, TokenType::Newline) {
                self.advance();
            } else {
                statements.push(self.parse_statement()?);
            }
            self.skip_newlines();
        }

        Ok(Program { functions, statements })
    }

    fn parse_function(&mut self) -> Result<FunctionDecl, String> {
        let line = self.current().line;
        self.expect(TokenType::Func)?;

        let name = match &self.current().token_type {
            TokenType::Identifier(n) => n.clone(),
            _ => return Err("Expected function name".to_string()),
        };
        self.advance();

        self.expect(TokenType::LeftParen)?;

        let mut params = Vec::new();
        if !matches!(self.current().token_type, TokenType::RightParen) {
            loop {
                let param_type = self.parse_type()?;
                let param_name = match &self.current().token_type {
                    TokenType::Identifier(n) => n.clone(),
                    _ => return Err("Expected parameter name".to_string()),
                };
                self.advance();
                params.push((param_name, param_type));

                if !matches!(self.current().token_type, TokenType::Comma) {
                    break;
                }
                self.advance();
            }
        }

        self.expect(TokenType::RightParen)?;

        let return_type = if matches!(self.current().token_type, TokenType::Indent | TokenType::Newline) {
            Type::Void
        } else {
            self.parse_type()?
        };

        self.skip_newlines();
        self.expect(TokenType::Indent)?;

        let mut body = Vec::new();
        while !matches!(self.current().token_type, TokenType::Dedent | TokenType::Eof) {
            if matches!(self.current().token_type, TokenType::Newline) {
                self.advance();
            } else {
                body.push(self.parse_statement()?);
            }
        }

        self.expect(TokenType::Dedent)?;

        Ok(FunctionDecl {
            name,
            params,
            return_type,
            body,
            line,
        })
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        match &self.current().token_type {
            TokenType::Int => {
                self.advance();
                Ok(Type::Int)
            }
            TokenType::Float => {
                self.advance();
                Ok(Type::Float)
            }
            TokenType::String => {
                self.advance();
                Ok(Type::String)
            }
            TokenType::Bool => {
                self.advance();
                Ok(Type::Bool)
            }
            TokenType::Void => {
                self.advance();
                Ok(Type::Void)
            }
            TokenType::Array => {
                self.advance();
                self.expect(TokenType::Less)?;
                let inner = self.parse_type()?;
                self.expect(TokenType::Greater)?;
                Ok(Type::Array(Box::new(inner)))
            }
            _ => Err(format!("Expected type, got {:?}", self.current().token_type)),
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        self.skip_newlines();

        let line = self.current().line;

        match &self.current().token_type {
            TokenType::If => self.parse_if(),
            TokenType::For => self.parse_for(),
            TokenType::While => self.parse_while(),
            TokenType::Do => self.parse_do_while(),
            TokenType::Break => {
                self.advance();
                Ok(Statement::Break { line })
            }
            TokenType::Continue => {
                self.advance();
                Ok(Statement::Continue { line })
            }
            TokenType::Return => self.parse_return(),
            TokenType::Int | TokenType::Float | TokenType::String | TokenType::Bool | TokenType::Array => {
                self.parse_var_decl()
            }
            TokenType::Identifier(_) => {
                let name = match &self.current().token_type {
                    TokenType::Identifier(n) => n.clone(),
                    _ => return Err("Expected identifier".to_string()),
                };
                self.advance();

                if matches!(self.current().token_type, TokenType::Equal) {
                    self.advance();
                    let value = self.parse_expression()?;
                    Ok(Statement::Assignment {
                        target: name,
                        value,
                        line,
                    })
                } else if matches!(self.current().token_type, TokenType::LeftBracket) {
                    self.advance();
                    let index = Box::new(self.parse_expression()?);
                    self.expect(TokenType::RightBracket)?;
                    self.expect(TokenType::Equal)?;
                    let value = self.parse_expression()?;
                    Ok(Statement::IndexAssignment {
                        target: name,
                        index,
                        value,
                        line,
                    })
                } else {
                    Err(format!("Unexpected token after identifier: {:?}", self.current().token_type))
                }
            }
            _ => Err(format!("Unexpected statement start: {:?}", self.current().token_type)),
        }
    }

    fn parse_var_decl(&mut self) -> Result<Statement, String> {
        let line = self.current().line;
        let var_type = self.parse_type()?;

        let name = match &self.current().token_type {
            TokenType::Identifier(n) => n.clone(),
            _ => return Err("Expected variable name".to_string()),
        };
        self.advance();

        self.expect(TokenType::Equal)?;
        let value = self.parse_expression()?;

        Ok(Statement::VarDecl {
            name,
            var_type: Some(var_type),
            value,
            line,
        })
    }

    fn parse_if(&mut self) -> Result<Statement, String> {
        let line = self.current().line;
        self.expect(TokenType::If)?;

        let condition = self.parse_expression()?;
        self.skip_newlines();
        self.expect(TokenType::Indent)?;

        let mut then_body = Vec::new();
        while !matches!(self.current().token_type, TokenType::Dedent | TokenType::Eof) {
            if matches!(self.current().token_type, TokenType::Newline) {
                self.advance();
            } else {
                then_body.push(self.parse_statement()?);
            }
        }
        self.expect(TokenType::Dedent)?;

        let else_body = if matches!(self.current().token_type, TokenType::Else) {
            self.advance();
            self.skip_newlines();
            self.expect(TokenType::Indent)?;

            let mut body = Vec::new();
            while !matches!(self.current().token_type, TokenType::Dedent | TokenType::Eof) {
                if matches!(self.current().token_type, TokenType::Newline) {
                    self.advance();
                } else {
                    body.push(self.parse_statement()?);
                }
            }
            self.expect(TokenType::Dedent)?;
            Some(body)
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then_body,
            else_body,
            line,
        })
    }

    fn parse_for(&mut self) -> Result<Statement, String> {
        let line = self.current().line;
        self.expect(TokenType::For)?;

        let init = if matches!(self.current().token_type, TokenType::Semicolon) {
            None
        } else {
            Some(Box::new(self.parse_statement()?))
        };

        self.expect(TokenType::Semicolon)?;
        let condition = self.parse_expression()?;
        self.expect(TokenType::Semicolon)?;

        let update = if matches!(self.current().token_type, TokenType::Indent) {
            None
        } else {
            Some(Box::new(self.parse_expression()?))
        };

        self.skip_newlines();
        self.expect(TokenType::Indent)?;

        let mut body = Vec::new();
        while !matches!(self.current().token_type, TokenType::Dedent | TokenType::Eof) {
            if matches!(self.current().token_type, TokenType::Newline) {
                self.advance();
            } else {
                body.push(self.parse_statement()?);
            }
        }
        self.expect(TokenType::Dedent)?;

        Ok(Statement::For {
            init,
            condition,
            update,
            body,
            line,
        })
    }

    fn parse_while(&mut self) -> Result<Statement, String> {
        let line = self.current().line;
        self.expect(TokenType::While)?;

        let condition = self.parse_expression()?;
        self.skip_newlines();
        self.expect(TokenType::Indent)?;

        let mut body = Vec::new();
        while !matches!(self.current().token_type, TokenType::Dedent | TokenType::Eof) {
            if matches!(self.current().token_type, TokenType::Newline) {
                self.advance();
            } else {
                body.push(self.parse_statement()?);
            }
        }
        self.expect(TokenType::Dedent)?;

        Ok(Statement::While { condition, body, line })
    }

    fn parse_do_while(&mut self) -> Result<Statement, String> {
        let line = self.current().line;
        self.expect(TokenType::Do)?;
        self.skip_newlines();
        self.expect(TokenType::Indent)?;

        let mut body = Vec::new();
        while !matches!(self.current().token_type, TokenType::Dedent | TokenType::Eof) {
            if matches!(self.current().token_type, TokenType::Newline) {
                self.advance();
            } else {
                body.push(self.parse_statement()?);
            }
        }
        self.expect(TokenType::Dedent)?;

        self.expect(TokenType::While)?;
        let condition = self.parse_expression()?;

        Ok(Statement::DoWhile { body, condition, line })
    }

    fn parse_return(&mut self) -> Result<Statement, String> {
        let line = self.current().line;
        self.expect(TokenType::Return)?;

        let value = if matches!(self.current().token_type, TokenType::Newline | TokenType::Eof | TokenType::Dedent) {
            None
        } else {
            Some(self.parse_expression()?)
        };

        Ok(Statement::Return { value, line })
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_and()?;

        while matches!(self.current().token_type, TokenType::Or) {
            self.advance();
            let right = self.parse_and()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op: BinOp::Or,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_bitwise_or()?;

        while matches!(self.current().token_type, TokenType::And) {
            self.advance();
            let right = self.parse_bitwise_or()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op: BinOp::And,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_bitwise_or(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_bitwise_xor()?;

        while matches!(self.current().token_type, TokenType::Pipe) {
            self.advance();
            let right = self.parse_bitwise_xor()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op: BinOp::BitOr,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_bitwise_xor(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_bitwise_and()?;

        while matches!(self.current().token_type, TokenType::Caret) {
            self.advance();
            let right = self.parse_bitwise_and()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op: BinOp::BitXor,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_bitwise_and(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_equality()?;

        while matches!(self.current().token_type, TokenType::Ampersand) {
            self.advance();
            let right = self.parse_equality()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op: BinOp::BitAnd,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_relational()?;

        loop {
            let op = match &self.current().token_type {
                TokenType::EqualEqual => BinOp::Eq,
                TokenType::NotEqual => BinOp::Ne,
                _ => break,
            };
            self.advance();
            let right = self.parse_relational()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_relational(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_shift()?;

        loop {
            let op = match &self.current().token_type {
                TokenType::Less => BinOp::Lt,
                TokenType::Greater => BinOp::Gt,
                TokenType::LessEqual => BinOp::Le,
                TokenType::GreaterEqual => BinOp::Ge,
                _ => break,
            };
            self.advance();
            let right = self.parse_shift()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_shift(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_additive()?;

        loop {
            let op = match &self.current().token_type {
                TokenType::LeftShift => BinOp::LeftShift,
                TokenType::RightShift => BinOp::RightShift,
                _ => break,
            };
            self.advance();
            let right = self.parse_additive()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_additive(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_multiplicative()?;

        loop {
            let op = match &self.current().token_type {
                TokenType::Plus => BinOp::Add,
                TokenType::Minus => BinOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplicative()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_multiplicative(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_unary()?;

        loop {
            let op = match &self.current().token_type {
                TokenType::Star => BinOp::Mul,
                TokenType::Slash => BinOp::Div,
                TokenType::Percent => BinOp::Mod,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expression, String> {
        match &self.current().token_type {
            TokenType::Not => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expression::Unary {
                    op: UnOp::Not,
                    operand: Box::new(operand),
                })
            }
            TokenType::Minus => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expression::Unary {
                    op: UnOp::Neg,
                    operand: Box::new(operand),
                })
            }
            TokenType::Tilde => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expression::Unary {
                    op: UnOp::BitNot,
                    operand: Box::new(operand),
                })
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_primary()?;

        loop {
            match &self.current().token_type {
                TokenType::LeftBracket => {
                    self.advance();
                    let index = self.parse_expression()?;
                    self.expect(TokenType::RightBracket)?;
                    expr = Expression::Index {
                        target: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                TokenType::LeftParen => {
                    if let Expression::Identifier(func_name) = expr {
                        self.advance();
                        let mut args = Vec::new();

                        if !matches!(self.current().token_type, TokenType::RightParen) {
                            loop {
                                args.push(self.parse_expression()?);
                                if !matches!(self.current().token_type, TokenType::Comma) {
                                    break;
                                }
                                self.advance();
                            }
                        }

                        self.expect(TokenType::RightParen)?;
                        expr = Expression::Call {
                            func: func_name,
                            args,
                        };
                    } else {
                        return Err("Cannot call non-function".to_string());
                    }
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        match &self.current().token_type.clone() {
            TokenType::IntLiteral(i) => {
                let value = *i;
                self.advance();
                Ok(Expression::IntLiteral(value))
            }
            TokenType::FloatLiteral(f) => {
                let value = *f;
                self.advance();
                Ok(Expression::FloatLiteral(value))
            }
            TokenType::StringLiteral(s) => {
                let value = s.clone();
                self.advance();
                Ok(Expression::StringLiteral(value))
            }
            TokenType::True => {
                self.advance();
                Ok(Expression::BoolLiteral(true))
            }
            TokenType::False => {
                self.advance();
                Ok(Expression::BoolLiteral(false))
            }
            TokenType::Null => {
                self.advance();
                Ok(Expression::NullLiteral)
            }
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(Expression::Identifier(name))
            }
            TokenType::LeftBracket => {
                self.advance();
                let mut elements = Vec::new();

                if !matches!(self.current().token_type, TokenType::RightBracket) {
                    loop {
                        elements.push(self.parse_expression()?);
                        if !matches!(self.current().token_type, TokenType::Comma) {
                            break;
                        }
                        self.advance();
                    }
                }

                self.expect(TokenType::RightBracket)?;
                Ok(Expression::ArrayLiteral(elements))
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(TokenType::RightParen)?;
                Ok(expr)
            }
            _ => Err(format!("Unexpected token: {:?}", self.current().token_type)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_function() {
        let mut parser = Parser::new("func add(int a, int b) int\n    return a + b\n");
        let program = parser.parse();
        assert!(program.is_ok());
    }
}
