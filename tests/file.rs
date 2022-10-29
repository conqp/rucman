use rucman::File;

#[test]
fn test_file() {
    let file = File::new("test", 42, 123);
    assert_eq!(file.name(), &String::from("test"));
    assert_eq!(file.size(), 42);
    assert_eq!(file.mode(), 123);
}
