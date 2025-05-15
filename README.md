# 📁 RFS - Rust File System

**RFS** is a simple and extensible Rust library that abstracts basic file system operations like creating, reading, writing, and deleting files and directories.

---

## ✨ Features

- Create and write to files
- Read files as `String`
- Create directories (including nested ones)
- Delete files and directories
- Trait-based interface: easy to test and extend

---

## 🔧 Usage

```rust
use rfs_br::file_system::{FileSystem, LocalFileSystem};

fn main() -> std::io::Result<()> {
    let fs = LocalFileSystem;

    fs.create_file("example.txt", b"Hello, world!")?;
    fs.write_file("example.txt", b"\nAppended content")?;
    
    let content = fs.read_file("example.txt")?;
    println!("{}", content);

    fs.delete_file("example.txt")?;
    Ok(())
}
```