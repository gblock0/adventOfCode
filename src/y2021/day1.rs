use crate::helpers;

pub fn q1() {
    let mut num_increased = 0;
    let split_cont: Vec<i32> = helpers::read_and_split_file("./inputs/2021/day1.txt")
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    for (i, x) in split_cont.iter().enumerate() {
        if i > 0 && x - split_cont[i - 1] > 0 {
            num_increased = num_increased + 1;
        }
    }

    println!("Num Increased: {}", num_increased);
}

pub fn q2() {
    let mut num_increased = 0;
    let split_cont: Vec<i32> = helpers::read_and_split_file("./inputs/2021/day1.txt")
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    for (i, x) in split_cont.iter().enumerate() {
        if i > 2 && x > &split_cont[i - 3] {
            num_increased = num_increased + 1;
        }
    }

    println!("Num Increased: {}", num_increased);
}
