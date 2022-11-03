pub mod value {
    use std::rc::Rc;

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum Value {
        List(Vec<Value>),
        Tuple(Vec<Value>),
        String(Rc<String>),
        None,
    }

    impl Value {
        pub fn split(&self) -> Self {
            match self {
                Value::String(s) => Self::List(
                    s.split_whitespace()
                        .map(|s| Self::String(Rc::new(s.to_string())))
                        .collect(),
                ),
                _ => panic!("undefined method"),
            }
        }

        pub fn shallow_copy(&self) -> Self {
            todo!()
        }
        pub fn assign(&self, value: Value) {
            todo!()
        }

        pub fn tuple(tuple: &[Value]) -> Self {
            Self::Tuple(tuple.iter().map(|v| v.shallow_copy()).collect())
        }
    }

    impl<S: AsRef<str>> From<S> for Value {
        fn from(s: S) -> Self {
            Value::String(Rc::new(s.as_ref().to_string()))
        }
    }
}

pub mod builtin {
    use std::{io::stdin, rc::Rc};

    use crate::value::Value;

    pub fn input() -> Value {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        Value::String(Rc::new(buf.trim().to_string()))
    }
}
