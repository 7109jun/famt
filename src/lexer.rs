#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords
    Func,
    If,
    Else,
    For,
    While,
    Do,
    Break,
    Continue,
    Return,
    Int,
    Float,
    String,
    Bool,
    Array,
    Void,
    True,
    False,
    Null,

    // Identifiers and Literals
    Identifier(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    PlusPlus,
    MinusMinus,

    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,

    EqualEqual,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    And,
    Or,
    Not,
    Ampersand,
    Pipe,
    Caret,
    Tilde,
    LeftShift,
    RightShift,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Colon,
    Semicolon,

    // Special
    Newline,
    Indent,
    Dedent,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize, column: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
            column,
        }
    }
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
    indent_stack: Vec<usize>,
    pending_tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
            indent_stack: vec![0],
            pending_tokens: Vec::new(),
        }
    }

    fn current_char(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    fn peek_char(&self, offset: usize) -> Option<char> {
        let pos = self.position + offset;
        if pos < self.input.len() {
            Some(self.input[pos])
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.current_char() {
            self.position += 1;
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            Some(ch)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_single_line_comment(&mut self) {
        // Skip //
        self.advance();
        self.advance();
        while let Some(ch) = self.current_char() {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn skip_multi_line_comment(&mut self) {
        // Skip /*
        self.advance();
        self.advance();
        while let Some(ch) = self.current_char() {
            if ch == '*' && self.peek_char(1) == Some('/') {
                self.advance();
                self.advance();
                break;
            }
            self.advance();
        }
    }

    fn read_string(&mut self, quote: char) -> String {
        let mut result = String::new();
        self.advance(); // skip opening quote

        while let Some(ch) = self.current_char() {
            if ch == quote {
                self.advance();
                break;
            }
            if ch == '\\' {
                self.advance();
                if let Some(escaped) = self.current_char() {
                    match escaped {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        'r' => result.push('\r'),
                        '\\' => result.push('\\'),
                        '"' => result.push('"'),
                        '\'' => result.push('\''),
                        _ => {
                            result.push('\\');
                            result.push(escaped);
                        }
                    }
                    self.advance();
                }
            } else {
                result.push(ch);
                self.advance();
            }
        }
        result
    }

    fn read_number(&mut self) -> Token {
        let start_line = self.line;
        let start_column = self.column;
        let mut number_str = String::new();

        // Handle hex, binary, octal
        if self.current_char() == Some('0') {
            if self.peek_char(1) == Some('x') {
                number_str.push_str("0x");
                self.advance();
                self.advance();
                while let Some(ch) = self.current_char() {
                    if ch.is_ascii_hexdigit() {
                        number_str.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }
                let value = i64::from_str_radix(&number_str[2..], 16).unwrap_or(0);
                return Token::new(TokenType::IntLiteral(value), number_str, start_line, start_column);
            } else if self.peek_char(1) == Some('b') {
                number_str.push_str("0b");
                self.advance();
                self.advance();
                while let Some(ch) = self.current_char() {
                    if ch == '0' || ch == '1' {
                        number_str.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }
                let value = i64::from_str_radix(&number_str[2..], 2).unwrap_or(0);
                return Token::new(TokenType::IntLiteral(value), number_str, start_line, start_column);
            } else if self.peek_char(1) == Some('o') {
                number_str.push_str("0o");
                self.advance();
                self.advance();
                while let Some(ch) = self.current_char() {
                    if ch >= '0' && ch <= '7' {
                        number_str.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }
                let value = i64::from_str_radix(&number_str[2..], 8).unwrap_or(0);
                return Token::new(TokenType::IntLiteral(value), number_str, start_line, start_column);
            }
        }

        // Regular decimal number
        let mut is_float = false;

        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() {
                number_str.push(ch);
                self.advance();
            } else if ch == '.' && !is_float && self.peek_char(1).map_or(false, |c| c.is_ascii_digit()) {
                is_float = true;
                number_str.push(ch);
                self.advance();
            } else if (ch == 'e' || ch == 'E') && !number_str.is_empty() {
                number_str.push(ch);
                self.advance();
                if let Some(sign_ch) = self.current_char() {
                    if sign_ch == '+' || sign_ch == '-' {
                        number_str.push(sign_ch);
                        self.advance();
                    }
                }
                is_float = true;
            } else {
                break;
            }
        }

        if is_float {
            let value = number_str.parse::<f64>().unwrap_or(0.0);
            Token::new(TokenType::FloatLiteral(value), number_str, start_line, start_column)
        } else {
            let value = number_str.parse::<i64>().unwrap_or(0);
            Token::new(TokenType::IntLiteral(value), number_str, start_line, start_column)
        }
    }

    fn read_identifier(&mut self) -> Token {
        let start_line = self.line;
        let start_column = self.column;
        let mut ident = String::new();

        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        let token_type = match ident.as_str() {
            "func" => TokenType::Func,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "for" => TokenType::For,
            "while" => TokenType::While,
            "do" => TokenType::Do,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "return" => TokenType::Return,
            "int" => TokenType::Int,
            "float" => TokenType::Float,
            "string" => TokenType::String,
            "bool" => TokenType::Bool,
            "array" => TokenType::Array,
            "void" => TokenType::Void,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "null" => TokenType::Null,
            _ => TokenType::Identifier(ident.clone()),
        };

        Token::new(token_type, ident, start_line, start_column)
    }

    fn handle_indentation(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut indent_level = 0;

        while let Some(ch) = self.current_char() {
            match ch {
                ' ' => {
                    indent_level += 1;
                    self.advance();
                }
                '\t' => {
                    indent_level += 4;
                    self.advance();
                }
                _ => break,
            }
        }

        if self.current_char() == Some('\n') || self.current_char() == Some('\r') || self.current_char().is_none() {
            return tokens;
        }

        let current_indent = *self.indent_stack.last().unwrap_or(&0);

        if indent_level > current_indent {
            self.indent_stack.push(indent_level);
            tokens.push(Token::new(TokenType::Indent, "INDENT".to_string(), self.line, self.column));
        } else if indent_level < current_indent {
            while let Some(&last_indent) = self.indent_stack.last() {
                if last_indent <= indent_level {
                    break;
                }
                self.indent_stack.pop();
                tokens.push(Token::new(TokenType::Dedent, "DEDENT".to_string(), self.line, self.column));
            }
        }

        tokens
    }

    pub fn next_token(&mut self) -> Token {
        if !self.pending_tokens.is_empty() {
            return self.pending_tokens.remove(0);
        }

        self.skip_whitespace();

        if self.current_char() == Some('/') && self.peek_char(1) == Some('/') {
            self.skip_single_line_comment();
            return self.next_token();
        }

        if self.current_char() == Some('/') && self.peek_char(1) == Some('*') {
            self.skip_multi_line_comment();
            return self.next_token();
        }

        let line = self.line;
        let column = self.column;

        match self.current_char() {
            None => Token::new(TokenType::Eof, "EOF".to_string(), line, column),
            Some('\n') => {
                self.advance();
                let mut indent_tokens = self.handle_indentation();
                if !indent_tokens.is_empty() {
                    let first = indent_tokens.remove(0);
                    self.pending_tokens.extend(indent_tokens);
                    first
                } else {
                    self.next_token()
                }
            }
            Some(ch) if ch.is_ascii_digit() => self.read_number(),
            Some(ch) if ch.is_alphabetic() || ch == '_' => self.read_identifier(),
            Some('"') => {
                let string_val = self.read_string('"');
                Token::new(
    TokenType::StringLiteral(string_val.clone()),
    format!("\"{}\"", string_val),
    line,
   Token::new(
    TokenType::StringLiteral(string_val.clone()),
    format!("'{}'", string_val),
    line,
    column,
)
)
            }
            Some('\'') => {
                let string_val = self.read_string('\'');
                Token::new(TokenType::StringLiteral(string_val), format!("'{}'", string_val), line, column)
            }
            Some('+') => {
                self.advance();
                if self.current_char() == Some('+') {
                    self.advance();
                    Token::new(TokenType::PlusPlus, "++".to_string(), line, column)
                } else if self.current_char() == Some('=') {
                    self.advance();
                    Token::new(TokenType::PlusEqual, "+=".to_string(), line, column)
                } else {
                    Token::new(TokenType::Plus, "+".to_string(), line, column)
                }
            }
            Some('-') => {
                self.advance();
                if self.current_char() == Some('-') {
                    self.advance();
                    Token::new(TokenType::MinusMinus, "--".to_string(), line, column)
                } else if self.current_char() == Some('=') {
                    self.advance();
                    Token::new(TokenType::MinusEqual, "-=".to_string(), line, column)
                } else {
                    Token::new(TokenType::Minus, "-".to_string(), line, column)
                }
            }
            Some('*') => {
                self.advance();
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::new(TokenType::StarEqual, "*=".to_string(), line, column)
                } else {
                    Token::new(TokenType::Star, "*".to_string(), line, column)
                }
            }
            Some('/') => {
                self.advance();
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::new(TokenType::SlashEqual, "/=".to_string(), line, column)
                } else {
                    Token::new(TokenType::Slash, "/".to_string(), line, column)
                }
            }
            Some('%') => {
                self.advance();
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::new(TokenType::PercentEqual, "%=".to_string(), line, column)
                } else {
                    Token::new(TokenType::Percent, "%".to_string(), line, column)
                }
            }
            Some('=') => {
                self.advance();
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::new(TokenType::EqualEqual, "==".to_string(), line, column)
                } else {
                    Token::new(TokenType::Equal, "=".to_string(), line, column)
                }
            }
            Some('!') => {
                self.advance();
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::new(TokenType::NotEqual, "!=".to_string(), line, column)
                } else {
                    Token::new(TokenType::Not, "!".to_string(), line, column)
                }
            }
            Some('<') => {
                self.advance();
                if self.current_char() == Some('<') {
                    self.advance();
                    Token::new(TokenType::LeftShift, "<<".to_string(), line, column)
                } else if self.current_char() == Some('=') {
                    self.advance();
                    Token::new(TokenType::LessEqual, "<=".to_string(), line, column)
                } else {
                    Token::new(TokenType::Less, "<".to_string(), line, column)
                }
            }
            Some('>') => {
                self.advance();
                if self.current_char() == Some('>') {
                    self.advance();
                    Token::new(TokenType::RightShift, ">>".to_string(), line, column)
                } else if self.current_char() == Some('=') {
                    self.advance();
                    Token::new(TokenType::GreaterEqual, ">=".to_string(), line, column)
                } else {
                    Token::new(TokenType::Greater, ">".to_string(), line, column)
                }
            }
            Some('&') => {
                self.advance();
                if self.current_char() == Some('&') {
                    self.advance();
                    Token::new(TokenType::And, "&&".to_string(), line, column)
                } else {
                    Token::new(TokenType::Ampersand, "&".to_string(), line, column)
                }
            }
            Some('|') => {
                self.advance();
                if self.current_char() == Some('|') {
                    self.advance();
                    Token::new(TokenType::Or, "||".to_string(), line, column)
                } else {
                    Token::new(TokenType::Pipe, "|".to_string(), line, column)
                }
            }
            Some('^') => {
                self.advance();
                Token::new(TokenType::Caret, "^".to_string(), line, column)
            }
            Some('~') => {
                self.advance();
                Token::new(TokenType::Tilde, "~".to_string(), line, column)
            }
            Some('(') => {
                self.advance();
                Token::new(TokenType::LeftParen, "(".to_string(), line, column)
            }
            Some(')') => {
                self.advance();
                Token::new(TokenType::RightParen, ")".to_string(), line, column)
            }
            Some('[') => {
                self.advance();
                Token::new(TokenType::LeftBracket, "[".to_string(), line, column)
            }
            Some(']') => {
                self.advance();
                Token::new(TokenType::RightBracket, "]".to_string(), line, column)
            }
            Some('{') => {
                self.advance();
                Token::new(TokenType::LeftBrace, "{".to_string(), line, column)
            }
            Some('}') => {
                self.advance();
                Token::new(TokenType::RightBrace, "}".to_string(), line, column)
            }
            Some(',') => {
                self.advance();
                Token::new(TokenType::Comma, ",".to_string(), line, column)
            }
            Some('.') => {
                self.advance();
                Token::new(TokenType::Dot, ".".to_string(), line, column)
            }
            Some(':') => {
                self.advance();
                Token::new(TokenType::Colon, ":".to_string(), line, column)
            }
            Some(';') => {
                self.advance();
                Token::new(TokenType::Semicolon, ";".to_string(), line, column)
            }
            Some(ch) => {
                self.advance();
                Token::new(TokenType::Identifier(ch.to_string()), ch.to_string(), line, column)
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            let is_eof = matches!(token.token_type, TokenType::Eof);
            tokens.push(token);
            if is_eof {
                break;
            }
        }

        // Add final dedents
        while self.indent_stack.len() > 1 {
            self.indent_stack.pop();
            tokens.insert(tokens.len() - 1, Token::new(TokenType::Dedent, "DEDENT".to_string(), 0, 0));
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::new("func add(int a, int b) int");
        let tokens = lexer.tokenize();
        assert_eq!(tokens.len(), 11);
    }

    #[test]
    fn test_string_literals() {
        let mut lexer = Lexer::new("\"hello\" 'world'");
        let tokens = lexer.tokenize();
        assert!(matches!(tokens[0].token_type, TokenType::StringLiteral(_)));
    }

    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new("42 3.14 0xFF 0b1010");
        let tokens = lexer.tokenize();
        assert!(matches!(tokens[0].token_type, TokenType::IntLiteral(42)));
        assert!(matches!(tokens[1].token_type, TokenType::FloatLiteral(_)));
    }
}
