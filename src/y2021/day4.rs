use crate::helpers;
use std::collections::HashSet;

fn parse_bingo_numbers(file_path: &str) -> Vec<usize> {
    helpers::read_and_split_file(file_path)[0]
        .split(',')
        .into_iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn parse_bingo_boards(file_path: &str) -> Vec<Vec<Vec<usize>>> {
    let rest_of_file: Vec<String> = helpers::read_and_split_file(file_path)
        .into_iter()
        .skip(2)
        .collect();

    let mut boards: Vec<Vec<Vec<usize>>> = Vec::new();
    boards.push(Vec::new());
    let mut board_number = 0;

    for row in rest_of_file {
        if row.len() == 0 {
            board_number = board_number + 1;
            boards.push(Vec::new());
            continue;
        }

        let mut row_nums = Vec::new();
        row.split(" ").into_iter().for_each(|s| {
            if s.len() != 0 {
                row_nums.push(s.parse::<usize>().unwrap());
            }
        });

        boards[board_number].push(row_nums);
    }

    boards
}

#[allow(dead_code)]
pub fn q1() {
    let file_path = "./inputs/2021/day4.txt";
    let bingo_numbers = parse_bingo_numbers(file_path);
    let boards = parse_bingo_boards(file_path);

    let mut called_numbers = HashSet::new();
    for bingo_number in bingo_numbers {
        println!("CALLING: {}", bingo_number);
        called_numbers.insert(bingo_number);
        let mut found_bingo = false;

        // Go through horizantal rows
        for board in &boards {
            let mut sum_of_unmarked_numbers = 0;
            for row in board {
                let mut found_all_numbers_in_row = true;
                for row_number in row {
                    if !called_numbers.contains(&row_number) {
                        found_all_numbers_in_row = false;
                        sum_of_unmarked_numbers += row_number;
                    }
                }
                if found_all_numbers_in_row {
                    found_bingo = true;
                }
            }

            if found_bingo {
                println!(
                    "2021 day4 q1: {} * {} = {}",
                    sum_of_unmarked_numbers,
                    bingo_number,
                    sum_of_unmarked_numbers * bingo_number
                );
                return;
            }
        }

        found_bingo = false;
        // go through vertical rows
        for board in &boards {
            let mut sum_of_unmarked_numbers = 0;
            let mut col_counter = 0;
            let mut row_counter = 0;

            loop {
                if col_counter >= board.len() {
                    break;
                }

                let mut found_all_numbers_in_row = true;
                loop {
                    if row_counter >= board[col_counter].len() {
                        break;
                    }
                    let row_number = board[col_counter][row_counter];

                    if !called_numbers.contains(&row_number) {
                        found_all_numbers_in_row = false;
                        sum_of_unmarked_numbers += row_number;
                    }
                    row_counter += 1;
                }
                if found_all_numbers_in_row {
                    found_bingo = true;
                }

                col_counter += 1;
                row_counter = 0;
            }

            if found_bingo {
                println!(
                    "2021 day4 q1: {} * {} = {}",
                    sum_of_unmarked_numbers,
                    bingo_number,
                    sum_of_unmarked_numbers * bingo_number
                );
                return;
            }
        }

        println!("COULD NOT FIND BINGO!");
    }
}

// Not working yet
pub fn q2() {
    let file_path = "./inputs/2021/day4-test.txt";
    let bingo_numbers = parse_bingo_numbers(file_path);
    let mut boards = parse_bingo_boards(file_path);

    let mut called_numbers = HashSet::new();
    for bingo_number in bingo_numbers {
        println!("CALLING: {}, boards.len(): {}", bingo_number, boards.len());
        called_numbers.insert(bingo_number);
        let mut found_bingo = false;
        let mut bad_boards_indexes = Vec::new();

        // Go through horizantal rows
        boards.iter().enumerate().for_each(|(i, board)| {
            let mut sum_of_unmarked_numbers = 0;
            for row in board {
                let mut found_all_numbers_in_row = true;
                for row_number in row {
                    if !called_numbers.contains(&row_number) {
                        found_all_numbers_in_row = false;
                        sum_of_unmarked_numbers += row_number;
                    }
                }
                if found_all_numbers_in_row {
                    found_bingo = true;
                }
            }

            if found_bingo {
                bad_boards_indexes.push(i);
            }
            if found_bingo && boards.len() == 1 {
                println!(
                    "2021 day4 q2: {} * {} = {}",
                    sum_of_unmarked_numbers,
                    bingo_number,
                    sum_of_unmarked_numbers * bingo_number
                );
                return;
            }
        });

        bad_boards_indexes
            .iter()
            .enumerate()
            .for_each(|(i, bad_board_index)| {
                if boards.len() == 1 {
                    boards.remove(bad_board_index - i);
                }
            });

        bad_boards_indexes = Vec::new();
        found_bingo = false;
        // go through vertical rows
        boards.iter().enumerate().for_each(|(i, board)| {
            let mut sum_of_unmarked_numbers = 0;
            let mut col_counter = 0;
            let mut row_counter = 0;

            loop {
                if col_counter >= board.len() {
                    break;
                }

                let mut found_all_numbers_in_row = true;
                loop {
                    if row_counter >= board[col_counter].len() {
                        break;
                    }
                    let row_number = board[col_counter][row_counter];

                    if !called_numbers.contains(&row_number) {
                        found_all_numbers_in_row = false;
                        sum_of_unmarked_numbers = sum_of_unmarked_numbers + row_number;
                    }
                    row_counter = row_counter + 1;
                }
                if found_all_numbers_in_row {
                    found_bingo = true;
                }

                col_counter = col_counter + 1;
                row_counter = 0;
            }

            if found_bingo {
                bad_boards_indexes.push(i);
            }
            if found_bingo {
                println!(
                    "2021 day4 q1: {} * {} = {}",
                    sum_of_unmarked_numbers,
                    bingo_number,
                    sum_of_unmarked_numbers * bingo_number
                );
                return;
            }
        });
        bad_boards_indexes
            .iter()
            .enumerate()
            .for_each(|(i, bad_board_index)| {
                if boards.len() > 1 {
                    boards.remove(bad_board_index - i);
                }
            });
    }
    println!("COULD NOT FIND BINGO!");
}
