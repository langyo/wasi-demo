use std::fs::read;
use std::fs::write;

fn main() {
    write("/test.txt", "LANGYO").unwrap();

    let content = read("/test.txt").unwrap();
    println!("test.txt: {}", String::from_utf8(content).unwrap());
}
