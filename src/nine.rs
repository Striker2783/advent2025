use std::{fs, path::Path};

pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let p = parse(&s);
    println!("{}", solve1(&p));
    println!("{}", solve2(&p));
}

fn solve1(v: &[(u64, u64)]) -> u64 {
    let mut max = 0;
    for (i, p1) in v.iter().enumerate() {
        for p2 in v.iter().skip(i + 1) {
            let area = (p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1);
            max = max.max(area);
        }
    }
    max
}

fn contains(v: &[(u64, u64)], c1: (u64, u64), c2: (u64, u64)) -> bool {
    let min_p = (c1.0.min(c2.0), c1.1.min(c2.1));
    let max_p = (c1.0.max(c2.0), c1.1.max(c2.1));
    for (&p1, &p2) in v.iter().zip(v.iter().skip(1).cycle()) {
        let within =
            |p: (u64, u64)| p.0 > min_p.0 && p.0 < max_p.0 && p.1 > min_p.1 && p.1 < max_p.1;
        if within(p1) || within(p2) {
            return false;
        }
        let mid = ((p1.0 + p2.0) / 2, (p1.1 + p2.1) / 2);
        if within(mid) {
            return false;
        }
    }
    true
}

fn solve2(v: &[(u64, u64)]) -> u64 {
    let mut max = 0;
    for (i, &p1) in v.iter().enumerate() {
        for &p2 in v.iter().skip(i + 1) {
            if !contains(v, p1, p2) {
                continue;
            }
            let area = (p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1);
            max = max.max(area);
        }
    }
    max
}

fn parse(s: &str) -> Vec<(u64, u64)> {
    s.lines()
        .map(|l| {
            let mut split = l.split(',');
            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::nine::{parse, solve1, solve2};

    const TEST: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";
    #[test]
    fn test_solve() {
        let p = parse(TEST);
        assert_eq!(solve1(&p), 50);
        assert_eq!(solve2(&p), 24);
    }
}
