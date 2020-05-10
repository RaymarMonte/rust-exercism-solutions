/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let normalized_isbn = &isbn.trim().replace('-', "");
    if normalized_isbn.len() != 10 {
        return false;
    }
    let mut validity = 0;
    for (i, c) in normalized_isbn.chars().enumerate() {
        let digit = match c.to_digit(10) {
            Some(x) => x,
            None => {
                if i == 9 && c == 'X' {
                    10
                } else {
                    return false;
                }
            }
        };
        validity += digit * (10 - (i as u32));
    }
    (validity % 11) == 0
}
