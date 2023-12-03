use std::collections::{HashMap, HashSet};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Num {
    val: u64,
    row: u64,
    col_start: u64,
    col_end: u64,
}

impl Num {
    pub const fn new(row: u64) -> Self {
        Self {
            val: 0,
            row,
            col_start: u64::MAX,
            col_end: 0,
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let mut sum: u64 = 0;

    let mut nums: Vec<Num> = Vec::new();
    let mut gears: HashMap<(u64, u64), HashSet<Num>> = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        let row = u64::try_from(row).unwrap();
        let mut cur: Num = Num::new(row);
        for (col, char) in line.char_indices() {
            let col = u64::try_from(col).unwrap();
            if let Some(d) = char.to_digit(10) {
                cur.val = cur.val * 10 + u64::from(d);
                cur.col_start = u64::min(cur.col_start, col);
                cur.col_end = u64::max(cur.col_end, col + 1);
            } else {
                if cur.col_start != u64::MAX {
                    nums.push(cur);
                }
                cur = Num::new(row);
                if char == '*' {
                    gears.insert((row, col), HashSet::new());
                }
            }
        }
        if cur.col_start != u64::MAX {
            nums.push(cur);
        }
    }

    'nums:
    for n in nums {
        let row_min = n.row.saturating_sub(1);
        let row_max = n.row.saturating_add(2);
        let col_min = n.col_start.saturating_sub(1);
        let col_max = n.col_end.saturating_add(1);
        for r in row_min..row_max {
            for c in col_min..col_max {
                if let Some(s) = gears.get_mut(&(r, c)) {
                    s.insert(n);
                    continue 'nums;
                }
            }
        }
    }

    for gear in gears.values() {
        if gear.len() == 2 {
            let mut prod: u64  = 1;
            for g in gear {
                prod *= g.val;
            }
            sum += prod;
        }
    }

    println!("{sum}");
}
