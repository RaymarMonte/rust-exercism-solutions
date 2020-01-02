pub fn nth(n: u32) -> u32 {
    let mut offseted_n = n + 1;
    let mut current_num = 1;
    while offseted_n > 0 {
        current_num += 1;
        let mut is_prime = true;
        let mut test_num = 2;
        while test_num < current_num {
            if current_num % test_num == 0 {
                is_prime = false;
                break;
            }
            test_num += 1;
        }
        if is_prime {
            offseted_n -= 1;
        }
    }
    current_num
}