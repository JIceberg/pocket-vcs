mod file;

use file::contents::{Readable, Line};
use file::history::Staged;

fn main() {
    let mut line = Line::new();

    line.stage("Hello, world!".to_string());
    println!("{:?}", line.read());

    line.stage("Rust is a great language.".to_string());
    line.stage("It is much better than C.".to_string());
    line.stage("Goodbye.".to_string());
    println!("{:?}", line.read());

    let saved = line.revert(4);
    println!("{:?}", line.read());
}