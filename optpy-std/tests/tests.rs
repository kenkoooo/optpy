use optpy_std::{print, sorted, value::Value};

#[test]
fn test_split() {
    let value = Value::from("abc efg\n");
    let list = value.split();
    assert_eq!(
        list,
        Value::from(vec![Value::from("abc"), Value::from("efg")])
    );
}

#[test]
fn test_list_assign() {
    let x = Value::from(vec![Value::from(1)]);
    x.index(&Value::from(0)).assign(&Value::from(2));
    assert_eq!(
        x.index(&Value::from(0)).__primitive(),
        Value::from(2).__primitive()
    );
}

#[test]
fn test_add() {
    let x = Value::from(vec![Value::from(1)]);
    let y = x.index(&Value::from(0)).__add(&Value::from(1));
    assert_eq!(y, Value::from(2));
}

#[test]
fn test_sorted() {
    let mut x = Value::none();
    x.assign(&Value::from(vec![Value::from(2), Value::from(1)]));
    sorted(&x);

    assert_eq!(
        x.index(&Value::from(0)).__primitive(),
        Value::from(1).__primitive()
    );
    assert_eq!(
        x.index(&Value::from(1)).__primitive(),
        Value::from(2).__primitive()
    );
}

#[test]
fn test_assign() {
    let mut __v0 = Value::none();
    __v0.assign(&Value::from(vec![Value::from(0i64)]));
    __v0.index(&Value::from(0i64))
        .assign(&__v0.index(&Value::from(0i64)));
    print!(&__v0.index(&Value::from(0i64)));
}
