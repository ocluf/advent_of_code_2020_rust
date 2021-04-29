use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();
    part_one(&contents);
    part_two(&contents);
}

fn part_one(contents: &str) {
    let tree_indexes = get_tree_indexes(&contents);
    let line_length = get_line_length(&contents);
    let trees_hit = make_flight(&tree_indexes, 3, &line_length, 1);
    println!("number of trees hit: {}", trees_hit);
}

fn part_two(contents: &str) {
    let tree_indexes = get_tree_indexes(&contents);
    let line_length = get_line_length(&contents);
    let right_1_down_1 = make_flight(&tree_indexes, 1, &line_length, 1);
    let right_3_down_1 = make_flight(&tree_indexes, 3, &line_length, 1);
    let right_5_down_1 = make_flight(&tree_indexes, 5, &line_length, 1);
    let right_7_down_1 = make_flight(&tree_indexes, 7, &line_length, 1);
    let right_1_down_2 = make_flight(&tree_indexes, 1, &line_length, 2);
    println!("{}", right_1_down_2);

    let answer = right_1_down_1 * right_1_down_2 * right_3_down_1 * right_5_down_1 * right_7_down_1;
    println!("answer part two: {}", answer);
}

fn get_line_length(contents: &str) -> usize {
    let mut lines = contents.lines();
    return lines.next().unwrap().len();
}

fn get_tree_indexes(contents: &str) -> Vec<Vec<usize>> {
    let lines = contents.lines();
    let mut result: Vec<Vec<usize>> = Vec::new();
    for line in lines {
        let mut tree_indexes: Vec<usize> = Vec::new();
        for (index, char) in line.chars().enumerate() {
            if char == '#' {
                tree_indexes.push(index);
            }
        }
        result.push(tree_indexes);
    }
    println!("{:#?}", result);
    return result;
}

fn make_flight(
    tree_indexes: &Vec<Vec<usize>>,
    angle: usize,
    line_length: &usize,
    step_size: usize,
) -> i64 {
    let mut nr_of_trees = 0;
    for (index, tree_line) in tree_indexes.iter().enumerate() {
        let position = (index / step_size) * angle; // currently takes normal line
        if index % step_size == 0 && tree_hit(tree_line, &position, line_length) {
            nr_of_trees += 1;
        }
    }
    return nr_of_trees;
}

fn tree_hit(tree_line_indexes: &Vec<usize>, position: &usize, line_length: &usize) -> bool {
    for tree_index in tree_line_indexes.iter() {
        if position % line_length == *tree_index {
            return true;
        }
    }
    return false;
}
