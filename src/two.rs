use std::{fs, ops::RangeInclusive, path::Path};

pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let ranges = parse(&s);
    println!("{}", solve1(&ranges));
    println!("{}", solve2(&ranges));
}

fn is_repeated(n: u64) -> bool {
    let mut i = 0;
    while 10u64.pow(i) < n {
        i += 1;
    }
    if i % 2 == 1 {
        return false;
    }
    n % 10u64.pow(i / 2) == n / 10u64.pow(i / 2)
}

fn is_repeated2(n: u64) -> bool {
    let str = n.to_string();
    'outer: for l in (1..).take_while(|&e| e <= str.len() / 2) {
        let slice = &str[0..l];
        if str.len() % l != 0 {
            continue;
        }
        for j in (0..(str.len())).step_by(l) {
            if &str[j..(j + l)] != slice {
                continue 'outer;
            }
        }
        return true;
    }
    false
}

fn solve1(v: &[RangeInclusive<u64>]) -> u64 {
    let mut sum = 0;
    for r in v.iter().cloned() {
        for n in r {
            if is_repeated(n) {
                sum += n;
            }
        }
    }
    sum
}

fn solve2(v: &[RangeInclusive<u64>]) -> u64 {
    let mut sum = 0;
    for r in v.iter().cloned() {
        for n in r {
            if is_repeated2(n) {
                sum += n;
            }
        }
    }
    sum
}

fn parse(s: &str) -> Vec<RangeInclusive<u64>> {
    s.split(',')
        .map(|ids| {
            let mut split = ids.split('-');
            let a1 = split.next().unwrap().parse().unwrap();
            let a2 = split.next().unwrap().parse().unwrap();
            a1..=a2
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::two::{is_repeated, parse, solve1, solve2};

    const TEST: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    #[test]
    fn test_solve() {
        assert!(is_repeated(123123));
        let v = parse(TEST);
        assert_eq!(solve1(&v), 1227775554);
        assert_eq!(solve2(&v), 4174379265);
    }
}
