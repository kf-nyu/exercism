pub fn encode(n: u64) -> String {
    let power_names = [
        "thousand",
        "million",
        "billion",
        "trillion",
        "quadrillion",
        "quintillion",
    ];
    let word_len = n.to_string().len();
    let power_of_k = (word_len - 1) / 3;
    let numbers = n / (1000u64.pow(power_of_k as u32));

    if n == 0 {
        "zero".to_string()
    } else {
        if power_of_k == 0 {
            three_numbers_wording(n)
        } else {
            if n - numbers * 1000u64.pow(power_of_k as u32) != 0 {
                three_numbers_wording(numbers)
                    + " "
                    + power_names[power_of_k - 1]
                    + " "
                    + &encode(n - numbers * 1000u64.pow(power_of_k as u32))
            } else {
                three_numbers_wording(numbers) + " " + power_names[power_of_k - 1]
            }
        }
    }
}

fn three_numbers_wording(n: u64) -> String {
    let number_words: [&str; 21] = [
        "",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
        "twenty",
    ];
    let power_of_ten: [&str; 8] = [
        "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    let mut sol = String::new();
    if n == 0 {
        return "".to_string();
    } else {
        let mut remain = n;
        if n >= 100 {
            sol.push_str(number_words[(n / 100) as usize]);
            sol.push_str(" hundred");
            remain = n % 100;
            if remain != 0 {
                sol.push(' ')
            }
        };
        if remain > 20 {
            sol.push_str(power_of_ten[(remain / 10 - 2) as usize]);
            remain %= 10;
            if remain != 0 {
                sol.push('-')
            }
        }
        sol.push_str(number_words[remain as usize])
    }
    sol
}
