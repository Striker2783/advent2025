use std::{fs, path::Path};

pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let v = parse(&s);
    println!("{}", solve(&v));
    println!("{}", solve2(&v));
}

fn solve(s: &[i32]) -> u32 {
    let mut count = 0;
    let mut curr = 50;
    for &n in s {
        curr = (curr + n).rem_euclid(100);
        if curr == 0 {
            count += 1;
        }
    }
    count
}

fn solve2(s: &[i32]) -> u32 {
    let mut count = 0u32;
    let mut curr = 50;
    for &n in s {
        let new = curr + n;
        if new >= 100 {
            count += new as u32 / 100;
        } else if new <= 0 {
            count += (new / 100).unsigned_abs() + if curr != 0 { 1 } else { 0 };
        }
        curr = (curr + n).rem_euclid(100);
    }
    count
}

fn parse(s: &str) -> Vec<i32> {
    s.lines()
        .map(|l| {
            let mut bytes = l.bytes();
            let sign = match bytes.next().unwrap() {
                b'R' => 1,
                b'L' => -1,
                _ => unreachable!(),
            };
            let mut n = 0;
            for d in bytes {
                n *= 10;
                n += (d - b'0') as i32;
            }
            sign * n
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::one::{parse, solve, solve2};

    const TEST1: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    #[test]
    fn test_solve() {
        let s = parse(TEST1);
        assert_eq!(solve(&s), 3);
        assert_eq!(solve2(&s), 6);
    }
}
