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
