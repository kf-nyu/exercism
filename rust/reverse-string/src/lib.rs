use unicode_segmentation::UnicodeSegmentation;
pub fn reverse(input: &str) -> String {
//    todo!("Write a function to reverse {input}");
    //let length:usize = input.len() - 1;
    let mut output = String::new();
    //println!("{}", &input[0..0]);
    //for i in 0..(length + 1) {
    //    let  pos = length - i;
    //    println!("{}",&input[pos..(pos+1)]);
    //    output.push_str(&input[pos..(pos+1)])
   // }
   // for c in input.chars().rev() {
    for c in input.graphemes(true).rev() {
        output.push_str(&c.to_string())
    }
    output
}
