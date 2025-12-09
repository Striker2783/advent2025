use std::{fs, path::Path};

pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let p = parse(&s);
    println!("{}", solve1(&p));
    // println!("{}", solve2(&p));
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
    use crate::nine::{parse, solve1};

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
    }
}
