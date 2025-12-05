use std::{fs, ops::RangeInclusive, path::Path};

pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let parsed = parse(&s);
    println!("{}", solve1(&parsed.0, &parsed.1));
    println!("{}", solve2(&parsed.0));
}

fn parse(s: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let mut split = s.split("\n\n");
    let ranges = split.next().unwrap();
    let nums = split.next().unwrap();
    (
        ranges
            .lines()
            .map(|l| {
                let split = l.split_once('-').unwrap();
                split.0.parse().unwrap()..=split.1.parse().unwrap()
            })
            .collect(),
        nums.lines().map(|l| l.parse().unwrap()).collect(),
    )
}

fn solve1(r: &[RangeInclusive<u64>], v: &[u64]) -> u32 {
    let mut count = 0;
    for &n in v {
        for r in r {
            if r.contains(&n) {
                count += 1;
                break;
            }
        }
    }
    count
}

fn solve2(r: &[RangeInclusive<u64>]) -> u64 {
    let mut count = 0;
    let mut r = r.to_vec();
    r.sort_by(|a, b| a.start().cmp(b.start()));
    let mut last = 0;
    for r in r {
        last = last.max(*r.start());
        if *r.end() < last {
            continue;
        }
        count += *r.end() - last + 1;
        last = *r.end() + 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::five::{parse, solve1, solve2};

    const TEST: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    #[test]
    fn test_solve1() {
        let parsed = parse(TEST);
        assert_eq!(solve1(&parsed.0, &parsed.1), 3);
        assert_eq!(solve2(&parsed.0), 14);
    }
}
