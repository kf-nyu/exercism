use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    let chunk_size = input.len().div_ceil(worker_count);
    if chunk_size == 0 {
        return counts;
    };
    std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for chunk in input.chunks(chunk_size) {
            let handle = scope.spawn(move || {
                let mut counts_sub = HashMap::new();
                for &list in chunk {
                    for c in list.chars().filter(|c| c.is_alphabetic()) {
                        *counts_sub.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
                    }
                }
                counts_sub
            });
            handles.push(handle);
        }
        for handle in handles {
            let worker_counts = handle.join().unwrap();
            for (c, count) in worker_counts {
                *counts.entry(c).or_insert(0) += count;
            }
        }
    });

    counts
}
