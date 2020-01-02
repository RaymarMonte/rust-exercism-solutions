pub fn verse(n: u32) -> String {
    match n {
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\n\
                Take one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\n\
        Take it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        0 => "No more bottles of beer on the wall, no more bottles of beer.\n\
        Go to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        _ => format!("{curr_n} bottles of beer on the wall, {curr_n} bottles of beer.\n\
                Take one down and pass it around, {next_n} bottles of beer on the wall.\n",
            curr_n=n, next_n=n-1)

    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = String::new();
    for current in (end..=start).rev() {
        if !song.is_empty() {
            song.push('\n');
        }
        song.push_str(&verse(current));
    }
    song
}
