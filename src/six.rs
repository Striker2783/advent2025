use std::{fs, path::Path};

pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let p = parse(&s);
    println!("{}", solve1(&p.0, &p.1));
    let p2 = parse2(&s);
    println!("{}", solve2(&p2.0, &p2.1));
}

fn solve1(p: &[Vec<u64>], o: &[Signs]) -> u64 {
    let mut sum = 0;
    for i in 0..p[0].len() {
        let operator = o[i];
        let mut add = operator.default_value();
        for j in 0..p.len() {
            match operator {
                Signs::Mult => add *= p[j][i],
                Signs::Plus => add += p[j][i],
            }
        }
        sum += add;
    }
    sum
}
fn solve2(p: &[Vec<u64>], o: &[Signs]) -> u64 {
    let mut sum = 0;
    for i in 0..p.len() {
        let operator = o[i];
        let mut add = operator.default_value();
        for j in 0..p[i].len() {
            match operator {
                Signs::Mult => add *= p[i][j],
                Signs::Plus => add += p[i][j],
            }
        }
        sum += add;
    }
    sum
}

fn parse2(s: &str) -> (Vec<Vec<u64>>, Vec<Signs>) {
    let mut lines = s.lines();
    let operators = lines.next_back().unwrap();
    let numbers: Vec<_> = lines.collect();
    let mut nums = Vec::new();
    let mut ops = Vec::new();
    for column in 0..operators.as_bytes().len() {
        if operators.as_bytes()[column] != b' ' {
            ops.push(Signs::new(operators.as_bytes()[column]));
            nums.push(Vec::new());
        }
        let v = nums.last_mut().unwrap();
        let mut n = 0;
        for row in 0..numbers.len() {
            if numbers[row].as_bytes()[column] == b' ' {
                continue;
            }
            n *= 10;
            n += (numbers[row].as_bytes()[column] - b'0') as u64;
        }
        if n != 0 {
            v.push(n);
        }
    }
    (nums, ops)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Signs {
    Mult,
    Plus,
}
impl Signs {
    pub fn default_value(self) -> u64 {
        match self {
            Signs::Mult => 1,
            Signs::Plus => 0,
        }
    }
    pub fn new(b: u8) -> Self {
        match b {
            b'+' => Signs::Plus,
            b'*' => Signs::Mult,
            _ => unreachable!(),
        }
    }
}

fn parse(s: &str) -> (Vec<Vec<u64>>, Vec<Signs>) {
    let mut split = s.lines();
    let operators = split
        .next_back()
        .unwrap()
        .split_whitespace()
        .map(|s| match s {
            "+" => Signs::Plus,
            "*" => Signs::Mult,
            _ => unreachable!(),
        })
        .collect();
    let numbers = split
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect();
    (numbers, operators)
}

#[cfg(test)]
mod tests {
    use crate::six::{parse, parse2, solve1, solve2};

    const TEST: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";
    #[test]
    fn test_solve() {
        let p = parse(TEST);
        assert_eq!(solve1(&p.0, &p.1), 4277556);
        let p2 = parse2(TEST);
        assert_eq!(solve2(&p2.0, &p2.1), 3263827);
    }
}
