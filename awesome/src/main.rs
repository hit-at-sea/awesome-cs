use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Awesome!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

// ------------
// | Awesome! |
// ------------
//               \
//                \
//                   _~^~^~_
//               \) /  o o  \ (/
//                 '_   -   _'
//                 / '-----' \
