pub fn build_proverb(list: &[&str]) -> String {
    let mut phrases_list = Vec::new();
    for index in 1..list.len() {
        phrases_list.push(format!("For want of a {} the {} was lost.", list[index-1], list[index]));
    }
    if !list.is_empty() {
        phrases_list.push(format!("And all for the want of a {}.", list[0]));
    }

    phrases_list.join("\n")
}
