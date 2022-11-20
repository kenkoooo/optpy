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
macro_rules! print {
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
