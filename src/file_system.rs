use std::fs;
use std::io::{self, Write, Read};

#[warn(dead_code)]
/// A trait defining a generic file system interface for file and directory operations.
///
/// This trait provides methods for basic file system interactions such as creating,
/// writing, reading, and deleting files and directories.
///
/// # Methods
///
/// - `create_file`: Creates a new file with the given content
/// - `create_dir`: Creates a directory (or nested directories)
/// - `write_file`: Appends content to an existing file
/// - `read_file`: Reads the entire content of a file as a string
/// - `delete_file`: Removes a specific file
/// - `delete_dir`: Removes a directory and its contents
///
/// # Errors
///
/// All methods return `io::Result<()>`, which allows handling of potential I/O errors
/// during file system operations.
pub trait FileSystem {
    fn create_file(&self, file: &str, content: &[u8]) -> io::Result<()>;
    fn create_dir(&self, path: &str) -> io::Result<()>;
    fn write_file(&self, file: &str, content: &[u8]) -> io::Result<()>;
    fn read_file(&self, file: &str) -> io::Result<String>;
    fn delete_file(&self, file: &str) -> io::Result<()>;
    fn delete_dir(&self, path: &str) -> io::Result<()>;
}

/// A local file system implementation that provides concrete methods for file system operations.
///
/// This struct implements the `FileSystem` trait, enabling standard file and directory
/// manipulation methods using the local machine's file system.
///
/// # Examples
///
/// 
///     let fs = LocalFileSystem;
///     fs.create_file("example.txt", b"Hello, world!")?;
///     fs.write_file("example.txt", b"Updated content")?;
///     let content = fs.read_file("example.txt")?;
///     fs.delete_file("example.txt")?;
///     fs.create_dir("example_dir")?;
///     fs.delete_dir("example_dir")?;
pub struct LocalFileSystem;

impl FileSystem for LocalFileSystem {
    fn create_file(&self, file: &str, content: &[u8]) -> io::Result<()> {
        let mut f = fs::File::create(file)?;
        f.write_all(content)
    }

    fn create_dir(&self, path: &str) -> io::Result<()> {
        fs::create_dir_all(path)
    }

    fn write_file(&self, file: &str, content: &[u8]) -> io::Result<()> {
        let mut f = fs::OpenOptions::new().write(true).append(true).open(file)?;
        f.write_all(content)
    }

    fn read_file(&self, file: &str) -> io::Result<String> {
        let mut f = fs::File::open(file)?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        Ok(content)
    }

    fn delete_file(&self, file: &str) -> io::Result<()> {
        fs::remove_file(file)
    }
    fn delete_dir(&self, path: &str) -> io::Result<()> {
        fs::remove_dir_all(path)
    }
}
