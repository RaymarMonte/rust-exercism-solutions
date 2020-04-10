use std::collections::VecDeque;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets_only = String::from(string);
    brackets_only.retain(|c| r#"{}[]()"#.contains(c));
    let mut bracket_stack = VecDeque::new();
    for c in brackets_only.as_str().chars() {
        match c {
            '{' | '[' | '(' => bracket_stack.push_front(c),
            '}' | ']' | ')' => {
                match bracket_stack.pop_front() {
                    Some(next_open_bracket) => {
                        match next_open_bracket {
                            '{' => if c != '}' {return false;},
                            '[' => if c != ']' {return false;},
                            '(' => if c != ')' {return false;},
                            _ => return false
                        };
                    },
                    None => return false
                };
            },
            _ => return false
        };
    }
    if bracket_stack.is_empty() {true} else {false}
}