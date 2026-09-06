use crate::ast::*;
use std::collections::HashMap;

pub struct Executor {
    variables: HashMap<String, Value>,
    functions: HashMap<String, FunctionDecl>,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn execute(&mut self, program: Program) -> Result<Value, String> {
        // Register all functions
        for func in program.functions {
            self.functions.insert(func.name.clone(), func);
        }

        // Execute main statements
        let mut result = Value::Null;
        for stmt in program.statements {
            result = self.execute_statement(&stmt)?;
        }

        // If there's a main function, execute it
        if let Some(main_func) = self.functions.get("main").cloned() {
            result = self.call_function(&main_func, vec![])?;
        }

        Ok(result)
    }

    fn execute_statement(&mut self, stmt: &Statement) -> Result<Value, String> {
        match stmt {
            Statement::VarDecl {
                name,
                var_type: _,
                value,
                line: _,
            } => {
                let val = self.evaluate_expression(value)?;
                self.variables.insert(name.clone(), val.clone());
                Ok(val)
            }

            Statement::Assignment {
                target,
                value,
                line: _,
            } => {
                let val = self.evaluate_expression(value)?;
                self.variables.insert(target.clone(), val.clone());
                Ok(val)
            }

            Statement::IndexAssignment {
                target,
                index,
                value,
                line: _,
            } => {
                let idx = self.evaluate_expression(index)?.to_int() as usize;
                let val = self.evaluate_expression(value)?;

                if let Some(Value::Array(ref mut arr)) = self.variables.get_mut(target) {
                    if idx < arr.len() {
                        arr[idx] = val.clone();
                    }
                }
                Ok(val)
            }

            Statement::If {
                condition,
                then_body,
                else_body,
                line: _,
            } => {
                let cond = self.evaluate_expression(condition)?;
                if cond.to_bool() {
                    let mut result = Value::Null;
                    for s in then_body {
                        result = self.execute_statement(s)?;
                    }
                    Ok(result)
                } else if let Some(else_stmts) = else_body {
                    let mut result = Value::Null;
                    for s in else_stmts {
                        result = self.execute_statement(s)?;
                    }
                    Ok(result)
                } else {
                    Ok(Value::Null)
                }
            }

            Statement::For {
                init,
                condition,
                update,
                body,
                line: _,
            } => {
                if let Some(init_stmt) = init {
                    self.execute_statement(init_stmt)?;
                }

                let mut result = Value::Null;
                loop {
                    let cond = self.evaluate_expression(condition)?;
                    if !cond.to_bool() {
                        break;
                    }

                    for s in body {
                        result = self.execute_statement(s)?;
                    }

                    if let Some(upd) = update {
                        self.evaluate_expression(upd)?;
                    }
                }
                Ok(result)
            }

            Statement::While {
                condition,
                body,
                line: _,
            } => {
                let mut result = Value::Null;
                loop {
                    let cond = self.evaluate_expression(condition)?;
                    if !cond.to_bool() {
                        break;
                    }

                    for s in body {
                        result = self.execute_statement(s)?;
                    }
                }
                Ok(result)
            }

            Statement::DoWhile {
                body,
                condition,
                line: _,
            } => {
                let mut result = Value::Null;
                loop {
                    for s in body {
                        result = self.execute_statement(s)?;
                    }

                    let cond = self.evaluate_expression(condition)?;
                    if !cond.to_bool() {
                        break;
                    }
                }
                Ok(result)
            }

            Statement::Break { line: _ } => Ok(Value::Null),
            Statement::Continue { line: _ } => Ok(Value::Null),

            Statement::Return {
                value,
                line: _,
            } => {
                if let Some(val_expr) = value {
                    self.evaluate_expression(val_expr)
                } else {
                    Ok(Value::Null)
                }
            }

            Statement::Expression {
                expr,
                line: _,
            } => self.evaluate_expression(expr),
        }
    }

    fn evaluate_expression(&mut self, expr: &Expression) -> Result<Value, String> {
        match expr {
            Expression::IntLiteral(i) => Ok(Value::Int(*i)),
            Expression::FloatLiteral(f) => Ok(Value::Float(*f)),
            Expression::StringLiteral(s) => Ok(Value::String(s.clone())),
            Expression::BoolLiteral(b) => Ok(Value::Bool(*b)),
            Expression::NullLiteral => Ok(Value::Null),

            Expression::ArrayLiteral(elements) => {
                let mut arr = Vec::new();
                for elem in elements {
                    arr.push(self.evaluate_expression(elem)?);
                }
                Ok(Value::Array(arr))
            }

            Expression::Identifier(name) => {
                Ok(self.variables.get(name).cloned().unwrap_or(Value::Null))
            }

            Expression::Binary { left, op, right } => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;

                match op {
                    BinOp::Add => match (left_val, right_val) {
                        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
                        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
                        (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 + b)),
                        (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + b as f64)),
                        (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                        (Value::String(a), b) => Ok(Value::String(format!("{}{}", a, b.to_string()))),
                        (a, Value::String(b)) => Ok(Value::String(format!("{}{}", a.to_string(), b))),
                        _ => Ok(Value::Null),
                    },

                    BinOp::Sub => match (left_val, right_val) {
                        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
                        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
                        (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 - b)),
                        (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - b as f64)),
                        _ => Ok(Value::Null),
                    },

                    BinOp::Mul => match (left_val, right_val) {
                        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
                        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
                        (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 * b)),
                        (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * b as f64)),
                        _ => Ok(Value::Null),
                    },

                    BinOp::Div => match (left_val, right_val) {
                        (Value::Int(a), Value::Int(b)) if b != 0 => Ok(Value::Int(a / b)),
                        (Value::Float(a), Value::Float(b)) if b != 0.0 => Ok(Value::Float(a / b)),
                        (Value::Int(a), Value::Float(b)) if b != 0.0 => Ok(Value::Float(a as f64 / b)),
                        (Value::Float(a), Value::Int(b)) if b != 0 => Ok(Value::Float(a / b as f64)),
                        _ => Err("Division by zero".to_string()),
                    },

                    BinOp::Mod => match (left_val, right_val) {
                        (Value::Int(a), Value::Int(b)) if b != 0 => Ok(Value::Int(a % b)),
                        _ => Err("Invalid modulo operation".to_string()),
                    },

                    BinOp::Eq => Ok(Value::Bool(left_val == right_val)),
                    BinOp::Ne => Ok(Value::Bool(left_val != right_val)),

                    BinOp::Lt => match (left_val, right_val) {
                        (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
                        (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
                        (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) < b)),
                        (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a < (b as f64))),
                        _ => Ok(Value::Bool(false)),
                    },

                    BinOp::Gt => match (left_val, right_val) {
                        (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
                        (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
                        (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) > b)),
                        (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a > (b as f64))),
                        _ => Ok(Value::Bool(false)),
                    },

                    BinOp::Le => match (left_val, right_val) {
                        (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
                        (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a <= b)),
                        (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) <= b)),
                        (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a <= (b as f64))),
                        _ => Ok(Value::Bool(false)),
                    },

                    BinOp::Ge => match (left_val, right_val) {
                        (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
                        (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a >= b)),
                        (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) >= b)),
                        (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a >= (b as f64))),
                        _ => Ok(Value::Bool(false)),
                    },

                    BinOp::And => Ok(Value::Bool(left_val.to_bool() && right_val.to_bool())),
                    BinOp::Or => Ok(Value::Bool(left_val.to_bool() || right_val.to_bool())),

                    BinOp::BitAnd => Ok(Value::Int(left_val.to_int() & right_val.to_int())),
                    BinOp::BitOr => Ok(Value::Int(left_val.to_int() | right_val.to_int())),
                    BinOp::BitXor => Ok(Value::Int(left_val.to_int() ^ right_val.to_int())),
                    BinOp::LeftShift => Ok(Value::Int(left_val.to_int() << right_val.to_int())),
                    BinOp::RightShift => Ok(Value::Int(left_val.to_int() >> right_val.to_int())),
                }
            }

            Expression::Unary { op, operand } => {
                let val = self.evaluate_expression(operand)?;
                match op {
                    UnOp::Not => Ok(Value::Bool(!val.to_bool())),
                    UnOp::Neg => match val {
                        Value::Int(i) => Ok(Value::Int(-i)),
                        Value::Float(f) => Ok(Value::Float(-f)),
                        _ => Ok(Value::Null),
                    },
                    UnOp::BitNot => Ok(Value::Int(!val.to_int())),
                    UnOp::Pos => Ok(val),
                }
            }

            Expression::Call { func, args } => {
                let mut arg_vals = Vec::new();
                for arg in args {
                    arg_vals.push(self.evaluate_expression(arg)?);
                }

                self.call_builtin(func, arg_vals)
            }

            Expression::Index { target, index } => {
                let target_val = self.evaluate_expression(target)?;
                let idx = self.evaluate_expression(index)?.to_int() as usize;

                if let Value::Array(arr) = target_val {
                    Ok(arr.get(idx).cloned().unwrap_or(Value::Null))
                } else {
                    Ok(Value::Null)
                }
            }

            Expression::Member { .. } => Ok(Value::Null),
        }
    }

    fn call_builtin(&mut self, name: &str, args: Vec<Value>) -> Result<Value, String> {
        match name {
            "print" => {
                for arg in args {
                    print!("{}", arg.to_string());
                }
                Ok(Value::Null)
            }

            "println" => {
                for arg in &args {
                    println!("{}", arg.to_string());
                }
                Ok(Value::Null)
            }

            "len" => {
                if let Some(Value::Array(arr)) = args.first() {
                    Ok(Value::Int(arr.len() as i64))
                } else if let Some(Value::String(s)) = args.first() {
                    Ok(Value::Int(s.len() as i64))
                } else {
                    Ok(Value::Int(0))
                }
            }

            "toInt" => {
                if let Some(val) = args.first() {
                    Ok(Value::Int(val.to_int()))
                } else {
                    Ok(Value::Int(0))
                }
            }

            "toFloat" => {
                if let Some(val) = args.first() {
                    Ok(Value::Float(val.to_float()))
                } else {
                    Ok(Value::Float(0.0))
                }
            }

            "toString" => {
                if let Some(val) = args.first() {
                    Ok(Value::String(val.to_string()))
                } else {
                    Ok(Value::String("null".to_string()))
                }
            }

            "toBool" => {
                if let Some(val) = args.first() {
                    Ok(Value::Bool(val.to_bool()))
                } else {
                    Ok(Value::Bool(false))
                }
            }

            "abs" => {
                if let Some(val) = args.first() {
                    match val {
                        Value::Int(i) => Ok(Value::Int(i.abs())),
                        Value::Float(f) => Ok(Value::Float(f.abs())),
                        _ => Ok(Value::Null),
                    }
                } else {
                    Ok(Value::Null)
                }
            }

            "sqrt" => {
                if let Some(val) = args.first() {
                    Ok(Value::Float(val.to_float().sqrt()))
                } else {
                    Ok(Value::Null)
                }
            }

            "pow" => {
                if let (Some(base), Some(exp)) = (args.get(0), args.get(1)) {
                    Ok(Value::Float(base.to_float().powf(exp.to_float())))
                } else {
                    Ok(Value::Null)
                }
            }

            "floor" => {
                if let Some(val) = args.first() {
                    Ok(Value::Float(val.to_float().floor()))
                } else {
                    Ok(Value::Null)
                }
            }

            "ceil" => {
                if let Some(val) = args.first() {
                    Ok(Value::Float(val.to_float().ceil()))
                } else {
                    Ok(Value::Null)
                }
            }

            "round" => {
                if let Some(val) = args.first() {
                    Ok(Value::Float(val.to_float().round()))
                } else {
                    Ok(Value::Null)
                }
            }

            "min" => {
                if let (Some(a), Some(b)) = (args.get(0), args.get(1)) {
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => Ok(Value::Int(*x.min(y))),
                        (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x.min(*y))),
                        _ => Ok(Value::Null),
                    }
                } else {
                    Ok(Value::Null)
                }
            }

            "max" => {
                if let (Some(a), Some(b)) = (args.get(0), args.get(1)) {
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => Ok(Value::Int(*x.max(y))),
                        (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x.max(*y))),
                        _ => Ok(Value::Null),
                    }
                } else {
                    Ok(Value::Null)
                }
            }

            _ => Err(format!("Unknown function: {}", name)),
        }
    }

    fn call_function(&mut self, func: &FunctionDecl, args: Vec<Value>) -> Result<Value, String> {
        let old_vars = self.variables.clone();

        for (i, (param_name, _)) in func.params.iter().enumerate() {
            if let Some(arg) = args.get(i) {
                self.variables.insert(param_name.clone(), arg.clone());
            }
        }

        let mut result = Value::Null;
        for stmt in &func.body {
            result = self.execute_statement(stmt)?;
            if let Statement::Return { .. } = stmt {
                break;
            }
        }

        self.variables = old_vars;
        Ok(result)
    }
}
