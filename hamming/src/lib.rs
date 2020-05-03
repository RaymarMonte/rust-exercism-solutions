/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None
    }
    let mut s1_chars = s1.chars();
    let mut s2_chars = s2.chars();
    let mut distance = 0;
    for _ in 0..s1.len() {
        if s1_chars.next() != s2_chars.next() {
            distance += 1;
        }
    }
    Some(distance)
}
