fn main() {
    let input = include_str!("./input.txt");
    let mut sum: u64 = 0;
    for line in input.lines() {
        let first = line.chars().find(|c| c.is_ascii_digit()).expect("first digit not found");
        let last = line.chars().rev().find(|c| c.is_ascii_digit()).expect("last digit not found");
        let first = first.to_digit(10).unwrap();
        let last = last.to_digit(10).unwrap();
        sum += u64::from(first) * 10 + u64::from(last);
    }
    println!("{sum}");
}
