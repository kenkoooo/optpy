use crate::{
    number::Number,
    typed_value::{TypedList, TypedString, TypedValue},
};

pub fn len<T>(x: TypedList<T>) -> Number {
    x.__len()
}

pub fn list<T: TypedValue>(x: TypedList<T>) -> TypedList<T> {
    x.__list()
}

pub fn abs(x: Number) -> Number {
    x.abs()
}
pub fn __range1(x: Number) -> TypedList<Number> {
    match x {
        Number::Int64(i) => {
            let list = (0..i).map(|i| Number::from(i)).collect::<Vec<_>>();
            TypedList::from(list)
        }
        _ => unreachable!(),
    }
}

pub fn __range2(from: Number, to: Number) -> TypedList<Number> {
    match (from, to) {
        (Number::Int64(from), Number::Int64(to)) => {
            let list = (from..to).map(|i| Number::from(i)).collect::<Vec<_>>();
            TypedList::from(list)
        }
        _ => unreachable!(),
    }
}

pub fn __min2(a: Number, b: Number) -> Number {
    a.__min(b)
}

pub fn map_int(_: TypedList<TypedString>) -> TypedList<Number> {
    todo!()
}

pub fn input() -> TypedString {
    todo!()
}

#[macro_export]
macro_rules! typed_range {
    ($stop:expr) => {
        __range1($stop)
    };
    ($start:expr, $stop:expr) => {
        __range2($start, $stop)
    };
}

#[macro_export]
macro_rules! typed_print_values {
    ($($arg:expr),+) => {
        let s = [$($arg),+].iter().map(|v| v.to_string()).collect::<Vec<_>>();
        println!("{}", s.join(" "));
    };
}

#[macro_export]
macro_rules! typed_pow {
    ($number:expr, $power:expr, $modulus:expr) => {
        __pow3($number, $power, $modulus)
    };
}
#[macro_export]
macro_rules! typed_set {
    () => {
        __set0()
    };
    ($iter:expr) => {
        __set1($iter)
    };
}

#[macro_export]
macro_rules! typed_exit {
    () => {
        __exit0()
    };
    ($code:expr) => {
        __exit1($code)
    };
}

#[macro_export]
macro_rules! typed_max {
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
macro_rules! typed_min {
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
macro_rules! typed_sum {
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
