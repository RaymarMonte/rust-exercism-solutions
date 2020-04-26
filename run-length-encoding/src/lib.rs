pub fn encode(source: &str) -> String {
    let mut runs = Vec::new();
    let mut mutable_source = source;
    while !mutable_source.is_empty() {
        let first_char = mutable_source.chars().next().unwrap();
        match mutable_source.find(|c: char| c != first_char) {
            Some(mid) => {
                let (left, right) = mutable_source.split_at(mid);
                runs.push(left);
                mutable_source = right;
            }
            None => {
                runs.push(mutable_source);
                mutable_source = "";
            }
        }
    }
    let mut encoded = String::new();
    for run in runs.iter() {
        let run_len = run.len();
        if run_len > 1 {
            encoded.push_str(&run_len.to_string());
        }
        encoded.push(run.chars().next().unwrap());
    }
    encoded
}

pub fn decode(source: &str) -> String {
    let mut multiplier_string = String::new();
    let mut decoded = String::new();
    for char_from_source in source.chars() {
        if char_from_source.is_digit(10) {
            multiplier_string.push(char_from_source);
        } else {
            let mut multiplier = 1;
            if !multiplier_string.is_empty() {
                multiplier = multiplier_string.parse::<i32>().unwrap();
                multiplier_string.clear();
            }
            for _ in 0..multiplier {
                decoded.push(char_from_source);
            }
        }
    }
    decoded
}
