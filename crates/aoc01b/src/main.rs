use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let re = Regex::new(r#"\d|zero|one|two|three|four|five|six|seven|eight|nine"#).unwrap();
    let re_rev = Regex::new(r#"\d|orez|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin"#).unwrap();

    let mut sum: u64 = 0;
    for line in input.lines() {
        let first = re.find(line).expect("first not found");
        let rev_line = String::from_iter(line.chars().rev());
        let last = re_rev.find(&rev_line).expect("last not found");

        let first = first.as_str();
        let last = String::from_iter(last.as_str().chars().rev());
        let last = &last;
        sum += to_digit(first) * 10 + to_digit(last);
    }
    println!("{sum}");
}

fn to_digit(s: &str) -> u64 {
    match s {
        "0" | "zero" => 0,
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => panic!("not a digit"),
    }
}
