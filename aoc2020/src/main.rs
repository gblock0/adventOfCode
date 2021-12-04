mod y2020;
use y2020::day1;
use y2020::day2;
pub mod helpers;

struct Cli {
    year: usize,
    day: usize,
}

fn main() {
    let year = std::env::args().nth(1).expect("no pattern given");
    let day = std::env::args().nth(2).expect("no path given");
    let args = Cli {
        year: year.parse::<usize>().unwrap(),
        day: day.parse::<usize>().unwrap(),
    };

    day1::q1();
    day1::q2();
    day2::q1();
    day2::q2();
}
