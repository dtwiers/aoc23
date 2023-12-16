use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

enum NumberToken {
    Number(u8),
    Other,
}


const spelled_out_numbers: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

// Tokenize with spelled out or numeric numbers, even without whitespace
fn tokenize(input: String) -> Vec<NumberToken> {
    let mut tokens: Vec<NumberToken> = Vec::new();
    let mut current = input.clone();
    for number in spelled_out_numbers.iter() {
        let mut split = current.split(number);
        let mut result = split.next().unwrap().to_string();
        for _ in 0..number.len() {
            result.push(' ');
        }
        result.push_str(split.next().unwrap());
        current = result;
    }

}

trait Sliceable {
    fn slice(&self, start: usize, end: usize) -> Self;
    // fn starts_with(&self, other: &Self) -> bool;
}

impl Sliceable for String {
    fn slice(&self, start: usize, end: usize) -> String {
        let mut result = String::new();
        for (i, c) in self.chars().enumerate() {
            if i >= start && i < end {
                result.push(c);
            }
        }
        result
    }

    // fn starts_with(&self, other: &Self) -> bool {
    //     let mut result = true;
    //     for (i, c) in other.chars().enumerate() {
    //         if self.chars().nth(i) != other.chars().nth(i) {
    //             result = false;
    //         }
    //     }
    //     result
    // }
}

pub fn parse_calibration_document(input: String) -> u64 {
    input
        .lines()
        .map(|line| {
            let num1 = line.find(char::is_numeric).unwrap();
            let num2 = line.rfind(char::is_numeric).unwrap();
            let bytes = line.as_bytes();
            let char1: char = bytes[num1].into();
            let char2: char = bytes[num2].into();
            println!("num1: {}, num2: {}, orig: {}", char1, char2, line);
            let result: u64 = format!("{}{}", char1, char2).parse().unwrap();
            // println!("{:?}", result);
            return result;
        })
        .sum()
}

#[test]
fn test_calibration() {
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;
    let path = Path::new("./src/day1input.txt");
    let mut file = File::open(path).unwrap();
    let mut s = String::new();
    let _ = file.read_to_string(&mut s).unwrap();
    println!("{}", parse_calibration_document(s));
}

pub fn run() {
    let path = Path::new("./src/day1input.txt");
    let mut file = File::open(path).unwrap();
    let mut s = String::new();
    let _ = file.read_to_string(&mut s).unwrap();
    println!("{}", parse_calibration_document(s));
}
