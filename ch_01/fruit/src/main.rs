fn main() {
    let fruit = vec!["chicken", "steak", "egg"];

    let buffer_overflow = fruit[4];
    assert_eq!(buffer_overflow, "fish")
}