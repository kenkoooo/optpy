use std::{collections::HashMap, rc::Rc};

use crate::{cell::UnsafeRefCell, number::Number, object::Object, value::Value};

fn rc_unsafe_ref_cell<T>(v: T) -> Rc<UnsafeRefCell<T>> {
    Rc::new(UnsafeRefCell::new(v))
}

mod value {
    use std::{collections::HashMap, io::stdin, rc::Rc};

    use crate::{number::Number, value::Value};

    use super::rc_unsafe_ref_cell;

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
    pub(super) fn str(value: &Value) -> Value {
        match value {
            Value::String(_) => value.clone(),
            Value::Number(n) => Value::String(Rc::new(n.to_string())),
            _ => todo!(),
        }
    }

    pub(super) fn list(value: &Value) -> Value {
        match value {
            Value::List(_) => value.clone(),
            _ => todo!(),
        }
    }
    pub(super) fn tuple(value: &Value) -> Value {
        list(value)
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
    pub(super) fn __min1(a: &Value) -> Value {
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
    pub(super) fn __min2(a: &Value, b: &Value) -> Value {
        if a > b {
            b.clone()
        } else {
            a.clone()
        }
    }
    pub(super) fn __max1(a: &Value) -> Value {
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
    pub(super) fn __max2(a: &Value, b: &Value) -> Value {
        if a > b {
            a.clone()
        } else {
            b.clone()
        }
    }
    pub(super) fn __sum1(a: &Value) -> Value {
        match a {
            Value::List(list) => list
                .borrow()
                .iter()
                .fold(Value::from(0), |a, b| a.__add(&b.borrow())),
            _ => todo!(),
        }
    }
    pub(super) fn __sum2(a: &Value, b: &Value) -> Value {
        a.__add(b)
    }

    pub(super) fn sorted(value: &Value) -> Value {
        value.sort();
        value.clone()
    }

    pub(super) fn len(value: &Value) -> Value {
        value.__len()
    }
    pub(super) fn any(value: &Value) -> Value {
        match value {
            Value::List(list) => Value::Boolean(list.borrow().iter().any(|v| v.borrow().test())),
            _ => todo!(),
        }
    }
    pub(super) fn all(value: &Value) -> Value {
        match value {
            Value::List(list) => Value::Boolean(list.borrow().iter().all(|v| v.borrow().test())),
            _ => todo!(),
        }
    }
    pub(super) fn __set1(iter: &Value) -> Value {
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

    pub(super) fn enumerate(iter: &Value) -> Value {
        match iter {
            Value::List(list) => {
                let list = list
                    .borrow()
                    .iter()
                    .enumerate()
                    .map(|(i, v)| {
                        rc_unsafe_ref_cell(Value::from(vec![
                            Value::from(i as i64),
                            v.borrow().clone(),
                        ]))
                    })
                    .collect();
                Value::List(rc_unsafe_ref_cell(list))
            }
            iter => enumerate(&list(iter)),
        }
    }
    pub(super) fn next(iter: &Value) -> Value {
        match iter {
            Value::List(list) => {
                let head = list.borrow_mut().remove(0);
                head.borrow().clone()
            }
            _ => todo!(),
        }
    }
}

fn map_1_1<F: Fn(&Value) -> Value>(obj: &Object, f: F) -> Object {
    let value = match obj {
        Object::Ref(r) => f(&r.borrow()),
        Object::Value(v) => f(v),
    };
    Object::Value(value)
}

macro_rules! define_map1_1 {
    ($name:ident) => {
        pub fn $name(obj: &Object) -> Object {
            map_1_1(obj, value::$name)
        }
    };
}
define_map1_1!(len);
define_map1_1!(any);
define_map1_1!(all);
define_map1_1!(sorted);
define_map1_1!(__range1);
define_map1_1!(__max1);
define_map1_1!(__min1);
define_map1_1!(__sum1);
define_map1_1!(list);
define_map1_1!(tuple);
define_map1_1!(int);
define_map1_1!(str);
define_map1_1!(map_int);
define_map1_1!(__set1);
define_map1_1!(enumerate);
define_map1_1!(next);

fn map_2_1<F: Fn(&Value, &Value) -> Value>(obj1: &Object, obj2: &Object, f: F) -> Object {
    let value = match (obj1, obj2) {
        (Object::Ref(obj1), Object::Ref(obj2)) => f(&obj1.borrow(), &obj2.borrow()),
        (Object::Ref(obj1), Object::Value(obj2)) => f(&obj1.borrow(), obj2),
        (Object::Value(obj1), Object::Ref(obj2)) => f(obj1, &obj2.borrow()),
        (Object::Value(obj1), Object::Value(obj2)) => f(obj1, obj2),
    };
    Object::Value(value)
}

macro_rules! define_map2_1 {
    ($name:ident) => {
        pub fn $name(obj1: &Object, obj2: &Object) -> Object {
            map_2_1(obj1, obj2, value::$name)
        }
    };
}
define_map2_1!(__range2);
define_map2_1!(__min2);
define_map2_1!(__max2);
define_map2_1!(__sum2);

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

pub fn __exit1(code: &Object) -> ! {
    match code.__number() {
        Number::Int64(code) => std::process::exit(code as i32),
        _ => unreachable!(),
    }
}
pub fn __exit0() -> ! {
    std::process::exit(0)
}

pub fn __set0() -> Object {
    Object::Value(Value::Dict(rc_unsafe_ref_cell(HashMap::new())))
}

pub fn dict() -> Object {
    Object::Value(Value::Dict(rc_unsafe_ref_cell(HashMap::new())))
}
