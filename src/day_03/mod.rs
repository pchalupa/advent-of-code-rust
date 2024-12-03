use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

use regex::Regex;

use crate::utils::{print_result, read_input_as_buffer};

/** Day number */
const DAY: u8 = 3;

pub fn main() {
    let reader = read_input_as_buffer(DAY, false);
    let lines = reader.lines();
    let result_1 = task_1(lines);

    let reader = read_input_as_buffer(DAY, false);
    let lines = reader.lines();
    let result_2 = task_2(lines);

    print_result(result_1, result_2);
}

fn task_1(lines: Lines<BufReader<File>>) -> u32 {
    let mut result: u32 = 0;
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");

    for line in lines {
        let line = line.expect("Not able to get the line");

        for capture in pattern.captures_iter(&line) {
            let x: u32 = capture
                .get(1)
                .expect("Not able to get value")
                .as_str()
                .parse()
                .expect("Not able to parse the first number");
            let y: u32 = capture
                .get(2)
                .expect("Not able to get value")
                .as_str()
                .parse()
                .expect("Not able to parse the second number");

            result += x * y;
        }
    }

    result
}

fn task_2(lines: Lines<BufReader<File>>) -> u32 {
    let mut result: u32 = 0;
    let mut joined_line = String::new();

    let command_pattern = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");
    let filter_pattern = Regex::new(r"don't\(\).*?do\(\)").expect("Invalid regex");

    for line in lines {
        let line = line.expect("Unable to get line");

        joined_line.push_str(&line);
    }

    let filtered_line = filter_pattern.replace_all(&joined_line, "").into_owned();

    for capture in command_pattern.captures_iter(&filtered_line) {
        let x: u32 = capture
            .get(1)
            .expect("Not able to get value")
            .as_str()
            .parse()
            .expect("Not able to parse the first number");
        let y: u32 = capture
            .get(2)
            .expect("Not able to get value")
            .as_str()
            .parse()
            .expect("Not able to parse the second number");

        result += x * y;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_1() {
        let reader = read_input_as_buffer(DAY, true);
        let lines = reader.lines();
        let result = task_1(lines);

        assert_eq!(result, 161)
    }

    #[test]
    fn test_task_2() {
        let reader = read_input_as_buffer(DAY, true);
        let lines = reader.lines();
        let result = task_2(lines);

        assert_eq!(result, 48)
    }
}
