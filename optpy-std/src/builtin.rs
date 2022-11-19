use std::{collections::HashMap, rc::Rc};

use crate::{cell::UnsafeRefCell, number::Number, value::Value, Object};

mod value {
    use std::{collections::HashMap, io::stdin, rc::Rc};

    use crate::{cell::UnsafeRefCell, number::Number, value::Value};

    pub(super) fn input() -> Value {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        Value::from(buf.trim())
    }

    pub(super) fn map_int(value: &Value) -> Value {
        match value {
            Value::List(list) => {
                let list = list
                    .borrow()
                    .iter()
                    .map(|v| int(&v.borrow()))
                    .collect::<Vec<_>>();
                Value::from(list)
            }
            _ => unreachable!(),
        }
    }
    pub(super) fn int(value: &Value) -> Value {
        match value {
            Value::String(s) => {
                Value::Number(Number::Int64(s.parse::<i64>().expect("non-integer")))
            }
            Value::Number(Number::Int64(i)) => Value::Number(Number::Int64(*i)),
            _ => unreachable!(),
        }
    }

    pub(super) fn list(value: &Value) -> Value {
        match value {
            Value::List(_) => value.clone(),
            _ => todo!(),
        }
    }

    pub(super) fn __range1(value: &Value) -> Value {
        __range2(&Value::Number(Number::Int64(0)), value)
    }

    pub(super) fn __range2(start: &Value, stop: &Value) -> Value {
        match (start, stop) {
            (Value::Number(Number::Int64(start)), Value::Number(Number::Int64(stop))) => {
                let list = (*start..*stop)
                    .map(|i| Value::Number(Number::Int64(i)))
                    .collect::<Vec<_>>();
                Value::from(list)
            }
            _ => unreachable!(),
        }
    }

    pub(super) fn sorted(value: &Value) -> Value {
        value.sort();
        value.clone()
    }

    pub(super) fn len(value: &Value) -> Value {
        value.__len()
    }
    pub(super) fn __set1(iter: &Value) -> Value {
        match iter {
            Value::List(list) => {
                let map = list
                    .borrow()
                    .iter()
                    .map(|v| {
                        (
                            v.borrow().__as_dict_key(),
                            Rc::new(UnsafeRefCell::new(Value::None)),
                        )
                    })
                    .collect::<HashMap<_, _>>();
                Value::Dict(Rc::new(UnsafeRefCell::new(map)))
            }
            Value::Dict(map) => {
                let map = map
                    .borrow()
                    .keys()
                    .map(|key| (key.clone(), Rc::new(UnsafeRefCell::new(Value::None))))
                    .collect::<HashMap<_, _>>();
                Value::Dict(Rc::new(UnsafeRefCell::new(map)))
            }
            Value::String(_) => todo!(),
            _ => unreachable!(),
        }
    }
}

fn map0<F: Fn(&Value) -> Value>(obj: &Object, f: F) -> Object {
    let value = match obj {
        Object::Ref(r) => f(&r.borrow()),
        Object::Value(v) => f(v),
    };
    Object::Value(value)
}

macro_rules! define_map0 {
    ($name:ident) => {
        pub fn $name(obj: &Object) -> Object {
            map0(obj, value::$name)
        }
    };
}
define_map0!(len);
define_map0!(sorted);
define_map0!(__range1);
define_map0!(list);
define_map0!(int);
define_map0!(map_int);
define_map0!(__set1);

fn map1<F: Fn(&Value, &Value) -> Value>(obj1: &Object, obj2: &Object, f: F) -> Object {
    let value = match (obj1, obj2) {
        (Object::Ref(obj1), Object::Ref(obj2)) => f(&obj1.borrow(), &obj2.borrow()),
        (Object::Ref(obj1), Object::Value(obj2)) => f(&obj1.borrow(), obj2),
        (Object::Value(obj1), Object::Ref(obj2)) => f(obj1, &obj2.borrow()),
        (Object::Value(obj1), Object::Value(obj2)) => f(obj1, obj2),
    };
    Object::Value(value)
}

macro_rules! define_map1 {
    ($name:ident) => {
        pub fn $name(obj1: &Object, obj2: &Object) -> Object {
            map1(obj1, obj2, value::$name)
        }
    };
}
define_map1!(__range2);

pub fn __pow3(number: &Object, power: &Object, modulus: &Object) -> Object {
    let number = number.__number();
    let power = power.__number();
    let modulus = modulus.__number();
    let int = |n: Number| match n {
        Number::Int64(i) => i,
        Number::Float(_) => unreachable!(),
    };

    let number = int(number);
    let power = int(power);
    let modulus = int(modulus);

    let mut result = 1;
    let mut cur = number;
    let mut e = power;
    while e > 0 {
        if e & 1 == 1 {
            result = (result * cur) % modulus;
        }
        cur = (cur * cur) % modulus;
        e >>= 1;
    }
    Object::Value(Value::Number(Number::Int64(result)))
}

pub fn input() -> Object {
    Object::Value(value::input())
}

pub fn exit(code: &Object) -> ! {
    match code.__number() {
        Number::Int64(code) => std::process::exit(code as i32),
        _ => unreachable!(),
    }
}

pub fn __set0() -> Object {
    Object::Value(Value::Dict(Rc::new(UnsafeRefCell::new(HashMap::new()))))
}
