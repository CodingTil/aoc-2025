const INPUT: &str = include_str!("../input.txt");

fn main() {
    let (ranges, numbers) = parse_input(INPUT);
    println!("Part 1: {}", part_1(&ranges, &numbers));
    println!("Part 2: {}", part_2(&ranges));
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let mut ranges = Vec::new();
    let mut numbers = Vec::new();

    for line in input.lines().map(|l| l.trim()) {
        if line.is_empty() {
            continue;
        }

        if let Some((start, end)) = line.split_once('-') {
            if let (Ok(start), Ok(end)) = (start.parse::<usize>(), end.parse::<usize>()) {
                ranges.push((start, end));
            } else {
                println!("{}-{}", start, end);
            }
        } else if let Ok(number) = line.parse::<usize>() {
            numbers.push(number);
        }
    }

    (ranges, numbers)
}

fn part_1(ranges: &[(usize, usize)], numbers: &[usize]) -> usize {
    let mut count = 0;
    for number in numbers {
        for (start, end) in ranges {
            if start <= number && number <= end {
                count += 1;
                break;
            }
        }
    }
    count
}

fn part_2(ranges: &[(usize, usize)]) -> usize {
    // Order Ranges by start, and then by end
    let mut ranges = ranges.to_vec();
    ranges.sort();

    // If two ranges overlap, merge them
    let mut merged_ranges = Vec::new();
    let (mut current_start, mut current_end) = ranges[0];
    for (start, end) in &ranges[1..] {
        if *start <= current_end {
            current_end = current_end.max(*end);
        } else {
            merged_ranges.push((current_start, current_end));
            current_start = *start;
            current_end = *end;
        }
    }
    merged_ranges.push((current_start, current_end));

    merged_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
    3-5
    10-14
    16-20
    12-18
    
    1
    5
    8
    11
    17
    32
    ";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(TEST_INPUT),
            (
                vec![(3, 5), (10, 14), (16, 20), (12, 18)],
                vec![1, 5, 8, 11, 17, 32]
            )
        );
    }

    #[test]
    fn test_part_1_simple() {
        let (ranges, numbers) = parse_input(TEST_INPUT);
        assert_eq!(part_1(&ranges, &numbers), 3);
    }

    #[test]
    fn test_part_1_final() {
        let (ranges, numbers) = parse_input(INPUT);
        assert_eq!(part_1(&ranges, &numbers), 617);
    }

    #[test]
    fn test_part_2_simple() {
        let (ranges, _) = parse_input(TEST_INPUT);
        assert_eq!(part_2(&ranges), 14);
    }

    #[test]
    fn test_part_2_final() {
        let (ranges, _) = parse_input(INPUT);
        assert_eq!(part_2(&ranges), 338258295736104);
    }
}
