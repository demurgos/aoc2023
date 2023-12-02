use std::str::FromStr;
use regex::Regex;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Sample {
    r: u64,
    g: u64,
    b: u64,
}

impl Sample {
    pub fn update(&mut self, other: &Sample) {
        self.r = u64::max(self.r, other.r);
        self.g = u64::max(self.g, other.g);
        self.b = u64::max(self.b, other.b);
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let mut sum: u64 = 0;
    let number = Regex::new(r#"\d+"#).unwrap();

    for line in input.lines() {
        let (game_id, samples) = line.split_once(':').unwrap();
        let game_id = u64::from_str(number.find(game_id).unwrap().as_str()).unwrap();
        let mut max = Sample::default();
        let max = &mut max;
        for sample in samples.split(';') {
            let mut s = Sample::default();
            for cube_count in sample.split(',') {
                let num = u64::from_str(number.find(cube_count).unwrap().as_str()).unwrap();
                if cube_count.contains("red") {
                    s.r = num;
                }else if cube_count.contains("green") {
                    s.g = num;
                } else if cube_count.contains("blue") {
                    s.b = num;
                } else {
                    panic!("no color");
                }
            }
            max.update(&s);
        }
        if max.r <= 12 && max.g <= 13 && max.b <= 14 {
            sum += game_id;
        }
    }
    println!("{sum}");
}
