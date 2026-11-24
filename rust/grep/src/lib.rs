use anyhow::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
#[derive(Debug)]
pub struct Flags {
    line_number: bool,
    file_names: bool,
    case_insensitive: bool,
    invert: bool,
    entire_line: bool,
}
impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut line_number = false;
        let mut file_names = false;
        let mut case_insensitive = false;
        let mut invert = false;
        let mut entire_line = false;

        for &flag in flags.iter() {
            match flag {
                "-n" => line_number = true,
                "-l" => file_names = true,
                "-i" => case_insensitive = true,
                "-v" => invert = true,
                "-x" => entire_line = true,
                _ => {}
            }
        }
        Self {
            line_number,
            file_names,
            case_insensitive,
            invert,
            entire_line,
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut result_contents: Vec<String> = Vec::new();

    for file_name in files.iter() {
        let file = File::open(file_name)?;
        let reader = BufReader::new(file);

        for (i, line) in reader.lines().enumerate() {
            let mut line = line?;
            let mut line_low = line.clone();
            let mut pattern_chk: String = pattern.to_string();
            if flags.case_insensitive {
                line_low = line_low.to_ascii_lowercase();
                pattern_chk.make_ascii_lowercase();
            }

            if flags.entire_line
                && ((line_low != pattern_chk) && !flags.invert
                    || (line_low == pattern_chk) && flags.invert)
            {
                continue;
            }

            if (line_low.contains(&pattern_chk)) && !flags.invert
                || !(line_low.contains(&pattern_chk)) && flags.invert
            {
                if flags.file_names {
                    result_contents.push(file_name.to_string());
                    break;
                } else {
                    if flags.line_number {
                        line = format!("{}:{}", i + 1, line);
                    };
                    if files.len() > 1 {
                        line = format!("{}:{}", file_name, line)
                    }
                }
                result_contents.push(line)
            }
        }
    }
    Ok(result_contents)
}
