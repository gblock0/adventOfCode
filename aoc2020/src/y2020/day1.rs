use std::collections::HashSet;

use crate::helpers;

pub fn q1() {
    let split_cont: Vec<i32> = helpers::read_and_split_file("./inputs/day1.txt")
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let mut diff_nums = HashSet::new();
    for num in split_cont {
        let diff = 2020 - num;

        if diff_nums.contains(&diff) {
            println!("Answer 1: {} * {} = {}", diff, num, diff * num);
        } else {
            diff_nums.insert(num);
        }
    }
}
pub fn q2() {
    let split_cont: Vec<i32> = helpers::read_and_split_file("./inputs/day1.txt")
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // I know this is gross, I'll get around to fixing it eventually
    for num1 in &split_cont {
        for num2 in &split_cont {
            for num3 in &split_cont {
                if 2020 == num1 + num2 + num3 {
                    println!(
                        "Answer 2: {} * {} * {} = {}",
                        num1,
                        num2,
                        num3,
                        num1 * num2 * num3
                    );
                    return;
                }
            }
        }
    }
}
