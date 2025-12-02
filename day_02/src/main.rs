use std::ops::Div;

const INPUT: &str = include_str!("../input.txt");

/// Parse the input into a vector of ranges.
/// Input example: 12-123,333-444
fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .split(",")
        .map(|r| -> (usize, usize) {
            let mut parts = r.split("-");
            let start = parts.next().unwrap().parse().unwrap();
            let end = parts.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect()
}

fn main() {
    let parsed_input = parse_input(INPUT);

    let part_1_result = part_1(&parsed_input);
    println!("Part 1: {}", part_1_result);

    let part_2_result = part_2(&parsed_input);
    println!("Part 2: {}", part_2_result);
}

fn part_1(input: &Vec<(usize, usize)>) -> usize {
    get_invalid_ids(input, is_valid_id_part_1).iter().sum()
}

/// An ID is invalid if it is made only of some sequence of digits repeated twice.
fn is_valid_id_part_1(id: usize) -> bool {
    let number_of_digits = id.ilog10() + 1;
    if !number_of_digits.is_multiple_of(2) {
        return true;
    }

    let half_point = number_of_digits / 2;

    let first_part = id.div_euclid(10usize.pow(half_point));
    let second_part = id.rem_euclid(10usize.pow(half_point));

    first_part != second_part
}

fn part_2(input: &Vec<(usize, usize)>) -> usize {
    get_invalid_ids(input, is_valid_id_part_2).iter().sum()
}

/// An ID is invalid if it is made only of some sequence of digits repeated at least twice.
fn is_valid_id_part_2(id: usize) -> bool {
    let number_of_digits = id.ilog10() + 1;

    for segment_count in 2..=number_of_digits {
        if !number_of_digits.is_multiple_of(segment_count) {
            continue;
        }

        let segment_length = number_of_digits.div(segment_count);
        let mut segments = Vec::with_capacity(segment_count as usize);
        let mut remainder = id;
        for _ in 0..segment_count {
            segments.push(remainder.rem_euclid(10usize.pow(segment_length)));
            remainder = remainder.div_euclid(10usize.pow(segment_length));
        }

        if segments.iter().all(|&segment| segment == segments[0]) {
            return false;
        }
    }

    true
}

fn get_invalid_ids(input: &Vec<(usize, usize)>, validator: fn(usize) -> bool) -> Vec<usize> {
    let mut invalid_ids = Vec::new();
    for &(start, end) in input {
        for id in start..=end {
            if !validator(id) {
                invalid_ids.push(id);
            }
        }
    }
    invalid_ids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "12-123,333-444";
        let expected = vec![(12, 123), (333, 444)];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_parse_input_with_leading_zero() {
        let input = "011-00123";
        let expected = vec![(11, 123)];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_is_valid_id_part_1() {
        assert_eq!(is_valid_id_part_1(55), false);
        assert_eq!(is_valid_id_part_1(6464), false);
        assert_eq!(is_valid_id_part_1(123123), false);
        assert_eq!(is_valid_id_part_1(101), true);
        assert_eq!(is_valid_id_part_1(12341234), false);
        assert_eq!(is_valid_id_part_1(123123123), true);
        assert_eq!(is_valid_id_part_1(1212121212), true);
        assert_eq!(is_valid_id_part_1(1111111), true);
        assert_eq!(is_valid_id_part_1(3), true);
    }

    #[test]
    fn test_is_valid_id_part_2() {
        assert_eq!(is_valid_id_part_2(55), false);
        assert_eq!(is_valid_id_part_2(6464), false);
        assert_eq!(is_valid_id_part_2(123123), false);
        assert_eq!(is_valid_id_part_2(101), true);
        assert_eq!(is_valid_id_part_2(12341234), false);
        assert_eq!(is_valid_id_part_2(123123123), false);
        assert_eq!(is_valid_id_part_2(1212121212), false);
        assert_eq!(is_valid_id_part_2(1111111), false);
        assert_eq!(is_valid_id_part_2(3), true);
    }

    #[test]
    fn test_get_invalid_ids_part_1() {
        assert_eq!(
            get_invalid_ids(&vec![(11, 22)], is_valid_id_part_1),
            vec![11, 22]
        );
        assert_eq!(
            get_invalid_ids(&vec![(95, 115)], is_valid_id_part_1),
            vec![99]
        );
        assert_eq!(
            get_invalid_ids(&vec![(998, 1012)], is_valid_id_part_1),
            vec![1010]
        );
        assert_eq!(
            get_invalid_ids(&vec![(1188511880, 1188511890)], is_valid_id_part_1),
            vec![1188511885]
        );
        assert_eq!(
            get_invalid_ids(&vec![(222220, 222224)], is_valid_id_part_1),
            vec![222222]
        );
        assert_eq!(
            get_invalid_ids(&vec![(1698522, 1698528)], is_valid_id_part_1),
            vec![]
        );
        assert_eq!(
            get_invalid_ids(&vec![(446443, 446449)], is_valid_id_part_1),
            vec![446446]
        );
        assert_eq!(
            get_invalid_ids(&vec![(38593856, 38593862)], is_valid_id_part_1),
            vec![38593859]
        );
        assert_eq!(
            get_invalid_ids(&vec![(565653, 565659)], is_valid_id_part_1),
            vec![]
        );
        assert_eq!(
            get_invalid_ids(&vec![(824824821, 824824827)], is_valid_id_part_1),
            vec![]
        );
        assert_eq!(
            get_invalid_ids(&vec![(2121212118, 2121212124)], is_valid_id_part_1),
            vec![]
        );
    }

    #[test]
    fn test_get_invalid_ids_part_2() {
        assert_eq!(
            get_invalid_ids(&vec![(11, 22)], is_valid_id_part_2),
            vec![11, 22]
        );
        assert_eq!(
            get_invalid_ids(&vec![(95, 115)], is_valid_id_part_2),
            vec![99, 111]
        );
        assert_eq!(
            get_invalid_ids(&vec![(998, 1012)], is_valid_id_part_2),
            vec![999, 1010]
        );
        assert_eq!(
            get_invalid_ids(&vec![(1188511880, 1188511890)], is_valid_id_part_2),
            vec![1188511885]
        );
        assert_eq!(
            get_invalid_ids(&vec![(222220, 222224)], is_valid_id_part_2),
            vec![222222]
        );
        assert_eq!(
            get_invalid_ids(&vec![(1698522, 1698528)], is_valid_id_part_2),
            vec![]
        );
        assert_eq!(
            get_invalid_ids(&vec![(446443, 446449)], is_valid_id_part_2),
            vec![446446]
        );
        assert_eq!(
            get_invalid_ids(&vec![(38593856, 38593862)], is_valid_id_part_2),
            vec![38593859]
        );
        assert_eq!(
            get_invalid_ids(&vec![(565653, 565659)], is_valid_id_part_2),
            vec![565656]
        );
        assert_eq!(
            get_invalid_ids(&vec![(824824821, 824824827)], is_valid_id_part_2),
            vec![824824824]
        );
        assert_eq!(
            get_invalid_ids(&vec![(2121212118, 2121212124)], is_valid_id_part_2),
            vec![2121212121]
        );
    }

    #[test]
    fn test_part_1_simple() {
        let input = vec![
            (11, 22),
            (95, 115),
            (998, 1012),
            (1188511880, 1188511890),
            (222220, 222224),
            (1698522, 1698528),
            (446443, 446449),
            (38593856, 38593862),
            (565653, 565659),
            (824824821, 824824827),
            (2121212118, 2121212124),
        ];
        assert_eq!(part_1(&input), 1227775554);
    }

    #[test]
    fn test_part_1_final() {
        assert_eq!(part_1(&parse_input(INPUT)), 23534117921);
    }

    #[test]
    fn test_part_2_simple() {
        let input = vec![
            (11, 22),
            (95, 115),
            (998, 1012),
            (1188511880, 1188511890),
            (222220, 222224),
            (1698522, 1698528),
            (446443, 446449),
            (38593856, 38593862),
            (565653, 565659),
            (824824821, 824824827),
            (2121212118, 2121212124),
        ];
        assert_eq!(part_2(&input), 4174379265);
    }

    #[test]
    fn test_part_2_final() {
        assert_eq!(part_2(&parse_input(INPUT)), 31755323497);
    }
}
