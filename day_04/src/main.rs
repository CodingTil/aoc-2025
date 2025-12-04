use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

fn parse_input(input: &str) -> HashSet<(usize, usize)> {
    let mut set = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                set.insert((x, y));
            }
        }
    }
    set
}

fn part_1(input: &str) -> usize {
    let mut set = parse_input(input);

    let count_before = set.len();
    take_removable_rolls(&mut set);
    let count_after = set.len();
    count_before - count_after
}

fn part_2(input: &str) -> usize {
    let mut set = parse_input(input);

    let count_before = set.len();
    while take_removable_rolls(&mut set) {
        continue;
    }
    let count_after = set.len();
    count_before - count_after
}

fn take_removable_rolls(set: &mut HashSet<(usize, usize)>) -> bool {
    let mut removable_rolls = Vec::new();
    for &(x, y) in set.iter() {
        let mut adjecent_count = 0;
        for dx in -1isize..=1 {
            for dy in -1isize..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if dx == -1 && x == 0 {
                    continue;
                }
                if dy == -1 && y == 0 {
                    continue;
                }
                let (new_x, new_y) = (x as isize + dx, y as isize + dy);
                if set.contains(&(new_x as usize, new_y as usize)) {
                    adjecent_count += 1;
                }
            }
        }
        if adjecent_count < 4 {
            removable_rolls.push((x, y));
        }
    }
    for roll in &removable_rolls {
        set.remove(roll);
    }
    !removable_rolls.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_simple() {
        let input = "
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
            ";
        assert_eq!(part_1(input), 13);
    }

    #[test]
    fn test_part_1_complex() {
        assert_eq!(part_1(INPUT), 1549);
    }

    #[test]
    fn test_part_2_simple() {
        let input = "
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
            ";
        assert_eq!(part_2(input), 43);
    }

    #[test]
    fn test_part_2_complex() {
        assert_eq!(part_2(INPUT), 8887);
    }
}
