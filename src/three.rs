use std::{fs, path::Path};

pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let v = parse(&s);
    println!("{}", solve1(&v));
}

fn parse(s: &str) -> Vec<Vec<u8>> {
    s.lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect()
}

fn solve1(v: &[Vec<u8>]) -> u64 {
    let mut sum = 0;
    for v in v {
        let mut max = 0;
        for (i, &n1) in v.iter().enumerate() {
            for &n2 in v.iter().skip(i + 1) {
                max = max.max(n1 as u64 * 10 + n2 as u64);
            }
        }
        sum += max;
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::three::{parse, solve1};

    const TEST: &str = "987654321111111
811111111111119
234234234234278
818181911112111
";
    #[test]
    fn test_solve() {
        let v = parse(TEST);
        assert_eq!(solve1(&v), 357);
    }
}
