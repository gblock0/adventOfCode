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

pub fn q2() {
    let mut nums_to_check = helpers::read_and_split_file("./inputs/2021/day3.txt");

    let mut index_to_check = 0;

    loop {
        let mut zero_nums: Vec<String> = Vec::new();
        let mut one_nums: Vec<String> = Vec::new();
        nums_to_check.iter().for_each(|x| {
            let nums: Vec<char> = x.chars().into_iter().collect();

            if nums[index_to_check] == '0' {
                zero_nums.push(x.to_string())
            } else {
                one_nums.push(x.to_string());
            }
        });

        if zero_nums.len() > one_nums.len() {
            nums_to_check = zero_nums;
        } else {
            nums_to_check = one_nums;
        }

        if nums_to_check.len() == 1 {
            break;
        }

        index_to_check = index_to_check + 1;
    }

    let oxygen_generator_rating = isize::from_str_radix(&nums_to_check[0], 2).unwrap();
    nums_to_check = helpers::read_and_split_file("./inputs/2021/day3.txt");
    index_to_check = 0;

    loop {
        let mut zero_nums: Vec<String> = Vec::new();
        let mut one_nums: Vec<String> = Vec::new();
        nums_to_check.iter().for_each(|x| {
            let nums: Vec<char> = x.chars().into_iter().collect();

            if nums[index_to_check] == '0' {
                zero_nums.push(x.to_string())
            } else {
                one_nums.push(x.to_string());
            }
        });

        if zero_nums.len() <= one_nums.len() {
            nums_to_check = zero_nums;
        } else {
            nums_to_check = one_nums;
        }

        if nums_to_check.len() == 1 {
            break;
        }

        index_to_check = index_to_check + 1;
    }
    let co2_scrubber_rating = isize::from_str_radix(&nums_to_check[0], 2).unwrap();

    println!(
        "2021 day3 q2: oxygen_generator_rating x CO2 scrubber rating: {} x {} = {}",
        oxygen_generator_rating,
        co2_scrubber_rating,
        oxygen_generator_rating * co2_scrubber_rating
    );
}
