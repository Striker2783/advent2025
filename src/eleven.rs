use std::{collections::HashMap, fs, path::Path};

pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let p = parse(&s);
    println!("{}", solve1(&p));
    println!("{}", solve2(&p));
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
    end: &str,
) -> u64 {
    if curr == end {
        return 1;
    }
    if let Some(&c) = counts.get(curr) {
        return c;
    }
    if let Some(v) = map.get(curr) {
        let count: u64 = v.iter().map(|s| recursive1(map, counts, s, end)).sum();
        counts.insert(curr.to_string(), count);
        return count;
    }
    0
}

fn solve1(map: &HashMap<String, Vec<String>>) -> u64 {
    let mut counts = HashMap::new();
    recursive1(map, &mut counts, "you", "out")
}

fn solve2(map: &HashMap<String, Vec<String>>) -> u64 {
    let a = recursive1(map, &mut HashMap::new(), "svr", "dac");
    let b = recursive1(map, &mut HashMap::new(), "dac", "fft");
    let c = recursive1(map, &mut HashMap::new(), "fft", "out");
    let a2 = recursive1(map, &mut HashMap::new(), "svr", "fft");
    let b2 = recursive1(map, &mut HashMap::new(), "fft", "dac");
    let c2 = recursive1(map, &mut HashMap::new(), "dac", "out");
    a * b * c + a2 * b2 * c2
}

#[cfg(test)]
mod tests {
    use crate::eleven::{parse, solve1, solve2};

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

    const TEST2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";
    #[test]
    fn test_solve() {
        let map = parse(TEST);
        assert_eq!(solve1(&map), 5);
        let map = parse(TEST2);
        assert_eq!(solve2(&map), 2);
    }
}
