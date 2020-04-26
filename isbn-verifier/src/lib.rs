/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let normalized_isbn = &isbn.trim().replace('-', "");
    if normalized_isbn.len() != 10 {
        return false;
    }
    let mut validity = 0;
    // i: index, c: character; enumerate() returns tuple consisting of the
    // index and the actual data
    for (i, c) in normalized_isbn.chars().enumerate() {
        // to_digit() requires the base number. 10 = decimal
        let digit = match c.to_digit(10) {
            Some(x) => x,   // If c is indeed a digit
            None => {       // If not
                if i == 9 && c == 'X' {
                    10      // No semicolon means return the value
                } else {
                    return false;
                }
            }
        };
        validity += digit * (10 - (i as u32));
    }
    (validity % 11) == 0
}
