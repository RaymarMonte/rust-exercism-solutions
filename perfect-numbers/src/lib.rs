#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None
    }
    let mut factors = Vec::new();
    for i in 1..num {
        if num % i == 0 {
            factors.push(i);
        }
    }
    let aliquot_sum: u64 = factors.iter().sum::<u64>();
    if aliquot_sum == num {
        return Some(Classification::Perfect);
    } else if aliquot_sum > num {
        return Some(Classification::Abundant);
    } else if aliquot_sum < num {
        return Some(Classification::Deficient);
    }
    None
}
