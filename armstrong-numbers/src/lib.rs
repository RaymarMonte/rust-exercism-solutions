pub fn is_armstrong_number(num: u32) -> bool {
    let num_as_string = num.to_string();
    let digits_count = num_as_string.chars().count() as u32;
    let armstrong_num: u32 = num_as_string.chars().map(|c| c.to_digit(10).unwrap().pow(digits_count)).sum();
    armstrong_num == num
}
