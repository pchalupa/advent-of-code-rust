use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

use crate::utils::{print_result, read_input_as_buffer};

#[derive(PartialEq)]
enum Order {
    Asc,
    Desc,
    Eq,
}

/** Day number */
const DAY: u8 = 2;

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

    for line in lines {
        let values: Vec<i32> = line
            .expect("Missing line")
            .split_whitespace()
            .map(|value| value.parse().expect("Not able to parse the value"))
            .collect();
        let mut is_safe = true;
        let mut order: Order = Order::Eq;

        for (index, number) in values.iter().enumerate() {
            let count = values.len();
            let next_number = if index < (count - 1) {
                values.get(index + 1).expect("foo")
            } else {
                break;
            };

            if index == 0 && number < next_number {
                order = Order::Asc
            } else if index == 0 && number > next_number {
                order = Order::Desc
            }

            if ((number > next_number && order == Order::Desc)
                || (number < next_number && order == Order::Asc))
                && number.abs_diff(*next_number) <= 3
            {
                continue;
            } else {
                is_safe = false;
                break;
            }
        }

        if is_safe {
            result += 1;
        }
    }

    result
}

fn task_2(lines: Lines<BufReader<File>>) -> u32 {
    let mut result: u32 = 0;

    for line in lines {
        let mut values: Vec<i32> = line
            .expect("Missing line")
            .split_whitespace()
            .map(|value| value.parse().expect("Not able to parse the value"))
            .collect();
        let mut is_safe = true;
        let mut is_partially_safe = true;
        let mut order: Order = Order::Eq;
        let mut passes: u8 = 0;

        while passes < 2 {
            for index in 0..values.len() {
                let number = values.get(index).expect("Unable to get number");
                let next_number = if index < (values.len() - 1) {
                    values.get(index + 1).expect("Unable to get next number")
                } else {
                    break;
                };

                if index == 0 && number < next_number {
                    order = Order::Asc
                } else if index == 0 && number > next_number {
                    order = Order::Desc
                }

                if ((number > next_number && order == Order::Desc)
                    || (number < next_number && order == Order::Asc))
                    && number.abs_diff(*next_number) <= 3
                {
                    continue;
                } else if !is_safe {
                    is_partially_safe = false;
                    break;
                } else {
                    values.remove(index);
                    is_safe = false;
                    break;
                }
            }
            passes += 1;
        }

        if is_safe || is_partially_safe {
            result += 1;
        }
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

        assert_eq!(result, 2)
    }

    #[test]
    fn test_task_2() {
        let reader = read_input_as_buffer(DAY, true);
        let lines = reader.lines();
        let result = task_2(lines);

        assert_eq!(result, 4)
    }
}
