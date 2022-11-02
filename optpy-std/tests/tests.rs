use optpy_std::value::Value;

#[test]
fn test_split() {
    let value = Value::from("abc efg\n");
    let list = value.split();
    assert_eq!(
        list,
        Value::List(vec![Value::from("abc"), Value::from("efg")])
    );
}
