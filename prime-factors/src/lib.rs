pub fn factors(n: u64) -> Vec<u64> {
    let mut dividend = n;
    let mut all_factors = Vec::new();
    while dividend != 1 {
        for divisor in 2..=dividend {
            if dividend % divisor == 0 {
                all_factors.push(divisor);
                dividend = dividend / divisor;
                break;
            }
        }
    }
    all_factors
}