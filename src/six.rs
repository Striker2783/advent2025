use std::{fs, path::Path};

pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let p = parse(&s);
    println!("{}", solve(&p.0, &p.1))
}

fn solve(p: &[Vec<u64>], o: &[Signs]) -> u64 {
    let mut sum = 0;
    for i in 0..p[0].len() {
        let operator = o[i];
        let mut add = match operator {
            Signs::Mult => 1,
            Signs::Plus => 0,
        };
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Signs {
    Mult,
    Plus,
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
    use crate::six::{parse, solve};

    const TEST: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";
    #[test]
    fn test_solve() {
        let p = parse(TEST);
        assert_eq!(solve(&p.0, &p.1), 4277556);
    }
}
