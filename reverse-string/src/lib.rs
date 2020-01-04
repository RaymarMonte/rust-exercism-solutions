//extern crate unicode_segmentation; No longer needed in the 2018 edition of rust

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect::<String>()
}
