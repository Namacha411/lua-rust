use crate::vm::ExeState;
use std::fmt;

#[derive(Clone)]
pub enum Value {
    Nil,
    Boolean(bool),
    Interger(i64),
    Float(f64),
    String(String),
    Function(fn(&mut ExeState) -> i32),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(b) => write!(f, "{b}"),
            Value::Interger(i) => write!(f, "{i}"),
            Value::Float(n) => write!(f, "{n:?}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Function(_) => write!(f, "function"),
        }
    }
}
