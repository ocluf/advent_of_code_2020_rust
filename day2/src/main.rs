use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).unwrap();
    let result = part_one(&contents);
    println!("result part 1 = {}", result);

    let result = part_two(&contents);
    println!("result part 2 = {}", result)
}

struct Row {
    min: i32,
    max: i32,
    rule_letter: char,
    password: String,
}

fn parse_row(line: &str) -> Row {
    let mut iter = line.split_whitespace();
    let bounds: Vec<i32> = iter
        .next()
        .unwrap()
        .split("-")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let min = bounds[0];
    let max = bounds[1];
    let rule_letter = iter
        .next()
        .unwrap()
        .replace(":", "")
        .chars()
        .next()
        .unwrap();
    let password = String::from(iter.next().unwrap());

    return Row {
        min,
        max,
        rule_letter,
        password,
    };
}

fn part_one(contents: &str) -> i32 {
    let lines = contents.lines();
    let mut valid_passwords = 0;
    for line in lines {
        let row = parse_row(line);
        let mut character_count = 0;
        for c in row.password.chars() {
            if c == row.rule_letter {
                character_count = character_count + 1;
            }
        }
        if character_count >= row.min && character_count <= row.max {
            valid_passwords = valid_passwords + 1;
        }
    }
    return valid_passwords;
}

fn part_two(contents: &str) -> i32 {
    let lines = contents.lines();
    let mut valid_passwords = 0;
    for line in lines {
        let row = parse_row(line);
        let condition1 = row.rule_letter == row.password.as_bytes()[(row.min - 1) as usize] as char;
        let condition2 = row.rule_letter == row.password.as_bytes()[(row.max - 1) as usize] as char;
        if condition1 != condition2 {
            valid_passwords += 1
        }
    }
    return valid_passwords;
}
