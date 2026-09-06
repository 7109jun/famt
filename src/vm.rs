use crate::ast::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Instruction {
    // Constants
    LoadInt(i64),
    LoadFloat(f64),
    LoadString(String),
    LoadBool(bool),
    LoadNull,
    LoadArray(usize), // Array size

    // Variables
    LoadVar(String),
    StoreVar(String),
    LoadGlobal(String),
    StoreGlobal(String),

    // Operators
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,
    Not,

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
    BitNot,
    LeftShift,
    RightShift,

    // Control Flow
    JumpIfFalse(usize),
    Jump(usize),

    // Array
    Index,
    IndexStore,
    ArrayLen,

    // Functions
    Call(String, usize), // function name, arg count
    Return,

    // I/O
    Print,
    PrintLn,

    // Misc
    Pop,
    Nop,
}

pub struct VM {
    stack: Vec<Value>,
    variables: HashMap<String, Value>,
    globals: HashMap<String, Value>,
    instructions: Vec<Instruction>,
    pc: usize, // Program counter
    call_stack: Vec<usize>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: Vec::new(),
            variables: HashMap::new(),
            globals: HashMap::new(),
            instructions: Vec::new(),
            pc: 0,
            call_stack: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, instr: Instruction) {
        self.instructions.push(instr);
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Value {
        self.stack.pop().unwrap_or(Value::Null)
    }

    pub fn peek(&self) -> Value {
        self.stack.last().cloned().unwrap_or(Value::Null)
    }

    pub fn execute(&mut self) -> Result<Value, String> {
        while self.pc < self.instructions.len() {
            let instr = self.instructions[self.pc].clone();
            self.pc += 1;

            match instr {
                Instruction::LoadInt(i) => self.push(Value::Int(i)),
                Instruction::LoadFloat(f) => self.push(Value::Float(f)),
                Instruction::LoadString(s) => self.push(Value::String(s)),
                Instruction::LoadBool(b) => self.push(Value::Bool(b)),
                Instruction::LoadNull => self.push(Value::Null),

                Instruction::LoadArray(size) => {
                    let mut arr = Vec::new();
                    for _ in 0..size {
                        arr.push(self.pop());
                    }
                    arr.reverse();
                    self.push(Value::Array(arr));
                }

                Instruction::LoadVar(name) => {
                    let value = self.variables.get(&name).cloned().unwrap_or(Value::Null);
                    self.push(value);
                }

                Instruction::StoreVar(name) => {
                    let value = self.pop();
                    self.variables.insert(name, value);
                }

                Instruction::LoadGlobal(name) => {
                    let value = self.globals.get(&name).cloned().unwrap_or(Value::Null);
                    self.push(value);
                }

                Instruction::StoreGlobal(name) => {
                    let value = self.pop();
                    self.globals.insert(name, value);
                }

                Instruction::Add => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = match (a, b) {
                        (Value::Int(x), Value::Int(y)) => Value::Int(x + y),
                        (Value::Float(x), Value::Float(y)) => Value::Float(x + y),
                        (Value::Int(x), Value::Float(y)) => Value::Float(x as f64 + y),
                        (Value::Float(x), Value::Int(y)) => Value::Float(x + y as f64),
                        (Value::String(x), Value::String(y)) => Value::String(format!("{}{}", x, y)),
                        _ => Value::Null,
                    };
                    self.push(result);
                }

                Instruction::Sub => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = match (a, b) {
                        (Value::Int(x), Value::Int(y)) => Value::Int(x - y),
                        (Value::Float(x), Value::Float(y)) => Value::Float(x - y),
                        (Value::Int(x), Value::Float(y)) => Value::Float(x as f64 - y),
                        (Value::Float(x), Value::Int(y)) => Value::Float(x - y as f64),
                        _ => Value::Null,
                    };
                    self.push(result);
                }

                Instruction::Mul => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = match (a, b) {
                        (Value::Int(x), Value::Int(y)) => Value::Int(x * y),
                        (Value::Float(x), Value::Float(y)) => Value::Float(x * y),
                        (Value::Int(x), Value::Float(y)) => Value::Float(x as f64 * y),
                        (Value::Float(x), Value::Int(y)) => Value::Float(x * y as f64),
                        _ => Value::Null,
                    };
                    self.push(result);
                }

                Instruction::Div => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = match (a, b) {
                        (Value::Int(x), Value::Int(y)) if y != 0 => Value::Int(x / y),
                        (Value::Float(x), Value::Float(y)) if y != 0.0 => Value::Float(x / y),
                        (Value::Int(x), Value::Float(y)) if y != 0.0 => Value::Float(x as f64 / y),
                        (Value::Float(x), Value::Int(y)) if y != 0 => Value::Float(x / y as f64),
                        _ => Value::Null,
                    };
                    self.push(result);
                }

                Instruction::Mod => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = match (a, b) {
                        (Value::Int(x), Value::Int(y)) if y != 0 => Value::Int(x % y),
                        _ => Value::Null,
                    };
                    self.push(result);
                }

                Instruction::Neg => {
                    let a = self.pop();
                    let result = match a {
                        Value::Int(x) => Value::Int(-x),
                        Value::Float(x) => Value::Float(-x),
                        _ => Value::Null,
                    };
                    self.push(result);
                }

                Instruction::Not => {
                    let a = self.pop();
                    self.push(Value::Bool(!a.to_bool()));
                }

                Instruction::Eq => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Bool(a == b));
                }

                Instruction::Ne => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Bool(a != b));
                }

                Instruction::Lt => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = match (a, b) {
                        (Value::Int(x), Value::Int(y)) => x < y,
                        (Value::Float(x), Value::Float(y)) => x < y,
                        (Value::Int(x), Value::Float(y)) => (x as f64) < y,
                        (Value::Float(x), Value::Int(y)) => x < (y as f64),
                        _ => false,
                    };
                    self.push(Value::Bool(result));
                }

                Instruction::Gt => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = match (a, b) {
                        (Value::Int(x), Value::Int(y)) => x > y,
                        (Value::Float(x), Value::Float(y)) => x > y,
                        (Value::Int(x), Value::Float(y)) => (x as f64) > y,
                        (Value::Float(x), Value::Int(y)) => x > (y as f64),
                        _ => false,
                    };
                    self.push(Value::Bool(result));
                }

                Instruction::Le => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = match (a, b) {
                        (Value::Int(x), Value::Int(y)) => x <= y,
                        (Value::Float(x), Value::Float(y)) => x <= y,
                        (Value::Int(x), Value::Float(y)) => (x as f64) <= y,
                        (Value::Float(x), Value::Int(y)) => x <= (y as f64),
                        _ => false,
                    };
                    self.push(Value::Bool(result));
                }

                Instruction::Ge => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = match (a, b) {
                        (Value::Int(x), Value::Int(y)) => x >= y,
                        (Value::Float(x), Value::Float(y)) => x >= y,
                        (Value::Int(x), Value::Float(y)) => (x as f64) >= y,
                        (Value::Float(x), Value::Int(y)) => x >= (y as f64),
                        _ => false,
                    };
                    self.push(Value::Bool(result));
                }

                Instruction::And => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Bool(a.to_bool() && b.to_bool()));
                }

                Instruction::Or => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Bool(a.to_bool() || b.to_bool()));
                }

                Instruction::BitAnd => {
                    let b = self.pop().to_int();
                    let a = self.pop().to_int();
                    self.push(Value::Int(a & b));
                }

                Instruction::BitOr => {
                    let b = self.pop().to_int();
                    let a = self.pop().to_int();
                    self.push(Value::Int(a | b));
                }

                Instruction::BitXor => {
                    let b = self.pop().to_int();
                    let a = self.pop().to_int();
                    self.push(Value::Int(a ^ b));
                }

                Instruction::BitNot => {
                    let a = self.pop().to_int();
                    self.push(Value::Int(!a));
                }

                Instruction::LeftShift => {
                    let b = self.pop().to_int();
                    let a = self.pop().to_int();
                    self.push(Value::Int(a << b));
                }

                Instruction::RightShift => {
                    let b = self.pop().to_int();
                    let a = self.pop().to_int();
                    self.push(Value::Int(a >> b));
                }

                Instruction::JumpIfFalse(addr) => {
                    let cond = self.pop();
                    if !cond.to_bool() {
                        self.pc = addr;
                    }
                }

                Instruction::Jump(addr) => {
                    self.pc = addr;
                }

                Instruction::Index => {
                    let idx = self.pop().to_int() as usize;
                    let arr = self.pop();
                    if let Value::Array(a) = arr {
                        let result = a.get(idx).cloned().unwrap_or(Value::Null);
                        self.push(result);
                    } else {
                        self.push(Value::Null);
                    }
                }

                Instruction::IndexStore => {
                    let value = self.pop();
                    let idx = self.pop().to_int() as usize;
                    let arr = self.pop();
                    if let Value::Array(mut a) = arr {
                        if idx < a.len() {
                            a[idx] = value;
                        }
                        self.push(Value::Array(a));
                    }
                }

                Instruction::ArrayLen => {
                    let arr = self.pop();
                    if let Value::Array(a) = arr {
                        self.push(Value::Int(a.len() as i64));
                    } else {
                        self.push(Value::Int(0));
                    }
                }

                Instruction::Print => {
                    let val = self.pop();
                    print!("{}", val.to_string());
                }

                Instruction::PrintLn => {
                    let val = self.pop();
                    println!("{}", val.to_string());
                }

                Instruction::Pop => {
                    self.pop();
                }

                Instruction::Nop => {}

                _ => return Err("Unimplemented instruction".to_string()),
            }
        }

        Ok(self.peek())
    }
}
