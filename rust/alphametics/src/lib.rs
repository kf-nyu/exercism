use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    fn cal(digit_map: &HashMap<char, u8>, element: &[&str]) -> u64 {
        element
            .iter()
            .map(|word| {
                word.chars().fold(0u64, |acc, c| {
                    let digit: u64 = (*digit_map.get(&c).unwrap_or(&0)).into();
                    acc * 10 + digit
                })
            })
            .sum()
    }

    fn combo(
        k: usize,
        w: &[char],
        f: &[char],
        ans: &mut HashMap<char, u8>,
        last_v: &[&str],
        rest_v: &[&str],
    ) -> Option<HashMap<char, u8>> {
        let n = w.len();

        if k == 0 {
            if cal(ans, last_v) == cal(ans, rest_v) {
                return Some(ans.clone());
            } else {
                return None;
            }
        };

        for i in 0..=9 {
            if (i == 0 && f.contains(&w[n - k])) || ans.values().any(|&v| v == i) {
                continue;
            } else {
                ans.insert(w[n - k], i);
                match combo(k - 1, w, f, ans, last_v, rest_v) {
                    Some(result) => return Some(result),
                    None => {
                        ans.remove(&w[n - k]);
                    }
                }
            }
        }
        None
    }

    let v: Vec<&str> = input
        .split([' ', '+', '='])
        .filter(|s| !s.is_empty())
        .collect();

    let (last, rest) = v.split_last()?;

    let last_v: &[&str] = &[*last];
    let rest_v: &[&str] = rest;

    let f_set: HashSet<char> = v
        .iter()
        .copied()
        .map(|c| c.chars().next().unwrap())
        .collect();

    let f: Vec<char> = f_set.into_iter().collect();
    let w_set: HashSet<char> = v.iter().flat_map(|s| s.chars()).collect();
    let mut w: Vec<char> = w_set.into_iter().collect();
    w.sort();
    let n = w.len();
    let mut ans = HashMap::<char, u8>::new();

    combo(n, &w, &f, &mut ans, last_v, rest_v)
}
