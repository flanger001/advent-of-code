use {
    once_cell::sync::Lazy,
    regex::{Regex, RegexSet},
};

static PATTERNS: [&str; 10] = [
    r"\d", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
static RS: Lazy<regex::RegexSet> = Lazy::new(|| RegexSet::new(PATTERNS).expect("should work"));
static REGEXES: Lazy<Vec<regex::Regex>> = Lazy::new(|| {
    RS.patterns()
        .iter()
        .map(|p| Regex::new(p).expect("bad regex"))
        .collect()
});

pub fn parse_digit(digit_str: &str) -> usize {
    match digit_str.parse() {
        Ok(digit) => digit,
        Err(_) => match digit_str {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => 0,
        },
    }
}

pub fn parse_line(line: &str) -> usize {
    if line.is_empty() {
        return 0;
    }

    let mut matches: Vec<regex::Match> = vec![];
    for idx in RS.matches(line) {
        let re = &REGEXES[idx];
        for m in re.find_iter(line) {
            matches.push(m)
        }
    }

    matches.sort_by_key(|a: &regex::Match| a.start());

    let first = parse_digit(&matches.first().expect("should exist").as_str());
    let last = parse_digit(&matches.last().expect("should exist").as_str());

    let value = (first * 10) + last;

    value
}

pub fn parse_document(document: &str) -> usize {
    let mut count = 0;
    for line in document.split("\n") {
        count += parse_line(line);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_digit() {
        assert_eq!(parse_digit("1"), 1);
        assert_eq!(parse_digit("12"), 12);
        assert_eq!(parse_digit("one"), 1);
        assert_eq!(parse_digit("two"), 2);
        assert_eq!(parse_digit("three"), 3);
        assert_eq!(parse_digit("four"), 4);
        assert_eq!(parse_digit("five"), 5);
        assert_eq!(parse_digit("six"), 6);
        assert_eq!(parse_digit("seven"), 7);
        assert_eq!(parse_digit("eight"), 8);
        assert_eq!(parse_digit("nine"), 9);
        assert_eq!(parse_digit("goo"), 0);
    }

    #[test]
    fn test_parse_line() {
        let r = parse_line("1abc2");
        assert_eq!(r, 12);
        let r = parse_line("pqr3stu8vwx");
        assert_eq!(r, 38);
        let r = parse_line("a1b2c3d4e5f");
        assert_eq!(r, 15);
        let r = parse_line("treb7uchet");
        assert_eq!(r, 77);
        let r = parse_line("oneacnekljsadgf3sadgsadgd");
        assert_eq!(r, 13);
        let r = parse_line("oneacnekljsadgf3sadgsafourdgd");
        assert_eq!(r, 14);
    }

    #[test]
    fn test_document_1() {
        let document = r"1abc2\n\
        pqr3stu8vwx\n\
        a1b2c3d4e5f\n\
        treb7uchet";
        let r = parse_document(document);
        assert_eq!(r, 142);
    }

    #[test]
    fn test_aoc01_01_02() {
        let document = fs::read_to_string("./src/input.txt").expect("file doesn't exist");
        let r = parse_document(&document);
        println!("{:?}", r);
        assert_eq!(r, 54518);
    }
}
