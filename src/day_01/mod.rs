use crate::utils::{print_result, read_input_as_buffer};
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn main() {
    let reader = read_input_as_buffer(1, false);
    let (left_list, right_list) = preprocess(reader);
    let result_1 = task_1(&left_list, &right_list);
    let result_2 = task_2(&left_list, &right_list);

    print_result(result_1, result_2);
}

fn preprocess(reader: BufReader<File>) -> (Vec<u32>, Vec<u32>) {
    let pattern = Regex::new(r"(\d+)\s+(\d+)").expect("Invalid regex");
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Unable to read a line");
        let captures = pattern.captures(&line).expect("Regex not matched");

        let left_value: u32 = captures
            .get(1)
            .expect("Unable to find the first number")
            .as_str()
            .parse::<u32>()
            .expect("Invalid number");
        let right_value: u32 = captures
            .get(2)
            .expect("Unable to find the second number")
            .as_str()
            .parse::<u32>()
            .expect("Invlaid number");

        left_list.push(left_value);
        right_list.push(right_value);
    }

    left_list.sort();
    right_list.sort();

    (left_list, right_list)
}

fn task_1(left_list: &[u32], right_list: &[u32]) -> u32 {
    let mut result: u32 = 0;
    let mut i = 0;

    while i < left_list.len() {
        let &left_value = left_list.get(i).expect("Not able to get value");
        let &right_value = right_list.get(i).expect("Not able to get value");

        result += left_value.abs_diff(right_value);

        i += 1;
    }

    result
}

fn task_2(left_list: &[u32], right_list: &[u32]) -> u32 {
    let mut result: u32 = 0;

    for &value in left_list {
        let count: u32 = right_list
            .iter()
            .filter(|&&x| x == value)
            .count()
            .try_into()
            .expect("Not able to convert");

        result += value * count
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_1() {
        let reader = read_input_as_buffer(1, true);
        let (left_list, right_list) = preprocess(reader);
        let result = task_1(&left_list, &right_list);

        assert_eq!(result, 11)
    }

    #[test]
    fn test_task_2() {
        let reader = read_input_as_buffer(1, true);
        let (left_list, right_list) = preprocess(reader);
        let result = task_2(&left_list, &right_list);

        assert_eq!(result, 31)
    }
}
