use std::{
    collections::HashSet,
    ops::{Add, Mul},
};

use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let coordinates = parse_input(INPUT);
    println!("Part 1: {}", part_1(&coordinates));

    let boundary = get_boundary(&coordinates);
    println!("Part 2: {}", part_2(&coordinates, &boundary));
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts = line.split_terminator(',').collect::<Vec<&str>>();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect()
}

fn part_1(coordinates: &[(usize, usize)]) -> usize {
    coordinates
        .iter()
        .enumerate()
        .flat_map(|(i, (x1, y1))| {
            coordinates[i + 1..]
                .iter()
                .map(move |(x2, y2)| x1.abs_diff(*x2).add(1).mul(y1.abs_diff(*y2).add(1)))
        })
        .max()
        .unwrap_or(0)
}

fn get_boundary(coordinates: &[(usize, usize)]) -> HashSet<(isize, isize)> {
    // green theorem. but we don't yet know if we cycle clockwise or counterclockwise
    let mut boundary_clockwise: HashSet<(isize, isize)> = HashSet::new();
    let mut boundary_counterclockwise: HashSet<(isize, isize)> = HashSet::new();

    for i in 0..coordinates.len() {
        let (x1, y1) = coordinates[i];
        let (x2, y2) = coordinates[(i + 1) % coordinates.len()];

        let (x1, y1) = (x1 as isize, y1 as isize);
        let (x2, y2) = (x2 as isize, y2 as isize);

        let dx: isize = x2 - x1;
        let dy: isize = y2 - y1;

        assert!((dx == 0 || dy == 0) && !(dx == 0 && dy == 0));
        if dy == 0 {
            for x in x1..=x2 {
                boundary_clockwise.insert((x, y1 - 1));
                boundary_counterclockwise.insert((x, y1 + 1));
            }
            for x in x2..=x1 {
                boundary_clockwise.insert((x, y1 + 1));
                boundary_counterclockwise.insert((x, y1 - 1));
            }
        } else {
            for y in y1..=y2 {
                boundary_clockwise.insert((x1 + 1, y));
                boundary_counterclockwise.insert((x1 - 1, y));
            }
            for y in y2..=y1 {
                boundary_clockwise.insert((x1 - 1, y));
                boundary_counterclockwise.insert((x1 + 1, y));
            }
        }
    }
    for i in 0..coordinates.len() {
        let (x1, y1) = coordinates[i];
        let (x2, y2) = coordinates[(i + 1) % coordinates.len()];

        let (x1, y1) = (x1 as isize, y1 as isize);
        let (x2, y2) = (x2 as isize, y2 as isize);

        let x_min = x1.min(x2);
        let x_max = x1.max(x2);
        let y_min = y1.min(y2);
        let y_max = y1.max(y2);

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                boundary_clockwise.remove(&(x, y));
                boundary_counterclockwise.remove(&(x, y));
            }
        }
    }
    assert_ne!(boundary_clockwise.len(), boundary_counterclockwise.len());
    if boundary_clockwise.len() > boundary_counterclockwise.len() {
        boundary_clockwise
    } else {
        boundary_counterclockwise
    }
}

fn part_2(coordinates: &[(usize, usize)], boundary: &HashSet<(isize, isize)>) -> usize {
    coordinates
        .iter()
        .tuple_combinations()
        .par_bridge()
        .filter(|((x1, y1), (x2, y2))| {
            let x_min = *x1.min(x2) as isize;
            let x_max = *x1.max(x2) as isize;
            let y_min = *y1.min(y2) as isize;
            let y_max = *y1.max(y2) as isize;

            for x in x_min..=x_max {
                if boundary.contains(&(x, y_min)) || boundary.contains(&(x, y_max)) {
                    return false;
                }
            }
            for y in y_min..=y_max {
                if boundary.contains(&(x_min, y)) || boundary.contains(&(x_max, y)) {
                    return false;
                }
            }
            true
        })
        .map(|((x1, y1), (x2, y2))| x1.abs_diff(*x2).add(1).mul(y1.abs_diff(*y2).add(1)))
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
    7,1
    11,1
    11,7
    9,7
    9,5
    2,5
    2,3
    7,3
    ";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(TEST_INPUT),
            vec![
                (7, 1),
                (11, 1),
                (11, 7),
                (9, 7),
                (9, 5),
                (2, 5),
                (2, 3),
                (7, 3)
            ]
        );
    }

    #[test]
    fn test_part_1_simple() {
        assert_eq!(part_1(&parse_input(TEST_INPUT)), 50);
    }

    #[test]
    fn test_part_1_final() {
        assert_eq!(part_1(&parse_input(INPUT)), 4763040296);
    }

    #[test]
    fn test_part_2_simple() {
        assert_eq!(
            part_2(
                &parse_input(TEST_INPUT),
                &get_boundary(&parse_input(TEST_INPUT))
            ),
            24
        );
    }

    #[test]
    fn test_part_2_final() {
        assert_eq!(
            part_2(&parse_input(INPUT), &get_boundary(&parse_input(INPUT))),
            1396494456
        );
    }
}
