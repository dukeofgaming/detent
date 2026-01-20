use detent::hello;

#[test]
fn test_hello() {
    assert_eq!(hello(), "Hello, world!");
}
