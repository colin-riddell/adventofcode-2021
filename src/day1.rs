// use error_chain::error_chain;
use std::fs;
use std::env;

pub fn run(){
    let lines = read_input("day1-input.txt");
    let mut count: usize =  0;

    // part 1
    // for (index, line) in lines.iter().enumerate() {
    //     if index + 1 < lines.len() {
    //         if lines[index + 1] > *line {
    //             count += 1;
    //         }
    //     }
    // }

    // part 2
    let mut lines_windowed = lines.windows(3).peekable();

    while let Some(current) = lines_windowed.next() {
        if let Some(next) = lines_windowed.peek() {
            if next.iter().sum::<usize>() > current.iter().sum() {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

fn read_input(file_name: &str)  -> Vec<usize>{
    std::fs::read_to_string(file_name)
    .unwrap()
    .split('\n')
    .into_iter()
    .filter(|s| !s.is_empty())
    .map(|str| str.parse::<usize>().unwrap())
    .collect()
}