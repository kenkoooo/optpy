mod typed_builtin {
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
}
mod cell {
    use std::{
        cell::UnsafeCell,
        fmt::Debug,
        ops::{Deref, DerefMut},
        ptr::NonNull,
        rc::Rc,
    };

    pub struct UnsafeRef<T: ?Sized> {
        value: NonNull<T>,
    }
    impl<T: ?Sized> Deref for UnsafeRef<T> {
        type Target = T;

        #[inline]
        fn deref(&self) -> &T {
            unsafe { self.value.as_ref() }
        }
    }
    pub struct UnsafeRefMut<T: ?Sized> {
        value: NonNull<T>,
    }

    impl<T: ?Sized> Deref for UnsafeRefMut<T> {
        type Target = T;

        #[inline]
        fn deref(&self) -> &T {
            unsafe { self.value.as_ref() }
        }
    }

    impl<T: ?Sized> DerefMut for UnsafeRefMut<T> {
        #[inline]
        fn deref_mut(&mut self) -> &mut T {
            unsafe { self.value.as_mut() }
        }
    }

    impl<T: ?Sized + PartialEq> PartialEq<T> for UnsafeRef<T> {
        fn eq(&self, other: &T) -> bool {
            self.deref() == other
        }
    }

    pub struct UnsafeRefCell<T> {
        cell: UnsafeCell<T>,
    }
    impl<T: Debug> Debug for UnsafeRefCell<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.borrow().fmt(f)
        }
    }

    impl<T: PartialOrd> PartialOrd for UnsafeRefCell<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.borrow().partial_cmp(&other.borrow())
        }
    }

    impl<T: PartialEq> PartialEq for UnsafeRefCell<T> {
        fn eq(&self, other: &Self) -> bool {
            self.borrow().eq(&other.borrow())
        }
    }

    impl<T> UnsafeRefCell<T> {
        pub fn new(value: T) -> UnsafeRefCell<T> {
            Self {
                cell: UnsafeCell::new(value),
            }
        }
        pub fn rc(value: T) -> Rc<UnsafeRefCell<T>> {
            Rc::new(Self::new(value))
        }
        pub fn borrow(&self) -> UnsafeRef<T> {
            let value = unsafe { NonNull::new_unchecked(self.cell.get()) };
            UnsafeRef { value }
        }
        pub fn borrow_mut(&self) -> UnsafeRefMut<T> {
            let value = unsafe { NonNull::new_unchecked(self.cell.get()) };
            UnsafeRefMut { value }
        }
    }
}
mod number {
    use std::{
        hash::Hash,
        ops::{Add, Div, Mul, Rem, Sub},
    };

    #[derive(Debug, Clone, Copy)]
    pub enum Number {
        Int64(i64),
        Float(f64),
    }
    impl Hash for Number {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            match self {
                Number::Int64(i) => i.hash(state),
                Number::Float(_) => todo!(),
            }
        }
    }
    impl Eq for Number {}

    impl PartialOrd for Number {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match (self, other) {
                (Number::Int64(l0), Number::Int64(r0)) => l0.partial_cmp(r0),
                (Number::Float(l0), Number::Float(r0)) => l0.partial_cmp(r0),
                (Number::Int64(l0), Number::Float(r0)) => (*l0 as f64).partial_cmp(r0),
                (Number::Float(l0), Number::Int64(r0)) => l0.partial_cmp(&(*r0 as f64)),
            }
        }
    }
    impl PartialEq for Number {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Number::Int64(l0), Number::Int64(r0)) => l0.eq(r0),
                (Number::Float(l0), Number::Float(r0)) => l0.eq(r0),
                (Number::Int64(l0), Number::Float(r0)) => *l0 as f64 == *r0,
                (Number::Float(l0), Number::Int64(r0)) => *l0 == *r0 as f64,
            }
        }
    }

    impl Number {
        pub fn floor_div(&self, rhs: &Number) -> Number {
            match (self, rhs) {
                (Number::Int64(l0), Number::Int64(r0)) => Number::Int64(l0 / r0),
                _ => todo!(),
            }
        }
        pub fn pow(&self, rhs: Number) -> Number {
            match (self, rhs) {
                (Number::Int64(l0), Number::Int64(r0)) => Number::Int64(l0.pow(r0 as u32)),
                _ => todo!(),
            }
        }
        pub fn abs(&self) -> Number {
            match self {
                Number::Int64(i) => Number::Int64(i.abs()),
                Number::Float(f) => Number::Float(f.abs()),
            }
        }
        pub fn test(&self) -> bool {
            match self {
                Number::Int64(i) => *i != 0,
                Number::Float(f) => *f != 0.0,
            }
        }
    }
    impl ToString for Number {
        fn to_string(&self) -> String {
            match self {
                Number::Int64(i) => i.to_string(),
                Number::Float(f) => f.to_string(),
            }
        }
    }

    macro_rules! impl_binop {
        ($t:tt, $name:ident) => {
            impl $t for Number {
                type Output = Number;

                fn $name(self, rhs: Self) -> Self::Output {
                    match (self, rhs) {
                        (Number::Int64(lhs), Number::Int64(rhs)) => Number::Int64(lhs.$name(rhs)),
                        (Number::Int64(lhs), Number::Float(rhs)) => {
                            Number::Float((lhs as f64).$name(rhs))
                        }
                        (Number::Float(lhs), Number::Int64(rhs)) => {
                            Number::Float(lhs.$name(rhs as f64))
                        }
                        (Number::Float(lhs), Number::Float(rhs)) => Number::Float(lhs.$name(rhs)),
                    }
                }
            }
        };
    }
    impl_binop!(Add, add);
    impl_binop!(Mul, mul);
    impl_binop!(Sub, sub);
    impl_binop!(Rem, rem);
    impl Div for Number {
        type Output = Number;

        fn div(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (Number::Int64(lhs), Number::Int64(rhs)) => Number::Float(lhs as f64 / rhs as f64),
                (Number::Int64(lhs), Number::Float(rhs)) => Number::Float(lhs as f64 / rhs),
                (Number::Float(lhs), Number::Int64(rhs)) => Number::Float(lhs / rhs as f64),
                (Number::Float(lhs), Number::Float(rhs)) => Number::Float(lhs / rhs),
            }
        }
    }

    impl From<i64> for Number {
        fn from(v: i64) -> Self {
            Number::Int64(v)
        }
    }

    impl From<f64> for Number {
        fn from(v: f64) -> Self {
            Number::Float(v)
        }
    }
}
mod typed_value {
    mod boolean {
        pub struct Bool(pub bool);
        impl Bool {
            pub fn test(&self) -> bool {
                self.0
            }

            pub fn __unary_not(&self) -> Self {
                Self(!self.0)
            }
        }

        impl Default for Bool {
            fn default() -> Self {
                Self(Default::default())
            }
        }

        impl From<bool> for Bool {
            fn from(v: bool) -> Self {
                Self(v)
            }
        }
    }
    pub use self::boolean::*;
    mod list {
        use std::rc::Rc;

        use crate::{
            cell::{UnsafeRefCell, UnsafeRefMut},
            number::Number,
        };

        use super::{IndexValue, TypedValue};

        pub struct TypedList<T>(pub Rc<UnsafeRefCell<Vec<Rc<UnsafeRefCell<T>>>>>);
        impl<T> From<Vec<T>> for TypedList<T> {
            fn from(v: Vec<T>) -> Self {
                let list = v.into_iter().map(|v| UnsafeRefCell::rc(v)).collect();
                Self(UnsafeRefCell::rc(list))
            }
        }

        impl<T> Default for TypedList<T> {
            fn default() -> Self {
                Self(UnsafeRefCell::rc(vec![]))
            }
        }

        impl<T> TypedList<T> {
            pub fn __len(&self) -> Number {
                todo!()
            }
            pub fn reverse(&self) {
                todo!()
            }
            pub fn __index_value<I: IndexValue>(&self, _: I) -> T {
                todo!()
            }
            pub fn append(&self, x: T) {
                todo!()
            }
            pub fn pop(&self) -> T {
                todo!()
            }
            pub fn __mul(&self, _: Number) -> Self {
                todo!()
            }
            pub fn __index_ref(&self, index: Number) -> UnsafeRefMut<T> {
                match index {
                    Number::Int64(i) => {
                        if i < 0 {
                            let i = self.0.borrow().len() as i64 + i;
                            self.0.borrow_mut()[i as usize].borrow_mut()
                        } else {
                            self.0.borrow_mut()[i as usize].borrow_mut()
                        }
                    }
                    _ => todo!(),
                }
            }
        }

        impl<T: TypedValue> TypedList<T> {
            pub fn __list(&self) -> TypedList<T> {
                let list = self
                    .0
                    .borrow()
                    .iter()
                    .map(|v| UnsafeRefCell::rc(v.borrow().__shallow_copy()))
                    .collect::<Vec<_>>();
                Self(UnsafeRefCell::rc(list))
            }
        }
        impl<T: TypedValue> TypedValue for TypedList<T> {
            fn __shallow_copy(&self) -> Self {
                Self(Rc::clone(&self.0))
            }
        }
    }
    pub use self::list::*;
    mod number {
        use crate::number::Number;

        use super::{Bool, IndexValue, TypedValue};

        impl Number {
            pub fn __min(&self, rhs: Self) -> Self {
                if self < &rhs {
                    *self
                } else {
                    rhs
                }
            }

            pub fn __sub(&self, rhs: Self) -> Self {
                *self - rhs
            }

            pub fn __add(&self, rhs: Self) -> Self {
                *self + rhs
            }
            pub fn __mul(&self, rhs: Self) -> Self {
                *self * rhs
            }

            pub fn __gt(&self, rhs: Self) -> Bool {
                Bool::from(*self > rhs)
            }
            pub fn __eq(&self, rhs: Self) -> Bool {
                Bool::from(*self == rhs)
            }
            pub fn __unary_sub(&self) -> Self {
                match self {
                    Number::Int64(i) => Number::Int64(-i),
                    Number::Float(f) => Number::Float(-f),
                }
            }
        }

        impl TypedValue for Number {
            fn __shallow_copy(&self) -> Self {
                *self
            }
        }

        impl Default for Number {
            fn default() -> Self {
                Number::Int64(0)
            }
        }
        impl IndexValue for Number {
            fn __as_number(&self) -> Number {
                *self
            }
        }
    }
    pub use self::number::*;
    mod string {
        use super::{TypedList, TypedValue};

        pub struct TypedString();

        impl TypedString {
            pub fn split(&self) -> TypedList<TypedString> {
                todo!()
            }
        }

        impl TypedValue for TypedString {
            fn __shallow_copy(&self) -> Self {
                todo!()
            }
        }

        impl Default for TypedString {
            fn default() -> Self {
                Self()
            }
        }
    }
    pub use self::string::*;
    mod traits {
        use crate::{cell::UnsafeRefMut, number::Number};

        pub trait TypedValue: Sized {
            fn __shallow_copy(&self) -> Self;
            fn assign(&mut self, value: Self) {
                *self = value;
            }
        }

        pub trait IndexValue {
            fn __as_number(&self) -> Number;
        }
    }
    pub use self::traits::*;
}
pub use number::Number;
pub use typed_builtin::*;
pub use typed_value::*;
fn main() {
    let mut __v0 = Default::default();
    let mut __v1 = Default::default();
    let mut __v12 = Default::default();
    let mut __v2 = Default::default();
    let mut __v3 = Default::default();
    let mut __v4 = Default::default();
    let mut __v49 = Default::default();
    let mut __v5 = Default::default();
    let mut __v50 = Default::default();
    let mut __v51 = Default::default();
    let mut __v52 = Default::default();
    let mut __v53 = Default::default();
    let mut __v54 = Default::default();
    let mut __v55 = Default::default();
    let mut __v6 = Default::default();
    let mut __v7 = Default::default();
    __v0 = Number::from(1000000000000000f64);
    __v1 = map_int(input().split().__shallow_copy());
    __v2 = __v1.__index_value(Number::from(0i64).__shallow_copy());
    __v3 = __v1.__index_value(Number::from(1i64).__shallow_copy());
    __v4 = list(map_int(input().split().__shallow_copy()).__shallow_copy());
    __v5 = __v2
        .__mul(Number::from(2i64).__shallow_copy())
        .__add(Number::from(2i64).__shallow_copy());
    __v6 = __v2.__mul(Number::from(2i64).__shallow_copy());
    __v7 = __v2
        .__mul(Number::from(2i64).__shallow_copy())
        .__add(Number::from(1i64).__shallow_copy());
    let __f0 = |mut __v8| {
        let mut __v10 = Default::default();
        let mut __v11 = Default::default();
        let mut __v9 = Default::default();
        __v9 = TypedList::from(vec![]);
        __v10 = list(typed_range!(__v8.__shallow_copy()).__shallow_copy());
        __v10.reverse();
        while (len(__v10.__shallow_copy()).__gt(Number::from(0i64).__shallow_copy())).test() {
            __v11 = __v10.pop();
            __v9.append(TypedList::from(vec![]).__shallow_copy());
        }
        return __v9.__shallow_copy();
        return Default::default();
    };
    __v12 = __f0(__v5.__shallow_copy());
    let __f1 = |mut __v13, mut __v14, mut __v15, mut __v16, mut __v17| {
        __v17.__index_value(__v13.__shallow_copy()).append(
            TypedList::from(vec![
                __v14.__shallow_copy(),
                __v15.__shallow_copy(),
                __v16.__shallow_copy(),
                len(__v17.__index_value(__v14.__shallow_copy()).__shallow_copy()).__shallow_copy(),
            ])
            .__shallow_copy(),
        );
        __v17.__index_value(__v14.__shallow_copy()).append(
            TypedList::from(vec![
                __v13.__shallow_copy(),
                Number::from(0i64).__shallow_copy(),
                __v16.__unary_sub().__shallow_copy(),
                len(__v17.__index_value(__v13.__shallow_copy()).__shallow_copy())
                    .__sub(Number::from(1i64).__shallow_copy())
                    .__shallow_copy(),
            ])
            .__shallow_copy(),
        );
        return Default::default();
    };
    let __f2 = |mut __v18, mut __v19, mut __v20, mut __v21| {
        let mut __v22 = Default::default();
        let mut __v23 = Default::default();
        let mut __v24 = Default::default();
        let mut __v25 = Default::default();
        let mut __v26 = Default::default();
        let mut __v27 = Default::default();
        let mut __v28 = Default::default();
        let mut __v29 = Default::default();
        let mut __v30 = Default::default();
        let mut __v31 = Default::default();
        let mut __v32 = Default::default();
        let mut __v33 = Default::default();
        let mut __v34 = Default::default();
        __v22 = TypedList::from(vec![__v19.__shallow_copy()]).__mul(__v21.__shallow_copy());
        __v22
            .__index_ref(__v18.__shallow_copy())
            .assign(Number::from(0i64).__shallow_copy());
        __v23 = TypedList::from(vec![Number::from(0i64).__shallow_copy()])
            .__mul(__v21.__shallow_copy());
        __v24 = TypedList::from(vec![Number::from(0i64).__shallow_copy()])
            .__mul(__v21.__shallow_copy());
        while (Bool::from(true)).test() {
            __v25 = Bool::from(false);
            __v26 = list(typed_range!(__v21.__shallow_copy()).__shallow_copy());
            __v26.reverse();
            while (len(__v26.__shallow_copy()).__gt(Number::from(0i64).__shallow_copy())).test() {
                __v27 = __v26.pop();
                if (__v22
                    .__index_value(__v27.__shallow_copy())
                    .__eq(__v19.__shallow_copy()))
                .test()
                {
                    continue;
                } else {
                };
                __v28 = list(
                    typed_range!(
                        len(__v20.__index_value(__v27.__shallow_copy()).__shallow_copy())
                            .__shallow_copy()
                    )
                    .__shallow_copy(),
                );
                __v28.reverse();
                while (len(__v28.__shallow_copy()).__gt(Number::from(0i64).__shallow_copy())).test()
                {
                    __v29 = __v28.pop();
                    __v30 = __v20
                        .__index_value(__v27.__shallow_copy())
                        .__index_value(__v29.__shallow_copy());
                    __v31 = __v30.__index_value(Number::from(0i64).__shallow_copy());
                    __v32 = __v30.__index_value(Number::from(1i64).__shallow_copy());
                    __v33 = __v30.__index_value(Number::from(2i64).__shallow_copy());
                    __v34 = __v30.__index_value(Number::from(3i64).__shallow_copy());
                    if (Bool::from(
                        __v32.__gt(Number::from(0i64).__shallow_copy()).test()
                            && __v22
                                .__index_value(__v31.__shallow_copy())
                                .__gt(
                                    __v22
                                        .__index_value(__v27.__shallow_copy())
                                        .__add(__v33.__shallow_copy())
                                        .__shallow_copy(),
                                )
                                .test(),
                    ))
                    .test()
                    {
                        __v22.__index_ref(__v31.__shallow_copy()).assign(
                            __v22
                                .__index_value(__v27.__shallow_copy())
                                .__add(__v33.__shallow_copy())
                                .__shallow_copy(),
                        );
                        __v25 = Bool::from(true);
                        __v23
                            .__index_ref(__v31.__shallow_copy())
                            .assign(__v27.__shallow_copy());
                        __v24
                            .__index_ref(__v31.__shallow_copy())
                            .assign(__v29.__shallow_copy());
                    } else {
                    }
                }
            }
            if (__v25.__unary_not()).test() {
                break;
            } else {
            }
        }
        return TypedList::from(vec![
            __v22.__shallow_copy(),
            __v23.__shallow_copy(),
            __v24.__shallow_copy(),
        ])
        .__shallow_copy();
        return Default::default();
    };
    let __f3 = |mut __v35, mut __v36, mut __v37, mut __v38, mut __v39, mut __v40| {
        let mut __v41 = Default::default();
        let mut __v42 = Default::default();
        let mut __v43 = Default::default();
        let mut __v44 = Default::default();
        let mut __v45 = Default::default();
        let mut __v46 = Default::default();
        let mut __v47 = Default::default();
        let mut __v48 = Default::default();
        __v41 = Number::from(0i64);
        while (__v37.__gt(Number::from(0i64).__shallow_copy())).test() {
            __v42 = __f2(
                __v35.__shallow_copy(),
                __v38.__shallow_copy(),
                __v39.__shallow_copy(),
                __v40.__shallow_copy(),
            );
            __v43 = __v42.__index_value(Number::from(0i64).__shallow_copy());
            __v44 = __v42.__index_value(Number::from(1i64).__shallow_copy());
            __v45 = __v42.__index_value(Number::from(2i64).__shallow_copy());
            if (__v43
                .__index_value(__v36.__shallow_copy())
                .__eq(__v38.__shallow_copy()))
            .test()
            {
                return __v38.__shallow_copy();
            } else {
            };
            __v46 = __v37;
            __v47 = __v36;
            while (__v47.__ne(__v35.__shallow_copy())).test() {
                __v46 = typed_min!(
                    __v46.__shallow_copy(),
                    __v39
                        .__index_value(__v44.__index_value(__v47.__shallow_copy()).__shallow_copy())
                        .__index_value(__v45.__index_value(__v47.__shallow_copy()).__shallow_copy())
                        .__index_value(Number::from(1i64).__shallow_copy())
                        .__shallow_copy()
                );
                __v47 = __v44.__index_value(__v47.__shallow_copy());
            }
            __v41 = __v41.__add(
                __v46
                    .__mul(__v43.__index_value(__v36.__shallow_copy()).__shallow_copy())
                    .__shallow_copy(),
            );
            __v37 = __v37.__sub(__v46.__shallow_copy());
            __v47 = __v36;
            while (__v47.__ne(__v35.__shallow_copy())).test() {
                __v39
                    .__index_ref(__v44.__index_value(__v47.__shallow_copy()).__shallow_copy())
                    .__index_ref(__v45.__index_value(__v47.__shallow_copy()).__shallow_copy())
                    .__index_ref(Number::from(1i64).__shallow_copy())
                    .assign(
                        __v39
                            .__index_value(
                                __v44.__index_value(__v47.__shallow_copy()).__shallow_copy(),
                            )
                            .__index_value(
                                __v45.__index_value(__v47.__shallow_copy()).__shallow_copy(),
                            )
                            .__index_value(Number::from(1i64).__shallow_copy())
                            .__sub(__v46.__shallow_copy())
                            .__shallow_copy(),
                    );
                __v48 = __v39
                    .__index_value(__v44.__index_value(__v47.__shallow_copy()).__shallow_copy())
                    .__index_value(__v45.__index_value(__v47.__shallow_copy()).__shallow_copy())
                    .__index_value(Number::from(3i64).__shallow_copy());
                __v39
                    .__index_ref(__v47.__shallow_copy())
                    .__index_ref(__v48.__shallow_copy())
                    .__index_ref(Number::from(1i64).__shallow_copy())
                    .assign(
                        __v39
                            .__index_value(__v47.__shallow_copy())
                            .__index_value(__v48.__shallow_copy())
                            .__index_value(Number::from(1i64).__shallow_copy())
                            .__add(__v46.__shallow_copy())
                            .__shallow_copy(),
                    );
                __v47 = __v44.__index_value(__v47.__shallow_copy());
            }
        }
        return __v41.__shallow_copy();
        return Default::default();
    };
    __v49 = list(typed_range!(__v2.__shallow_copy()).__shallow_copy());
    __v49.reverse();
    while (len(__v49.__shallow_copy()).__gt(Number::from(0i64).__shallow_copy())).test() {
        __v50 = __v49.pop();
        __f1(
            __v6.__shallow_copy(),
            __v50.__shallow_copy(),
            Number::from(1i64).__shallow_copy(),
            Number::from(0i64).__shallow_copy(),
            __v12.__shallow_copy(),
        );
        __f1(
            __v50.__shallow_copy(),
            __v7.__shallow_copy(),
            Number::from(1i64).__shallow_copy(),
            __v3.__shallow_copy(),
            __v12.__shallow_copy(),
        );
    }
    __v51 = list(typed_range!(__v2.__shallow_copy()).__shallow_copy());
    __v51.reverse();
    while (len(__v51.__shallow_copy()).__gt(Number::from(0i64).__shallow_copy())).test() {
        __v50 = __v51.pop();
        __v52 = list(
            typed_range!(
                __v50
                    .__add(Number::from(1i64).__shallow_copy())
                    .__shallow_copy(),
                __v2.__shallow_copy()
            )
            .__shallow_copy(),
        );
        __v52.reverse();
        while (len(__v52.__shallow_copy()).__gt(Number::from(0i64).__shallow_copy())).test() {
            __v53 = __v52.pop();
            __f1(
                __v50.__shallow_copy(),
                __v2.__add(__v53.__shallow_copy()).__shallow_copy(),
                Number::from(1i64).__shallow_copy(),
                abs(__v4
                    .__index_value(__v50.__shallow_copy())
                    .__sub(__v4.__index_value(__v53.__shallow_copy()).__shallow_copy())
                    .__shallow_copy())
                .__shallow_copy(),
                __v12.__shallow_copy(),
            );
        }
    }
    __v54 = list(typed_range!(__v2.__shallow_copy()).__shallow_copy());
    __v54.reverse();
    while (len(__v54.__shallow_copy()).__gt(Number::from(0i64).__shallow_copy())).test() {
        __v53 = __v54.pop();
        __f1(
            __v2.__add(__v53.__shallow_copy()).__shallow_copy(),
            __v7.__shallow_copy(),
            Number::from(1i64).__shallow_copy(),
            Number::from(0i64).__shallow_copy(),
            __v12.__shallow_copy(),
        );
    }
    __v55 = __f3(
        __v6.__shallow_copy(),
        __v7.__shallow_copy(),
        __v2.__shallow_copy(),
        __v0.__shallow_copy(),
        __v12.__shallow_copy(),
        __v5.__shallow_copy(),
    );
    typed_print_values!(__v55.__shallow_copy());
}
