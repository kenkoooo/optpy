use std::{collections::HashMap, hash::Hash, io::stdin, rc::Rc};

use crate::{
    cell::UnsafeRefCell,
    number::Number,
    typed_object::{
        AddValue, BooleanValue, DictValue, ListValue, NoneValue, NumberValue, Object, StringValue,
        Value,
    },
};

pub fn __range1(stop: &Object<NumberValue>) -> Object<ListValue<NumberValue>> {
    __range2(&Object::Value(NumberValue(Number::Int64(0))), stop)
}
pub fn __range2(
    start: &Object<NumberValue>,
    stop: &Object<NumberValue>,
) -> Object<ListValue<NumberValue>> {
    let start = start.__inner().0.__int();
    let stop = stop.__inner().0.__int();
    let list = (start..stop)
        .map(|i| Rc::new(UnsafeRefCell::new(NumberValue(Number::Int64(i)))))
        .collect::<Vec<_>>();
    Object::Value(ListValue(Rc::new(UnsafeRefCell::new(list))))
}

pub fn __pow3(
    number: &Object<NumberValue>,
    power: &Object<NumberValue>,
    modulus: &Object<NumberValue>,
) -> Object<NumberValue> {
    let number = number.__inner().0.__int();
    let power = power.__inner().0.__int();
    let modulus = modulus.__inner().0.__int();
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
    Object::Value(NumberValue(Number::Int64(result)))
}

pub fn __set0<K: Eq + Hash>() -> Object<DictValue<K, NoneValue>> {
    Object::Value(DictValue(Rc::new(UnsafeRefCell::new(HashMap::new()))))
}

pub fn input() -> Object<StringValue> {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    Object::Value(StringValue(Rc::new(buf.trim().to_string())))
}
pub fn int<T>(value: &Object<T>) -> Object<NumberValue>
where
    i64: From<T>,
    T: Value,
{
    Object::Value(NumberValue(Number::Int64(i64::from(value.__inner()))))
}
pub fn map_int<T>(value: &Object<ListValue<T>>) -> Object<ListValue<NumberValue>>
where
    i64: From<T>,
    T: Value,
{
    let list = value
        .__inner()
        .0
        .borrow()
        .iter()
        .map(|v| {
            Rc::new(UnsafeRefCell::new(NumberValue(Number::Int64(i64::from(
                v.borrow().clone(),
            )))))
        })
        .collect::<Vec<_>>();
    Object::Value(ListValue(Rc::new(UnsafeRefCell::new(list))))
}

pub fn __min1<T: PartialOrd + Value>(list: &Object<ListValue<T>>) -> Object<T> {
    let min = list
        .__inner()
        .0
        .borrow()
        .iter()
        .min_by(|a, b| a.borrow().partial_cmp(&b.borrow()).unwrap())
        .unwrap()
        .borrow()
        .clone();
    Object::Value(min)
}

pub fn __min2<T: PartialOrd + Clone>(a: &Object<T>, b: &Object<T>) -> Object<T> {
    if a > b {
        b.clone()
    } else {
        a.clone()
    }
}

pub fn __max1<T: PartialOrd + Value>(list: &Object<ListValue<T>>) -> Object<T> {
    let max = list
        .__inner()
        .0
        .borrow()
        .iter()
        .max_by(|a, b| a.borrow().partial_cmp(&b.borrow()).unwrap())
        .unwrap()
        .borrow()
        .clone();
    Object::Value(max)
}

pub fn __max2<T: PartialOrd + Clone>(a: &Object<T>, b: &Object<T>) -> Object<T> {
    if a < b {
        b.clone()
    } else {
        a.clone()
    }
}

pub fn __sum1(list: Object<ListValue<NumberValue>>) -> Object<NumberValue> {
    let sum = list
        .__inner()
        .0
        .borrow()
        .iter()
        .fold(Number::Int64(0), |sum, a| sum + a.borrow().0.clone());
    Object::Value(NumberValue(sum))
}

pub fn __sum2<T>(a: &Object<T>, b: &Object<T>) -> Object<T>
where
    Object<T>: AddValue<T>,
{
    a.__add(b)
}

pub fn sorted<T: Value + PartialOrd>(list: &Object<ListValue<T>>) -> Object<ListValue<T>> {
    list.__inner()
        .0
        .borrow_mut()
        .sort_by(|a, b| a.borrow().partial_cmp(&b.borrow()).unwrap());
    list.clone()
}

pub fn any<T>(list: &Object<ListValue<BooleanValue>>) -> Object<BooleanValue> {
    let result = list.__inner().0.borrow().iter().any(|v| v.borrow().test());
    Object::Value(BooleanValue(result))
}

pub fn all<T>(list: &Object<ListValue<BooleanValue>>) -> Object<BooleanValue> {
    let result = list.__inner().0.borrow().iter().all(|v| v.borrow().test());
    Object::Value(BooleanValue(result))
}
