pub fn check(candidate: &str) -> bool {
    let trim_lowercase_candidate = candidate.replace(&['-', ' '][..], "").to_lowercase();
    trim_lowercase_candidate.chars().all(|c| trim_lowercase_candidate.matches(c).collect::<Vec<_>>().len() == 1)
}
