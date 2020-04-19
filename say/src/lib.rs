pub fn encode(mut n: u64) -> String {
    let mut segments = Vec::new();
    let mut segment;
    let mut scale = 0;
    loop {
        let segment_value = n % 1_000;
        if segment_value > 0 {
            segment = encode_hundreds(segment_value);
            let scale_name = match scale {
                1 => " thousand",
                2 => " million",
                3 => " billion",
                4 => " trillion",
                5 => " quadrillion",
                6 => " quintillion",
                _ => ""
            };
            segment.push_str(scale_name);
            segments.push(segment);
        }
        n /= 1_000;
        scale += 1;
        if n == 0 { break; }
    }
    segments.reverse();
    segments.join(" ")
}

fn encode_hundreds(n: u64) -> String {
    let hundreds_value = n / 100;
    let mut hundreds_word_string;
    let mut hundreds_word = "";
    if hundreds_value > 0 {
        hundreds_word_string = encode_ones(hundreds_value);
        hundreds_word_string.push_str(" hundred ");
        hundreds_word = &hundreds_word_string;
    }
    let remaining_value = n % 100;
    let remaining_word_string;
    let mut remaining_word = "";
    if remaining_value > 0 || hundreds_value == 0 {
        remaining_word_string = encode_tens(remaining_value);
        remaining_word = &remaining_word_string;
    }
    format!("{}{}", hundreds_word, remaining_word).trim().to_string()
}

fn encode_tens(n: u64) -> String {
    let tens_value = n / 10;
    let ones_value = n % 10;
    let mut tens_word =  "";
    if tens_value > 0 {
        tens_word = match tens_value {
            1 => {
                match ones_value {
                    0 => "ten",
                    1 => "eleven",
                    2 => "twelve",
                    3 => "thirteen",
                    4 => "fourteen",
                    5 => "fifteen",
                    6 => "sixteen",
                    7 => "seventeen",
                    8 => "eighteen",
                    9 => "ninetheen",
                    _ => {
                        panic!("Error with ones_value variable: {}", ones_value);
                    }
                }
            },
            2 => "twenty",
            3 => "thirty",
            4 => "forty",
            5 => "fifty",
            6 => "sixty",
            7 => "seventy",
            8 => "eighty",
            9 => "ninety",
            _ => {
                panic!("encode_tens called for a number greater than 99");
            }
        };
    }
    let ones_word_string;
    let mut ones_word = "";
    if tens_value != 1 && (ones_value != 0 || tens_value == 0) {
        ones_word_string = encode_ones(ones_value);
        ones_word = &ones_word_string;
    }
    format!("{}-{}", tens_word, ones_word).trim_matches('-').to_string()
}

fn encode_ones(n: u64) -> String {
    let word = match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => {
            panic!("encode_ones called for a number greater than 9");
        }
    };
    word.to_string()
}