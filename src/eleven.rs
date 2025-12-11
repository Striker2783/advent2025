use std::{
    collections::HashMap,
    fs,
    path::Path,
};

pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let p = parse(&s);
    println!("{}", solve1(&p));
}

fn parse(s: &str) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for l in s.lines() {
        let mut split = l.split(": ");
        let key = split.next().unwrap().to_string();
        let v: Vec<String> = split
            .next()
            .unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();
        map.insert(key, v);
    }
    map
}

fn recursive1(
    map: &HashMap<String, Vec<String>>,
    counts: &mut HashMap<String, u64>,
    curr: &str,
) -> u64 {
    if curr == "out" {
        return 1;
    }
    if let Some(v) = map.get(curr) {
        let count: u64 = v.iter().map(|s| recursive1(map, counts, s)).sum();
        counts.insert(curr.to_string(), count);
        return count;
    }
    0
}

fn solve1(map: &HashMap<String, Vec<String>>) -> u64 {
    let mut counts = HashMap::new();
    recursive1(map, &mut counts, "you")
}

#[cfg(test)]
mod tests {
    use crate::eleven::{parse, solve1};

    const TEST: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";
    #[test]
    fn test_solve() {
        let map = parse(TEST);
        assert_eq!(solve1(&map), 5);
    }
}
