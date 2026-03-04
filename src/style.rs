// Storing formatting preferences to be shared for multiple locations
use std::iter;

static PADDING_STRING : &str = "-";


pub fn padding(depth: u8) -> String {
    PADDING_STRING.chars()
        .cycle()
        .take(depth as usize)
        .collect()
}