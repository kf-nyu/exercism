pub fn recite(start_bottles: u32, take_down: u32) -> String {

    fn lyric( num1: String, count_1: String, num2: String, count_2: String) -> String {
    format!("{} green {} hanging on the wall,\n{} green {} hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be {} green {} hanging on the wall.", num1, count_1, num1, count_1, num2, count_2)

    }

    fn num2wd(n: u32) -> String {
        match n {
            0 => "No".to_string(),
            1 => "One".to_string(),
            2 => "Two".to_string(),
            3 => "Three".to_string(),
            4 => "Four".to_string(),
            5 => "Five".to_string(),
            6 => "Six".to_string(),
            7 => "Seven".to_string(),
            8 => "Eight".to_string(),
            9 => "Nine".to_string(),
            10 => "Ten".to_string(),
            _  => "Zero".to_string(),
        }
    }

    let mut remain = start_bottles;
    let mut answer = String::new();
    
    for i in 0..take_down {
        if i > 0 {answer.push_str("\n\n")};
        let count_1 = if remain != 1 { "bottles".to_string() } else { "bottle".to_string()};
        let count_2 = if remain != 2 { "bottles".to_string() } else { "bottle".to_string()};
        answer += &lyric(num2wd(remain), count_1, num2wd(remain-1).to_lowercase(), count_2);
        remain -= 1;
        }
    answer
}



