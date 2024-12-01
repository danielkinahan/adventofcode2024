use std::fs::read_to_string;

fn main() {
    day_one_pt2();
}

fn day_one() {
    // Get length of file
    let mut total = 0;
    // Make an array of ints for each list
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    // Read file of historically significant location IDs
    for line in read_to_string("./input/1").unwrap().lines() {
        //Split by the space to get two list elements
        let item_pair: Vec<&str> = line.split_whitespace().collect();
        left_list.push(item_pair[0].parse::<i32>().unwrap());
        right_list.push(item_pair[1].parse::<i32>().unwrap());
    }
    left_list.sort();
    right_list.sort();

    for index in 0..left_list.len() {
        total += right_list[index].abs_diff(left_list[index]);
    }

    println!("{}", total);
}

fn day_one_pt2() {
    // Get length of file
    let mut total = 0;
    // Make an array of ints for each list
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    // Read file of historically significant location IDs
    for line in read_to_string("./input/1").unwrap().lines() {
        //Split by the space to get two list elements
        let item_pair: Vec<&str> = line.split_whitespace().collect();
        left_list.push(item_pair[0].parse::<i32>().unwrap());
        right_list.push(item_pair[1].parse::<i32>().unwrap());
    }
    left_list.sort();
    right_list.sort();

    for index in 0..left_list.len() {
        let element = left_list[index];
        let count = right_list.iter().filter(|&n| *n == element).count() as i32;
        total += element * count;
    }

    println!("{}", total);
}
