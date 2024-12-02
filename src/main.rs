use std::fs::read_to_string;

fn main() {
    day_one();
    day_two();
}

fn day_two() {
    let mut total: u32 = 0;
    for line in read_to_string("./input/2").unwrap().lines() {
        // Determine if consistently increasing or decreasing
        let report: Vec<u32> = line
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let mut last_item: u32 = 0;
        let increasing: bool = report[0] < report[1];
        let mut safe: bool = true;
        for item in report {
            if last_item == 0 {
                last_item = item;
                continue;
            }
            let difference: i32 = item as i32 - last_item as i32;
            // if absolute value of difference is less than one or greater than 3, break and mark unsafe
            if difference.abs() < 1 || difference.abs() > 3 {
                safe = false;
                break;
            }
            // if difference is negative (decreasing) and increasing is true, break and mark unsafe
            if difference < 0 && increasing == true {
                safe = false;
                break;
            }
            // if, conversely, difference is positive but increasing is false, break
            if difference > 0 && increasing == false {
                safe = false;
                break;
            }
            // else carry on
            last_item = item;
        }
        if safe {
            total += 1;
        }
    }
    println!("{}", total)
}

fn day_one() {
    // Get length of file
    let mut first_total = 0;
    let mut second_total = 0;
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
        first_total += right_list[index].abs_diff(left_list[index]);
        let element = left_list[index];
        let count = right_list.iter().filter(|&n| *n == element).count() as i32;
        second_total += element * count;
    }

    (first_total, second_total);
}
