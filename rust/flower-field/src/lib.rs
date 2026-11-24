use ndarray::Array2;
use std::str;

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let rows = garden.len();
    if rows == 0 {
        return vec![];
    }
    let cols = garden[0].len();

    let mx_flat: Vec<char> = garden.iter().flat_map(|s| s.chars()).collect();

    let mx = Array2::from_shape_vec((rows, cols), mx_flat).unwrap();

    let mut solution = Vec::with_capacity(rows);

    for r in 0..rows {
        let mut row = String::with_capacity(cols);
        for c in 0..cols {
            if mx[[r, c]] == '*' {
                row.push('*');
            } else {
                let count = b'0' + (r.saturating_sub(1).. (r + 2).min(rows))
                    .flat_map (|rdr| (c.saturating_sub(1).. (c + 2).min(cols))
                         .map(move |cdc| [rdr, cdc])
                    )
                        .filter(|&p| mx[p] == '*')
                        .count() as u8;
                row.push(if count == b'0' { ' ' } else { char::from(count) });
            }

        }
        solution.push(row);
    }

    solution

}
