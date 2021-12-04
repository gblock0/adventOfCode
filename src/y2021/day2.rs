use crate::helpers;

pub fn q1() {
    let mut depth = 0;
    let mut horizonal_position = 0;
    helpers::read_and_split_file("./inputs/2021/day2.txt")
        .into_iter()
        .for_each(|x| {
            let split: Vec<&str> = x.split(' ').collect();
            let movement = split[1].parse::<i32>().unwrap();
            match split[0] {
                "forward" => horizonal_position = horizonal_position + movement,
                "up" => depth = depth - movement,
                "down" => depth = depth + movement,
                _ => {}
            }
        });

    println!(
        "2021 day 2 q1: depth x horizonal_position: {}",
        depth * &horizonal_position
    );
}

pub fn q2() {
    let mut depth = 0;
    let mut horizonal_position = 0;
    let mut aim = 0;
    helpers::read_and_split_file("./inputs/2021/day2.txt")
        .into_iter()
        .for_each(|x| {
            let split: Vec<&str> = x.split(' ').collect();
            let movement = split[1].parse::<i32>().unwrap();
            match split[0] {
                "forward" => {
                    horizonal_position = horizonal_position + movement;
                    depth = depth + aim * movement;
                }
                "up" => {
                    aim = aim - movement;
                }
                "down" => {
                    aim = aim + movement;
                }
                _ => {
                    // Just ignore
                }
            }
        });

    println!(
        "2021 day2 q2: depth x horizonal_position: {}",
        depth * &horizonal_position
    );
}
