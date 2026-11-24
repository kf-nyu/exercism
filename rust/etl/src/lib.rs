use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    //    let mut sol = BTreeMap::<char, i32>::new();

    //    for (point, v_ch) in h.iter() {
    //        for ch in v_ch.iter() {
    //            sol.insert((*ch).to_lowercase().next().unwrap(), *point);
    //        }
    //    }
    //    sol

    h.iter()
        .flat_map(|(&point, v_ch)| v_ch.iter().map(move |c| (c.to_ascii_lowercase(), point)))
        .collect()
}
