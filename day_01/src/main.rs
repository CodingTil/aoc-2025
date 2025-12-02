use include_lines::include_lines;

const INPUT: &[&str] = &include_lines!("./day_01/input.txt");

fn main() {
    part_1(INPUT);
    part_2(INPUT);
}

fn part_1(input: &[&str]) -> isize {
    let mut dial: isize = 50;
    let mut zero_count = 0;

    for line in input {
        let clicks = get_clicks(line);
        dial = (dial + clicks).rem_euclid(100);
        if dial == 0 {
            zero_count += 1;
        }
    }

    println!(
        "[PART 1] The dial pointed to 0 a total of {} times.",
        zero_count
    );
    zero_count
}

fn part_2(input: &[&str]) -> isize {
    let mut dial: isize = 50;
    let mut zero_crossings_count = 0;

    for line in input {
        let clicks = get_clicks(line);
        let full_rotations = clicks.abs().div_euclid(100);
        zero_crossings_count += full_rotations;
        let remainder = clicks + (-clicks.signum()) * full_rotations * 100;
        if (remainder < 0 && dial > 0 && remainder + dial <= 0)
            || (remainder > 0 && dial < 100 && remainder + dial >= 100)
        {
            zero_crossings_count += 1;
        }
        dial = (dial + clicks).rem_euclid(100);
    }

    println!(
        "[PART 2] The dial crossed / pointed to 0 a total of {} times.",
        zero_crossings_count
    );
    zero_crossings_count
}

fn get_clicks(input: &str) -> isize {
    let (direction, value) = input.split_at(1);
    let value: isize = value.parse().unwrap();
    match direction {
        "L" => -value,
        "R" => value,
        _ => panic!("Invalid direction"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_complete() {
        assert_eq!(part_1(INPUT), 1172)
    }

    #[test]
    fn test_part_1_simple() {
        let input = vec![
            ("L68", 0),
            ("L30", 0),
            ("R48", 1),
            ("L5", 1),
            ("R60", 1),
            ("L55", 2),
            ("L1", 2),
            ("L99", 3),
            ("R14", 3),
            ("L82", 3),
        ];
        for (i, (_, expected)) in input.iter().enumerate() {
            let slice: Vec<&str> = input.iter().take(i + 1).map(|(s, _)| *s).collect();
            assert_eq!(
                part_1(&slice),
                *expected,
                "Failed at index {} with input {:?}",
                i,
                slice
            );
        }
    }

    #[test]
    fn test_part_2_complete() {
        assert_eq!(part_2(INPUT), 6932)
    }

    #[test]
    fn test_part_2_simple() {
        let input = vec![
            ("L68", 1),
            ("L30", 1),
            ("R48", 2),
            ("L5", 2),
            ("R60", 3),
            ("L55", 4),
            ("L1", 4),
            ("L99", 5),
            ("R14", 5),
            ("L82", 6),
        ];
        for (i, (_, expected)) in input.iter().enumerate() {
            let slice: Vec<&str> = input.iter().take(i + 1).map(|(s, _)| *s).collect();
            assert_eq!(
                part_2(&slice),
                *expected,
                "Failed at index {} with input {:?}",
                i,
                slice
            );
        }
    }
}
