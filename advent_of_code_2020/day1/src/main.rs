use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("reading file...");
    let contents = fs::read_to_string(filename).unwrap();
    let lines = contents.lines();
    let numbers : Vec<u64> = lines.map(|x| x.parse::<u64>().unwrap()).collect();
    for number in &numbers {
        for number2 in &numbers {
            for number3 in &numbers{
                let test = number + number2 + number3;
                 if test == 2020 {
                    let answer = number * number2 * number3;
                    println!("{} + {} + {} = 2020 answer is: {}", number, number2, number3, answer)
                 }
            }
            
        }
    }

}
