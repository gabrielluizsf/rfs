use rfs::file_system::{FileSystem, LocalFileSystem};

#[test]
fn test_file_system() {
    let file = "test.txt";
    let file_system = LocalFileSystem;
    file_system.create_file(file, bytes("abc")).unwrap();
    file_system.write_file(file, bytes("xyz")).unwrap();
    file_system
    .write_file(file, bytes("\nHello, world!"))
    .unwrap();
    let content = file_system.read_file(file).unwrap();
    assert_eq!(content, "abcxyz\nHello, world!");
    file_system.delete_file(file).unwrap();
    let test_dir = "test_dir";
    file_system.create_dir(test_dir).unwrap();
    file_system.create_file(&(test_dir.to_owned() + "/" + file), bytes("Hello, Rust")).unwrap();
    file_system.delete_dir(test_dir).unwrap();
    assert!(!file_system.read_file(&(test_dir.to_owned()+ "/"+ file)).is_ok());
}
fn bytes(content: &str) -> &[u8] {
    return content.as_bytes();
}
