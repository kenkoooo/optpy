use optpy_std::{sorted, value::Value};

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
    x.index(Value::from(0)).assign(Value::from(2));
    assert_eq!(x.index(Value::from(0)), Value::from(2));
}

#[test]
fn test_add() {
    let x = Value::from(vec![Value::from(1)]);
    let y = x.index(Value::from(0)).__add(Value::from(1));
    assert_eq!(y, Value::from(2));
}

#[test]
fn test_sorted() {
    let mut x = Value::none();
    x.assign(Value::from(vec![Value::from(2), Value::from(1)]));
    sorted(x.shallow_copy());

    assert_eq!(x.index(Value::from(0)), Value::from(1));
    assert_eq!(x.index(Value::from(1)), Value::from(2));
}
