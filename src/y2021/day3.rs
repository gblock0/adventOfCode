use crate::helpers;
use std::iter;

pub fn q1() {
    let mut line_count = 0;
    let lines: Vec<String> = helpers::read_and_split_file("./inputs/2021/day3.txt");

    let mut zero_counts: Vec<u32> = iter::repeat(0).take(lines[0].len()).collect();

    lines.iter().for_each(|x| {
        let nums = x.chars();

        line_count = line_count + 1;

        for (i, num) in nums.into_iter().enumerate() {
            if num == '0' {
                zero_counts[i] = zero_counts[i] + 1;
            }
        }

        // count 0's in each position
        // let split: Vec<&str> = x.split().collect();
        // let movement = split[1].parse::<i32>().unwrap();
        // match split[0] {
        //     "forward" => horizonal_position = horizonal_position + movement,
        //     "up" => depth = depth - movement,
        //     "down" => depth = depth + movement,
        //     _ => {}
        // }
    });

    let mut gamma_rate_str: String = "".to_string();
    let mut epsilon_rate_str: String = "".to_string();
    for (_, count) in zero_counts.into_iter().enumerate() {
        if count > line_count - count {
            gamma_rate_str = format!("{}{}", gamma_rate_str, "0");
            epsilon_rate_str = format!("{}{}", epsilon_rate_str, "1");
        } else {
            gamma_rate_str = format!("{}{}", gamma_rate_str, "1");
            epsilon_rate_str = format!("{}{}", epsilon_rate_str, "0");
        }
    }

    let gamma_rate = isize::from_str_radix(&gamma_rate_str, 2).unwrap();
    let epison_rate = isize::from_str_radix(&epsilon_rate_str, 2).unwrap();
    println!(
        "2021 day3 q1: gamma rate x epsilon: {}",
        gamma_rate * epison_rate
    );
}
