extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut reversed = String::new();
    for g in input.graphemes(true).rev() {
        reversed.push_str(g);
    }

    reversed
}
