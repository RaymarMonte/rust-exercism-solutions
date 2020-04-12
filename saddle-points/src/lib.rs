use array_tool::vec::Intersect;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    // Get the highest value positions per row
    let mut highest_per_row: Vec<(usize, usize)> = Vec::new();
    for (i, row) in input.iter().enumerate() {
        let mut highest_val: u64 = 0;
        let mut highest_pos: Vec<(usize, usize)> = Vec::new();
        for (j, cell) in row.iter().cloned().enumerate() {
            if cell > highest_val {
                highest_val = cell;
                highest_pos.clear();
                highest_pos.push((i, j));
            } else if cell == highest_val {
                highest_pos.push((i, j));
            }
        }
        highest_per_row.append(&mut highest_pos);
    }
    // Get the lowest value positions per column
    let mut lowest_per_col: Vec<(usize, usize)> = Vec::new();
    let mut col_count = 0;
    for row in input.iter().take(1) {
        col_count = row.len();
    }
    for j in 0..col_count {
        let mut lowest_val: u64 = 10;
        let mut lowest_pos: Vec<(usize, usize)> = Vec::new();
        for i in 0..input.len() {
            if input[i][j] < lowest_val {
                lowest_val = input[i][j];
                lowest_pos.clear();
                lowest_pos.push((i, j));
            } else if input[i][j] == lowest_val {
                lowest_pos.push((i, j));
            }
        }
        lowest_per_col.append(&mut lowest_pos);
    }
    // Get the intersection of the two vec's
    highest_per_row.intersect(lowest_per_col)
}
