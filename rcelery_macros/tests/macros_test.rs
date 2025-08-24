use rcelery_macros::Describe;

#[derive(Describe)]
struct TestStruct {
    field1: i32,
    field2: String,
}

#[test]
pub fn test_describe() {
    TestStruct::describe();
}
