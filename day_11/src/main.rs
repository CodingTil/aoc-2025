use std::collections::{BTreeMap, HashMap};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let edges = parse_input(INPUT);
    println!("Part 1: {}", part_1(&edges));
    println!("Part 2: {}", part_2(&edges));
}

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (from, to): (&str, &str) = line.split_once(":").unwrap();
            (
                from.to_string(),
                to.trim().split(" ").map(|s| s.to_string()).collect(),
            )
        })
        .collect()
}

fn count_all_paths_between(
    edges: &HashMap<String, Vec<String>>,
    start: &str,
    end: &str,
    must_visit: &[&str],
) -> usize {
    dfs(
        edges,
        start,
        end,
        must_visit.iter().map(|&s| (s.to_string(), false)).collect(),
        &mut HashMap::new(),
    )
}

fn dfs(
    edges: &HashMap<String, Vec<String>>,
    current: &str,
    end: &str,
    mut visited: BTreeMap<String, bool>,
    cache: &mut HashMap<(String, String, BTreeMap<String, bool>), usize>,
) -> usize {
    if let Some(&cached_count) = cache.get(&(current.to_string(), end.to_string(), visited.clone()))
    {
        return cached_count;
    }

    if current == end {
        if visited.values().all(|&v| v) {
            return 1;
        }
        return 0;
    }

    visited
        .entry(current.to_string())
        .and_modify(|entry| *entry = true);

    let mut paths_count = 0;

    if let Some(neighbors) = edges.get(current) {
        for neighbor in neighbors {
            let sub_paths_count = dfs(edges, neighbor, end, visited.clone(), cache);
            paths_count += sub_paths_count;
        }
    }

    cache.insert((current.to_string(), end.to_string(), visited), paths_count);

    paths_count
}

fn part_1(edges: &HashMap<String, Vec<String>>) -> usize {
    count_all_paths_between(edges, "you", "out", &[])
}

fn part_2(edges: &HashMap<String, Vec<String>>) -> usize {
    count_all_paths_between(edges, "svr", "out", &["dac", "fft"])
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_PART_1: &str = "
    aaa: you hhh
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

    const TEST_INPUT_PART_2: &str = "
    svr: aaa bbb
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
    fn test_parse_input() {
        assert_eq!(
            parse_input(TEST_INPUT_PART_1),
            HashMap::from([
                (
                    "aaa".to_string(),
                    vec!["you".to_string(), "hhh".to_string()]
                ),
                (
                    "you".to_string(),
                    vec!["bbb".to_string(), "ccc".to_string()]
                ),
                (
                    "bbb".to_string(),
                    vec!["ddd".to_string(), "eee".to_string()]
                ),
                (
                    "ccc".to_string(),
                    vec!["ddd".to_string(), "eee".to_string(), "fff".to_string()]
                ),
                ("ddd".to_string(), vec!["ggg".to_string()]),
                ("eee".to_string(), vec!["out".to_string()]),
                ("fff".to_string(), vec!["out".to_string()]),
                ("ggg".to_string(), vec!["out".to_string()]),
                (
                    "hhh".to_string(),
                    vec!["ccc".to_string(), "fff".to_string(), "iii".to_string()]
                ),
                ("iii".to_string(), vec!["out".to_string()]),
            ])
        );
    }

    #[test]
    fn test_part_1_simple() {
        let edges = parse_input(TEST_INPUT_PART_1);
        assert_eq!(part_1(&edges), 5);
    }

    #[test]
    fn test_part_1_final() {
        let edges = parse_input(INPUT);
        assert_eq!(part_1(&edges), 662);
    }

    #[test]
    fn test_part_2_simple() {
        let edges = parse_input(TEST_INPUT_PART_2);
        assert_eq!(part_2(&edges), 2);
    }

    #[test]
    fn test_part_2_final() {
        let edges = parse_input(INPUT);
        assert_eq!(part_2(&edges), 429399933071120);
    }
}
