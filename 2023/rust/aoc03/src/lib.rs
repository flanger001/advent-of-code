use once_cell::sync::Lazy;
use regex::Regex;

static NUMBERS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d*)").unwrap());

pub struct Point {
    x: usize,
    y: usize,
}

pub fn get_numbers(line: &str) -> Vec<Point> {
    let results = vec![];
    for n_cap in NUMBERS_REGEX.captures_iter(line) {
        println!("{:?}", n_cap);
        // let extracted = &n_cap.extract();
        // println!("{:?}", extracted);
    }
    results
}
pub fn parse_doc() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_doc() {
        let input = r"467..114..\n\
            ...*......\n\
            ..35..633.\n\
            ......#...\n\
            617*......\n\
            .....+.58.\n\
            ..592.....\n\
            ......755.\n\
            ...$.*....\n\
            .664.598..";
        for line in input.split("\n") {
            if !line.is_empty() {
                get_numbers(line.trim());
            }
        }
    }

    fn test_aoc03_01() {}
}
