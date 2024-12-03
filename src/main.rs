use regex::Regex;
use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    day_one(path);
    day_two(path);
    day_three(path);
}

fn day_three(path: &str) {
    let mut total: u32 = 0;
    let mut flag: bool = true;
    let re = Regex::new(r"mul\((\d+),(\d+)\)()|()()(do(?:n't)?\(\))").unwrap();
    let input = read_to_string(path.to_owned() + "3").unwrap();
    for (_, [x, y, z]) in re.captures_iter(&input).map(|m| m.extract()) {
        if z.is_empty() && flag {
            total += x.parse::<u32>().unwrap() * y.parse::<u32>().unwrap();
        } else if z == "don't()" {
            flag = false;
        } else if z == "do()" {
            flag = true;
        }
    }
    println!("{}", total);
}

fn day_two(path: &str) -> u32 {
    let mut total: u32 = 0;
    for line in read_to_string(path.to_owned() + "2").unwrap().lines() {
        // Determine if consistently increasing or decreasing
        let report: Vec<u32> = line
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        if is_report_safe(&report, false) {
            total += 1;
        }
    }
    total
}

fn is_report_safe(report: &Vec<u32>, problem_dampener_used: bool) -> bool {
    let mut last_item: u32 = 0;
    let increasing: bool = report[0] < report[1];
    let mut safe: bool = true;
    for index in 0..report.len() {
        let item = report[index];
        if last_item == 0 {
            last_item = item;
            continue;
        }
        let difference: i32 = item as i32 - last_item as i32;
        // if absolute value of difference is less than one or greater than 3, break and mark unsafe
        // if difference is negative (decreasing) and increasing is true, break and mark unsafe
        // if, conversely, difference is positive but increasing is false, break
        if difference.abs() < 1
            || difference.abs() > 3
            || (difference < 0 && increasing)
            || (difference > 0 && !increasing)
        {
            if problem_dampener_used {
                safe = false;
                break;
            } else {
                let mut found_subreport = false;
                for j in 0..report.len() {
                    let mut sub_report = report.clone();
                    sub_report.remove(j);
                    if is_report_safe(&sub_report, true) {
                        found_subreport = true;
                        break;
                    }
                }
                safe = found_subreport
            }
        }
        // else carry on
        last_item = item;
    }
    safe
}

fn day_one(path: &str) {
    // Get length of file
    let mut first_total = 0;
    let mut second_total = 0;
    // Make an array of ints for each list
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    // Read file of historically significant location IDs
    for line in read_to_string(path.to_owned() + "1").unwrap().lines() {
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
