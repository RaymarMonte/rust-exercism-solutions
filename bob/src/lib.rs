use regex::Regex;

pub fn reply(message: &str) -> &str {
    match message {
        x if Regex::new(r"^\s*$").unwrap().is_match(x)
            => "Fine. Be that way!",                // Whitespace
        x if Regex::new(r"^[^a-z]*[A-Z][^a-z]*\?\s*$").unwrap().is_match(x)
            => "Calm down, I know what I'm doing!", // All Caps Question
        x if Regex::new(r"^[^a-z]*[A-Z][^a-z]*$").unwrap().is_match(x)
            => "Whoa, chill out!",                  // All Caps
        x if Regex::new(r"^.*\?\s*$").unwrap().is_match(x)
            => "Sure.",                             // Normal Question
        _ => "Whatever.",                           // Random
    }
}
