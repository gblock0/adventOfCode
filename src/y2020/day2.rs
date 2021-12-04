use crate::helpers;

pub fn q1() {
    let split_content = helpers::read_and_split_file("./inputs/day2.txt");
    let mut valid_count = 0;

    for row in split_content {
        let row_split: Vec<&str> = row.split(':').collect();
        let password = row_split[1];
        let first_part_split: Vec<&str> = row_split[0].split(' ').collect();
        let letter_looking_for = first_part_split[1].parse::<char>().unwrap();
        let nums: Vec<&str> = first_part_split[0].split('-').collect();

        let lower_bound = nums[0].parse::<i32>().unwrap();
        let upper_bound = nums[1].parse::<i32>().unwrap();

        let mut letter_count = 0;

        for letter in password.chars() {
            if letter == letter_looking_for {
                letter_count = letter_count + 1;
            }
        }

        if letter_count >= lower_bound && letter_count <= upper_bound {
            valid_count = valid_count + 1;
        }
    }

    println!("Answer 1: Valid Count == {}", valid_count);
}

pub fn q2() {
    let split_content = helpers::read_and_split_file("./inputs/day2.txt");
    let mut valid_count = 0;

    for row in split_content {
        let row_split: Vec<&str> = row.split(':').collect();
        let password = row_split[1].trim();
        let first_part_split: Vec<&str> = row_split[0].split(' ').collect();
        let letter_looking_for = first_part_split[1].parse::<char>().unwrap();
        let nums: Vec<&str> = first_part_split[0].split('-').collect();

        let lower_bound = nums[0].parse::<usize>().unwrap() - 1;
        let upper_bound = nums[1].parse::<usize>().unwrap() - 1;

        let letters: Vec<char> = password.chars().collect();

        let mut count = 0;
        if lower_bound < letters.len() && letters[lower_bound] == letter_looking_for {
            count = count + 1;
        }

        if upper_bound < letters.len() && letters[upper_bound] == letter_looking_for {
            count = count + 1;
        }

        if count == 1 {
            valid_count = valid_count + 1;
        }
    }

    println!("Answer 2: Valid Count == {}", valid_count);
}
