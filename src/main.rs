mod file;

use file::contents::Line;
use file::history::Staged;
use file::header::Readable;

fn main() {
    let mut line = Line::new();

    line.stage(&['a' as u8]);   // 1
    line.stage(&['b' as u8]);   // 2
    line.stage(&['c' as u8]);   // 3
    line.stage(&['d' as u8]);   // 4
    
    line.revert(2);             // 5
    line.stage(&['e' as u8]);   // 6

    line.revert(3);
    line.reset();

    line.stage(&['f' as u8]);   // 7

    line.print_history();
}