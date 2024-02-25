use std::path::Path;
use async_trait::async_trait;

use auto_delegate::{delegate, Delegate};

#[delegate]
trait ReadFile {
    async fn read_file(&self, path: impl AsRef<Path>) -> String;
}



#[delegate]
#[async_trait]
trait ReadFileBufLen {
    async fn read_file_buf_len(&self, path: impl AsRef<Path> + Send) -> usize;
}

#[derive(Default)]
struct Io;

impl ReadFile for Io{
    async fn read_file(&self, path: impl AsRef<Path>) -> String {
        tokio::fs::read_to_string(path).await.unwrap()
    }
}


#[async_trait]
impl ReadFileBufLen for Io {
    async fn read_file_buf_len(&self, path: impl AsRef<Path> + Send) -> usize {
        tokio::fs::read_to_string(path).await.unwrap().len()
    }
}

#[derive(Default, Delegate)]
struct Parent {
    #[to(ReadFile, ReadFileBufLen)]
    child: Io,
}


#[tokio::main]
async fn main() {
    const PATH: &str = "examples/structs/async_trait.txt";
    let parent = Parent::default();
    
    assert_eq!(parent.read_file(PATH).await, "HELLO");
    assert_eq!(parent.read_file_buf_len(PATH).await, 5);
}