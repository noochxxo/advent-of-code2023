use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {

    let args = parse_args();
    let numbers = read_and_process_file(&args);
    let total = sum_number(&numbers);
    println!("{}", total);


}

fn parse_args() -> String {
    env::args().nth(1).expect("Hey my guy! No argument found.")
}

fn read_and_process_file( filename: &str ) -> Vec<u32> {
    let file = File::open(filename).expect("Nope! Its not oening.");
    let reader = BufReader::new(file);
    reader.lines().enumerate().map(|(_index, line)| {
        let l = line.expect("Can't read this line, aaahhhhh!");
        extract_numbers_from_line(&l)
    }).collect()
}

fn extract_numbers_from_line( line: &str ) -> u32 {
    let mut current_line_number = String::new();
    for ch in line.chars() {
        if ch.is_digit(10) {
            current_line_number.push(ch);
        }
    }

    let mut value = String::new();
    value.push(current_line_number.chars().next().unwrap());
    value.push(current_line_number.chars().last().unwrap());
    value.parse::<u32>().expect("Yeah, we can't parse this number")
}

fn sum_number( numbers: &[u32] ) ->u32 {
    numbers.iter().sum()
}