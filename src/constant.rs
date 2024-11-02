use colored::{Color, ColoredString, Colorize};
use std::io::Write;
// constants

/*
▀ 	▁ 	▂ 	▃ 	▄ 	▅ 	▆ 	▇ 	█ 	▉ 	▊ 	▋ 	▌ 	▍ 	▎ 	▏

▐ 	░ 	▒ 	▓ 	▔ 	▕ 	▖ 	▗ 	▘ 	▙ 	▚ 	▛ █ 	▜ 	▝ 	▞ 	▟
*/
pub const VERTICAL_LINE: char = '│';
pub const HORIZONTAL_LINE_HIGH: char = '─';
pub const HORIZONTAL_LINE_LOW: char = '─';
pub const LEFT_UPPER_SHOULDER: char = '┌';
pub const RIGHT_UPPER_SHOULDER: char = '┐';
pub const LEFT_LOWER_SHOULDER: char = '└';
pub const RIGHT_LOWER_SHOULDER: char = '┘';
pub const WHITESPACE: char = ' ';

pub const CHRONICLE_RESOURCE_PATH: &str =
    "/home/fizbin/lair/proj/rust/chronicle/asset/chronicle.txt";

pub const MENU_OPTION_NEW_ENTRY: &str = "[] NEW ENTRY";
pub const MENU_OPTION_BROWSE_ENTRIES: &str = "[] BROWSE ENTRIES";
pub const MENU_OPTION_QUIT: &str = "[] QUIT";

pub fn selected(s: &str) -> ColoredString {
    return s.to_owned().white().bold().on_bright_black();
}
/*
pub fn test_square() {
    let mut stdout = std::io::stdout();
    let mut top_str = String::new();
    top_str.push_str(LEFT_UPPER_SHOULDER);
    for _ in 0..=8 {
        top_str.push_str(HORIZONTAL_LINE_HIGH);
    }
    top_str.push_str(RIGHT_UPPER_SHOULDER);
    top_str.push('\n');
    print!("{}", top_str);
    stdout.flush().unwrap();

    let mut body: Vec<String> = Vec::new();
    for _ in 0..=8 {
        let mut str: String = String::new();
        str.push_str(VERTICAL_LINE);
        for _ in 0..=8 {
            str.push_str(WHITESPACE);
        }
        str.push_str(VERTICAL_LINE);
        body.push(str);
    }
    for str in body {
        println!("{}", str);
        stdout.flush().unwrap();
    }

    let mut bottom_str = String::new();
    bottom_str.push_str(LEFT_LOWER_SHOULDER);
    for _ in 0..=8 {
        bottom_str.push_str(HORIZONTAL_LINE_LOW);
    }
    bottom_str.push_str(RIGHT_LOWER_SHOULDER);
    println!("{}", bottom_str);
    stdout.flush().unwrap();
}
*/
