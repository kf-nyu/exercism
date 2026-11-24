pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        fn next_row(row_line: &[u32]) -> Vec<u32> {
            let mut line: Vec<u32> = row_line.to_owned();
            line.insert(0, 0);
            line.push(0);
            line.windows(2)
                .map(|w| w.iter().sum())
                .collect::<Vec<u32>>()
        }

        match &self.0 {
            0 => vec![],
            1 => vec![vec![1]],
            _ => {
                let prev = PascalsTriangle(self.0 - 1);
                let mut curr = prev.rows();
                let new_row = next_row(curr.last().unwrap());
                curr.push(new_row);
                curr
            }
        }
    }
}
