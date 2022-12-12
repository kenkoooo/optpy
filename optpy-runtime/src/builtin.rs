use std::{io::stdin, rc::Rc};

use crate::{number::Number, value::Value, ImmutableString, Iter};

pub fn input() -> Value {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    Value::from(buf.trim())
}

pub fn map_int(value: &Value) -> Value {
    match value {
        Value::List(list) => {
            let list = list.0.borrow().clone();
            let iter = list.into_iter().map(|v| int(&v.borrow()));
            Value::Iter(Iter::new(Box::new(iter)))
        }
        _ => unreachable!(),
    }
}
pub fn int(value: &Value) -> Value {
    match value {
        Value::String(s) => Value::Number(Number::Int64(s.0.parse::<i64>().expect("non-integer"))),
        Value::Number(Number::Int64(i)) => Value::Number(Number::Int64(*i)),
        _ => unreachable!(),
    }
}
pub fn float(value: &Value) -> Value {
    match value {
        Value::String(s) => Value::Number(Number::Float(s.0.parse::<f64>().expect("non-float"))),
        Value::Number(Number::Int64(i)) => Value::Number(Number::Float(*i as f64)),
        _ => unreachable!(),
    }
}
pub fn str(value: &Value) -> Value {
    match value {
        Value::String(_) => value.clone(),
        Value::Number(n) => Value::String(ImmutableString(Rc::new(n.to_string()))),
        _ => todo!(),
    }
}

pub fn list(value: &Value) -> Value {
    match value {
        Value::List(list) => {
            let vec = list
                .0
                .borrow()
                .iter()
                .map(|v| v.borrow().clone())
                .collect::<Vec<_>>();
            Value::from(vec)
        }
        Value::Iter(iter) => iter.__list(),
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
            let iter = (*start..*stop).map(|i| Value::Number(Number::Int64(i)));
            Value::Iter(Iter::new(Box::new(iter)))
        }
        _ => unreachable!(),
    }
}
pub fn __min1(list: &Value) -> Value {
    match list {
        Value::List(list) => list
            .0
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
            .0
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
            .0
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
    let cloned_value = list(value);
    cloned_value.sort();
    cloned_value
}

pub fn len(value: &Value) -> Value {
    value.__len()
}
pub fn any(value: &Value) -> Value {
    match value {
        Value::List(list) => Value::Boolean(list.0.borrow().iter().any(|v| v.borrow().test())),
        Value::Iter(iter) => Value::Boolean(iter.0.borrow_mut().any(|v| v.test())),
        _ => todo!(),
    }
}
pub fn all(value: &Value) -> Value {
    match value {
        Value::List(list) => Value::Boolean(list.0.borrow().iter().all(|v| v.borrow().test())),
        Value::Iter(iter) => Value::Boolean(iter.0.borrow_mut().all(|v| v.test())),
        _ => todo!(),
    }
}
pub fn __set1(iter: &Value) -> Value {
    match iter {
        Value::List(list) => {
            let pairs = list
                .0
                .borrow()
                .iter()
                .map(|v| (v.borrow().clone(), Value::None))
                .collect::<Vec<_>>();
            Value::dict(pairs)
        }
        Value::Dict(dict) => __set1(&dict.keys()),
        Value::String(_) => todo!(),
        _ => unreachable!(),
    }
}

pub fn enumerate(iter: &Value) -> Value {
    match iter {
        Value::List(list) => {
            let list = list
                .0
                .borrow()
                .iter()
                .enumerate()
                .map(|(i, v)| Value::from(vec![Value::from(i as i64), v.borrow().clone()]))
                .collect::<Vec<_>>();
            Value::from(list)
        }
        iter => enumerate(&list(iter)),
    }
}
pub fn __next1(iter: &Value) -> Value {
    match iter {
        Value::Iter(iter) => iter.__next().expect("stop iteration"),
        Value::List(list) => {
            let head = list.0.borrow_mut().remove(0);
            head.borrow().clone()
        }
        _ => todo!("{:?}", iter),
    }
}
pub fn __next2(iter: &Value, default: &Value) -> Value {
    match iter {
        Value::Iter(iter) => match iter.__next() {
            Some(value) => value,
            None => default.clone(),
        },
        _ => todo!(),
    }
}
pub fn iter(iter: &Value) -> Value {
    match iter {
        Value::List(list) => list.__iter(),
        Value::Iter(_) => iter.clone(),
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
    Value::Dict(Default::default())
}

pub fn dict() -> Value {
    Value::Dict(Default::default())
}

pub fn abs(v: &Value) -> Value {
    match v {
        Value::Number(n) => Value::Number(n.abs()),
        _ => unreachable!(),
    }
}
#[macro_export]
macro_rules! range {
    ($stop:expr) => {
        __range1($stop)
    };
    ($start:expr, $stop:expr) => {
        __range2($start, $stop)
    };
}

#[macro_export]
macro_rules! print_values {
    ($($arg:expr),+) => {
        let s = [$($arg),+].iter().map(|v| v.to_string()).collect::<Vec<_>>();
        println!("{}", s.join(" "));
    };
}

#[macro_export]
macro_rules! pow {
    ($number:expr, $power:expr, $modulus:expr) => {
        __pow3($number, $power, $modulus)
    };
}
#[macro_export]
macro_rules! set {
    () => {
        __set0()
    };
    ($iter:expr) => {
        __set1($iter)
    };
}

#[macro_export]
macro_rules! exit {
    () => {
        __exit0()
    };
    ($code:expr) => {
        __exit1($code)
    };
}

#[macro_export]
macro_rules! max {
    ($e:expr) => {
        __max1($e)
    };
    ($a:expr, $b:expr) => {
        __max2($a, $b)
    };
    ($a:expr, $($arg:expr),+) => {
        __max2($a, &max!($($arg),+))
    };
}

#[macro_export]
macro_rules! min {
    ($e:expr) => {
        __min1($e)
    };
    ($a:expr, $b:expr) => {
        __min2($a, $b)
    };
    ($a:expr, $($arg:expr),+) => {
        __min2($a, &min!($($arg),+))
    };
}

#[macro_export]
macro_rules! sum {
    ($e:expr) => {
        __sum1($e)
    };
    ($a:expr, $b:expr) => {
        __sum2($a, $b)
    };
    ($a:expr, $($arg:expr),+) => {
        __sum2($a, &sum!($($arg),+))
    };
}

#[macro_export]
macro_rules! next {
    ($e:expr) => {
        __next1($e)
    };
    ($a:expr, $b:expr) => {
        __next2($a, $b)
    };
}
