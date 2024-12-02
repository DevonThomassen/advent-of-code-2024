use filereader::FileReader;
use std::process;

const DIFFERENCE_THRESHOLD: i32 = 3;
const FILE_PATH: &str = "../resources/input.txt";

fn main() {
    let file_result = FileReader::read_file(FILE_PATH);
    let input = file_result.unwrap_or_else(|e| {
        eprintln!("{:?}", e);
        process::exit(1);
    });

    let mut answer = 0;
    let lines: Vec<&str> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    for line in lines {
        if is_line_safe(line) {
            answer += 1;
        }
    }
    println!("{}", answer);
}

fn is_line_safe(line: &str) -> bool {
    let numbers: Vec<i32> = line
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap_or_else(|_| process::exit(1)))
        .collect();

    let positive = (numbers[0] - numbers[1]) > 0;
    for window in numbers.windows(2) {
        let [left, right] = match *window {
            [left, right] => [left, right],
            _ => continue,
        };

        let difference = left - right;
        if difference == 0 || ((difference > 0) != positive) {
            return false;
        }
        if difference.abs() > DIFFERENCE_THRESHOLD {
            return false;
        }
    }
    true
}
