use std::collections::HashMap;
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let passports: Vec<&str> = contents.split("\n\n").collect();
    println!("{:#?}", passports);
    let mut passport_maps = Vec::new();
    for passport in passports {
        let mut passport_fields = HashMap::new();
        let fields: Vec<&str> = passport.split_ascii_whitespace().collect();
        for field in fields {
            let key_value: Vec<&str> = field.split(":").collect();
            passport_fields.insert(key_value[0], key_value[1]);
        }
        passport_maps.push(passport_fields);
    }
    let mut valid_passports = 0;
    for passport_map in passport_maps {
        let mut valid = true;
        for &required_field in &required_fields {
            if !passport_map.contains_key(required_field) {
                valid = false;
            }
        }
        if valid {
            valid_passports += 1;
        }
    }
    println!("valid passport = {}", valid_passports)
}
