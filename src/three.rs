use std::{fs, path::Path};

pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let v = parse(&s);
    println!("{}", solve1(&v));
    println!("{}", solve2(&v));
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

fn solve2(v: &[Vec<u8>]) -> u64 {
    let mut sum = 0;
    for v in v {
        let inner = idk(v, 12, 0);
        sum += inner;
    }
    sum
}

fn idk(v: &[u8], digs: usize, curr: u64) -> u64 {
    if digs == 0 {
        return curr;
    }
    let mut i = 0;
    let mut max = 0;
    let mut max_i = 0;
    while i <= v.len() - digs {
        if v[i] > max {
            max = v[i];
            max_i = i;
        }
        i += 1;
    }
    idk(&v[(max_i + 1)..], digs - 1, curr * 10 + max as u64)
}

#[cfg(test)]
mod tests {
    use crate::three::{parse, solve1, solve2};

    const TEST: &str = "987654321111111
811111111111119
234234234234278
818181911112111
";
    #[test]
    fn test_solve() {
        let v = parse(TEST);
        assert_eq!(solve1(&v), 357);
        assert_eq!(solve2(&v), 3121910778619u64);
    }
}
