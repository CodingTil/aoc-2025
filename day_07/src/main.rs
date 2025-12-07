use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let (start, splitters) = parse_input(INPUT);
    let (part_1, part_2) = calculate(start, &splitters);
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn parse_input(input: &str) -> (usize, HashMap<usize, Vec<usize>>) {
    let (frist_line, remaining_lines) = input.trim().split_once('\n').unwrap();
    let start = frist_line.find('S').unwrap();
    let mut graph = HashMap::new();
    for (i, line) in remaining_lines.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '^' {
                graph.entry(i + 1).or_insert(vec![]).push(j);
            }
        }
    }
    (start, graph)
}

fn calculate(start: usize, splitters: &HashMap<usize, Vec<usize>>) -> (usize, usize) {
    let mut current_beams = HashMap::from([(start, 1)]);
    let mut count_splits = 0;

    for current_row in 1..splitters.keys().max().unwrap() + 1 {
        let mut next_beams = HashMap::new();
        for (beam, count) in &current_beams {
            if splitters.contains_key(&current_row) && splitters[&current_row].contains(beam) {
                count_splits += 1;
                next_beams
                    .entry(beam + 1)
                    .and_modify(|c| *c += count)
                    .or_insert(*count);
                next_beams
                    .entry(beam - 1)
                    .and_modify(|c| *c += count)
                    .or_insert(*count);
            } else {
                next_beams
                    .entry(*beam)
                    .and_modify(|c| *c += count)
                    .or_insert(*count);
            }
        }
        current_beams = next_beams;
    }

    (count_splits, current_beams.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(INPUT_TEST),
            (
                7,
                HashMap::from([
                    (2, vec![7]),
                    (4, vec![6, 8]),
                    (6, vec![5, 7, 9]),
                    (8, vec![4, 6, 10]),
                    (10, vec![3, 5, 9, 11]),
                    (12, vec![2, 6, 12]),
                    (14, vec![1, 3, 5, 7, 9, 13])
                ])
            )
        )
    }

    #[test]
    fn test_simple() {
        let (start, splitters) = parse_input(INPUT_TEST);
        let (part_1, part_2) = calculate(start, &splitters);
        assert_eq!(part_1, 21);
        assert_eq!(part_2, 40);
    }

    #[test]
    fn test_final() {
        let (star, splitters) = parse_input(INPUT);
        let (part_1, part_2) = calculate(star, &splitters);
        assert_eq!(part_1, 1573);
        assert_eq!(part_2, 15093663987272);
    }
}
