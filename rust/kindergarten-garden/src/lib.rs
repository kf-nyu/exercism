use std::collections::HashMap;

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let list = vec![
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];
    let dic = HashMap::from([
        ('G', "grass"),
        ('C', "clover"),
        ('R', "radishes"),
        ('V', "violets"),
    ]);
    let v: Vec<Vec<char>> = diagram.lines().map(|line| line.chars().collect()).collect();
    let index = list.iter().position(|&name| name == student).unwrap();

    v.iter()
        .flat_map(|line| {
            line.iter()
                .enumerate()
                .filter_map(|(i, ch)| {
                    if i == index * 2 + 1 || i == index * 2 {
                        dic.get(ch)
                    } else {
                        None
                    }
                })
                .copied()
        })
        .collect::<Vec<&'static str>>()
}
