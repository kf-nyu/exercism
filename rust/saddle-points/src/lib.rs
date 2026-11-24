pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter().max().map(|&max_val| {
                row.iter()
                    .enumerate()
                    .filter(move |&(_, &val)| val == max_val)
                    .map(move |(col_idx, &val)| ((row_idx, col_idx), val))
            })
        })
        .flatten()
        .filter(|&((_, col_idx), max_val)| {
            input.iter().all(|other_row| match other_row.get(col_idx) {
                Some(&col_val) => col_val >= max_val,
                None => true,
            })
        })
        .map(|((row_idx, col_idx), _)| (row_idx, col_idx))
        .collect()
}
