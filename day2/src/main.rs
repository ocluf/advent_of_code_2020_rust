use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("reading file...");
    let contents = fs::read_to_string(filename).unwrap();
    let lines = contents.lines();
    let mut validPasswords = 0;
    for line in lines{
        let mut iter = line.split_whitespace();
        let bounds : Vec<u32> = iter.next().unwrap().split("-").map(|x| x.parse::<u32>().unwrap()).collect();
        let min = bounds[0];
        let max = bounds[1];
        let ruleLetter = iter.next().unwrap().replace(":","").chars().next().unwrap();
        let characters = iter.next().unwrap().chars();
        let mut characterCount = 0;
        for c in characters {
            if c == ruleLetter {
                characterCount = characterCount + 1;
            }
        }
        if characterCount >= min && characterCount <= max {
            validPasswords = validPasswords + 1;
        }
    }
    println!("number of valid passwords: {}", validPasswords)
}
