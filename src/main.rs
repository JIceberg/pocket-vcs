mod file;

use file::contents::Line;
use file::history::Staged;
use file::header::Readable;

fn main() {
    let mut line = Line::new();

    line.stage("a".to_string());    // 1
    line.stage("b".to_string());    // 2
    line.stage("c".to_string());    // 3
    line.stage("d".to_string());    // 4
    
    line.revert(2);                 // 5
    line.stage("e".to_string());    // 6

    line.revert(3);                 // 7
    line.stage(line.read());

    line.print_history();
}