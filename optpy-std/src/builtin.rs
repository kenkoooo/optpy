use std::{collections::HashMap, io::stdin, rc::Rc};

use crate::{cell::UnsafeRefCell, number::Number, value::Value};

fn rc_unsafe_ref_cell<T>(v: T) -> Rc<UnsafeRefCell<T>> {
    Rc::new(UnsafeRefCell::new(v))
}

pub fn input() -> Value {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    Value::from(buf.trim())
}

pub fn map_int(value: &Value) -> Value {
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
pub fn int(value: &Value) -> Value {
    match value {
        Value::String(s) => Value::Number(Number::Int64(s.parse::<i64>().expect("non-integer"))),
        Value::Number(Number::Int64(i)) => Value::Number(Number::Int64(*i)),
        _ => unreachable!(),
    }
}
pub fn str(value: &Value) -> Value {
    match value {
        Value::String(_) => value.clone(),
        Value::Number(n) => Value::String(Rc::new(n.to_string())),
        _ => todo!(),
    }
}

pub fn list(value: &Value) -> Value {
    match value {
        Value::List(_) => value.clone(),
        _ => todo!(),
    }
}
pub fn tuple(value: &Value) -> Value {
    list(value)
}

pub fn __range1(value: &Value) -> Value {
    __range2(&Value::Number(Number::Int64(0)), value)
}

pub fn __range2(start: &Value, stop: &Value) -> Value {
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
pub fn __min1(a: &Value) -> Value {
    match a {
        Value::List(list) => list
            .borrow()
            .iter()
            .min_by(|a, b| a.borrow().partial_cmp(&b.borrow()).unwrap())
            .unwrap()
            .borrow()
            .clone(),
        _ => todo!(),
    }
}
pub fn __min2(a: &Value, b: &Value) -> Value {
    if a > b {
        b.clone()
    } else {
        a.clone()
    }
}
pub fn __max1(a: &Value) -> Value {
    match a {
        Value::List(list) => list
            .borrow()
            .iter()
            .max_by(|a, b| a.borrow().partial_cmp(&b.borrow()).unwrap())
            .unwrap()
            .borrow()
            .clone(),
        _ => todo!(),
    }
}
pub fn __max2(a: &Value, b: &Value) -> Value {
    if a > b {
        a.clone()
    } else {
        b.clone()
    }
}
pub fn __sum1(a: &Value) -> Value {
    match a {
        Value::List(list) => list
            .borrow()
            .iter()
            .fold(Value::from(0), |a, b| a.__add(&b.borrow())),
        _ => todo!(),
    }
}
pub fn __sum2(a: &Value, b: &Value) -> Value {
    a.__add(b)
}

pub fn sorted(value: &Value) -> Value {
    value.sort();
    value.clone()
}

pub fn len(value: &Value) -> Value {
    value.__len()
}
pub fn any(value: &Value) -> Value {
    match value {
        Value::List(list) => Value::Boolean(list.borrow().iter().any(|v| v.borrow().test())),
        _ => todo!(),
    }
}
pub fn all(value: &Value) -> Value {
    match value {
        Value::List(list) => Value::Boolean(list.borrow().iter().all(|v| v.borrow().test())),
        _ => todo!(),
    }
}
pub fn __set1(iter: &Value) -> Value {
    match iter {
        Value::List(list) => {
            let map = list
                .borrow()
                .iter()
                .map(|v| (v.borrow().__as_dict_key(), rc_unsafe_ref_cell(Value::None)))
                .collect::<HashMap<_, _>>();
            Value::Dict(rc_unsafe_ref_cell(map))
        }
        Value::Dict(map) => {
            let map = map
                .borrow()
                .keys()
                .map(|key| (key.clone(), rc_unsafe_ref_cell(Value::None)))
                .collect::<HashMap<_, _>>();
            Value::Dict(rc_unsafe_ref_cell(map))
        }
        Value::String(_) => todo!(),
        _ => unreachable!(),
    }
}

pub fn enumerate(iter: &Value) -> Value {
    match iter {
        Value::List(list) => {
            let list = list
                .borrow()
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    rc_unsafe_ref_cell(Value::from(vec![Value::from(i as i64), v.borrow().clone()]))
                })
                .collect();
            Value::List(rc_unsafe_ref_cell(list))
        }
        iter => enumerate(&list(iter)),
    }
}
pub fn next(iter: &Value) -> Value {
    match iter {
        Value::List(list) => {
            let head = list.borrow_mut().remove(0);
            head.borrow().clone()
        }
        _ => todo!(),
    }
}

pub fn __pow3(number: &Value, power: &Value, modulus: &Value) -> Value {
    let int = |n: &Value| match n.__number() {
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
    Value::Number(Number::Int64(result))
}

pub fn __exit1(code: &Value) -> ! {
    match code.__number() {
        Number::Int64(code) => std::process::exit(code as i32),
        _ => unreachable!(),
    }
}
pub fn __exit0() -> ! {
    std::process::exit(0)
}

pub fn __set0() -> Value {
    Value::Dict(rc_unsafe_ref_cell(HashMap::new()))
}

pub fn dict() -> Value {
    Value::Dict(rc_unsafe_ref_cell(HashMap::new()))
}
