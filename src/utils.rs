use std::{
    fs::File,
    io::{self, BufReader},
    path::Path,
};

pub fn read_input_as_buffer(day: u8, test: bool) -> BufReader<File> {
    let file_name: String = if test {
        format!("input/{day}_test.txt")
    } else {
        format!("input/{day}.txt")
    };
    let path = Path::new(&file_name);
    let file = File::open(path).expect("Failed to open the file!");

    io::BufReader::new(file)
}

pub fn print_result(task_1: u32, task_2: u32) {
    println!("======================");
    println!("Task 1: {}\nTask 2: {}", task_1, task_2);
    println!("======================");
}
