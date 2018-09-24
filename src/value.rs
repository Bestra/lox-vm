use std::fmt;
#[derive(Clone, Copy)]
pub enum Value {
    Bool(bool),
    Nil,
    Number(f64),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Value::Bool(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::Nil => write!(f, "nil"),
        }
    }
}

pub type ValueArray = Vec<Value>;

pub fn print_value(v: Value) {
    print!("{}", v);
}
