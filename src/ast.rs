#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Array(Vec<Value>),
    Null,
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::String(_) => "string",
            Value::Bool(_) => "bool",
            Value::Array(_) => "array",
            Value::Null => "null",
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Int(i) => i.to_string(),
            Value::Float(f) => {
                if f.fract() == 0.0 {
                    format!("{:.1}", f)
                } else {
                    f.to_string()
                }
            }
            Value::String(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                format!("[{}]", elements.join(", "))
            }
            Value::Null => "null".to_string(),
        }
    }

    pub fn to_int(&self) -> i64 {
        match self {
            Value::Int(i) => *i,
            Value::Float(f) => *f as i64,
            Value::Bool(b) => if *b { 1 } else { 0 },
            Value::String(s) => s.parse::<i64>().unwrap_or(0),
            Value::Null => 0,
            _ => 0,
        }
    }

    pub fn to_float(&self) -> f64 {
        match self {
            Value::Int(i) => *i as f64,
            Value::Float(f) => *f,
            Value::Bool(b) => if *b { 1.0 } else { 0.0 },
            Value::String(s) => s.parse::<f64>().unwrap_or(0.0),
            Value::Null => 0.0,
            _ => 0.0,
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::Bool(b) => *b,
            Value::String(s) => !s.is_empty(),
            Value::Null => false,
            Value::Array(arr) => !arr.is_empty(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    Void,
    Array(Box<Type>),
}

impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::Int => "int".to_string(),
            Type::Float => "float".to_string(),
            Type::String => "string".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Void => "void".to_string(),
            Type::Array(t) => format!("array<{}>", t.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<FunctionDecl>,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub return_type: Type,
    pub body: Vec<Statement>,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub enum Statement {
    VarDecl {
        name: String,
        var_type: Option<Type>,
        value: Expression,
        line: usize,
    },
    Assignment {
        target: String,
        value: Expression,
        line: usize,
    },
    IndexAssignment {
        target: String,
        index: Box<Expression>,
        value: Expression,
        line: usize,
    },
    If {
        condition: Expression,
        then_body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
        line: usize,
    },
    For {
        init: Option<Box<Statement>>,
        condition: Expression,
        update: Option<Box<Expression>>,
        body: Vec<Statement>,
        line: usize,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
        line: usize,
    },
    DoWhile {
        body: Vec<Statement>,
        condition: Expression,
        line: usize,
    },
    Break {
        line: usize,
    },
    Continue {
        line: usize,
    },
    Return {
        value: Option<Expression>,
        line: usize,
    },
    Expression {
        expr: Expression,
        line: usize,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    NullLiteral,
    ArrayLiteral(Vec<Expression>),
    Identifier(String),
    Binary {
        left: Box<Expression>,
        op: BinOp,
        right: Box<Expression>,
    },
    Unary {
        op: UnOp,
        operand: Box<Expression>,
    },
    Call {
        func: String,
        args: Vec<Expression>,
    },
    Index {
        target: Box<Expression>,
        index: Box<Expression>,
    },
    Member {
        target: Box<Expression>,
        field: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    // Comparison
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    // Logical
    And,
    Or,
    // Bitwise
    BitAnd,
    BitOr,
    BitXor,
    LeftShift,
    RightShift,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Not,
    Neg,
    BitNot,
    Pos,
}
