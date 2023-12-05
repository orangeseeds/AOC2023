use std::{collections::HashMap, fs};
pub fn solve_part_one() {
    let path: &str = "./data/one.txt";
    let err_msg = format!("{} not found!", path);
    let contents = fs::read_to_string(&path).expect(&err_msg);

    let mut sum = 0;
    let mut all_nums: Vec<u32> = Vec::new();
    let mut line_numbers: Vec<char> = Vec::new();

    for ln in contents.lines() {
        for c in ln.chars() {
            if c.is_numeric() {
                line_numbers.push(c);
            }
        }

        if line_numbers.len() == 0 {
            all_nums.push(0);
        } else {
            let entire_num = format!(
                "{}{}",
                line_numbers[0],
                line_numbers[line_numbers.len() - 1]
            );

            let parsed = match entire_num.parse::<u32>() {
                Ok(val) => val,
                Err(_) => 0,
            };

            all_nums.push(parsed);
        }
        line_numbers.clear();
    }

    for n in all_nums {
        sum += n;
    }

    println!("[Day 1 - part one] The sum is: {}", sum);
}

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn solve_part_two() {
    let mut numbers = HashMap::new();
    for (i, el) in NUMBERS.iter().enumerate() {
        numbers.insert(*el, (i + 1).to_string());
    }

    let path: &str = "./data/one.txt";
    let err_msg = format!("{} not found!", path);
    let contents = fs::read_to_string(&path).expect(&err_msg);

    let mut sum = 0;
    let mut all_nums: Vec<u32> = Vec::new();
    let mut line_numbers: Vec<char> = Vec::new();
    for ln in contents.lines() {
        let mut replaced_line = ln.to_string();
        for n in NUMBERS {
            let end = n.get(n.len() - 1..n.len()).unwrap_or_else(|| "");
            let start = n.get(0..1).unwrap_or_else(|| "");
            let replacing = format!("{}{}{}", start, numbers[n], end);
            replaced_line = replaced_line.replace(n, &replacing);
        }

        for c in replaced_line.chars() {
            if c.is_numeric() {
                line_numbers.push(c);
            }
        }

        if line_numbers.len() == 0 {
            all_nums.push(0);
        } else {
            let entire_num = format!(
                "{}{}",
                line_numbers[0],
                line_numbers[line_numbers.len() - 1]
            );

            let parsed = match entire_num.parse::<u32>() {
                Ok(val) => val,
                Err(_) => 0,
            };

            all_nums.push(parsed);
        }

        // println!("{:?}", all_nums);
        line_numbers.clear();
    }
    for n in all_nums {
        sum += n;
    }

    println!("[Day 1 - part two] The sum is: {}", sum);
}
