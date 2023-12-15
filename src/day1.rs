enum NumberToken {
    Number(u8),
    Other,
}

const spelled_out_numbers: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];


// Tokenize with spelled out or numeric numbers, even without whitespace
fn tokenize(input: String) -> Vec<NumberToken> {
    input
}

pub fn parse_calibration_document(input: String) -> u64 {
    input.lines().map(|line| {
        let num1 = line.find(char::is_numeric).unwrap();
        let num2 = line.rfind(char::is_numeric).unwrap();
        let bytes = line.as_bytes();
        let char1: char = bytes[num1].into();
        let char2: char = bytes[num2].into();
        println!("num1: {}, num2: {}, orig: {}", char1, char2, line);
        let result: u64 = format!("{}{}", char1, char2).parse().unwrap();
        // println!("{:?}", result);
        return result;
    }).sum()
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
    // println!("{}", parse_calibration_document(s));
    assert_eq!(parse_calibration_document(s), 0u64);
}
