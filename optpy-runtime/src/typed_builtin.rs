use crate::{
    number::Number,
    typed_value::{TypedList, TypedString, TypedValue},
};

pub fn len<T: TypedValue>(x: &T) -> Number {
    x.__len()
}

pub fn list<T: TypedValue>(x: &TypedList<T>) -> TypedList<T> {
    x.__list()
}

pub fn abs<T: TypedValue>(x: &T) -> Number {
    x.__abs()
}
pub fn __range1<T: TypedValue>(x: &T) -> TypedList<Number> {
    let to = x.__as_number();
    match to {
        Number::Int64(i) => {
            let list = (0..i).map(|i| Number::from(i)).collect::<Vec<_>>();
            TypedList::from(list)
        }
        _ => unreachable!(),
    }
}

pub fn __range2<T: TypedValue, U: TypedValue>(from: &T, to: &U) -> TypedList<Number> {
    let from = from.__as_number();
    let to = to.__as_number();
    match (from, to) {
        (Number::Int64(from), Number::Int64(to)) => {
            let list = (from..to).map(|i| Number::from(i)).collect::<Vec<_>>();
            TypedList::from(list)
        }
        _ => unreachable!(),
    }
}

pub fn __min2<T: TypedValue, U: TypedValue>(a: &T, b: &U) -> impl TypedValue {
    a.__min(b)
}

pub fn map_int<T: TypedValue>(_: &T) -> TypedList<Number> {
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
