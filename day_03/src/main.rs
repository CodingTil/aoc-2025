use include_lines::include_lines;

const INPUT: &[&str] = &include_lines!("./day_03/input.txt");

fn main() {
    println!("Hello, world!");

    let banks: Vec<Vec<usize>> = INPUT
        .iter()
        .map(|line| bank_input_to_batteries(line))
        .collect();
    println!("Part 1: {}", part_1(&banks));
    println!("Part 2: {}", part_2(&banks));
}

fn bank_input_to_batteries(input: &str) -> Vec<usize> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn part_1(banks: &[Vec<usize>]) -> usize {
    banks
        .iter()
        .map(|bank| bank_largest_joultage::<2>(bank))
        .sum()
}

fn part_2(banks: &[Vec<usize>]) -> usize {
    banks
        .iter()
        .map(|bank| bank_largest_joultage::<12>(bank))
        .sum()
}

fn bank_largest_joultage<const N: usize>(bank: &[usize]) -> usize {
    assert!(bank.len() >= N);

    let mut chosen: [usize; N] = bank[0..N].try_into().unwrap();

    for battery in bank.iter().skip(N) {
        // If there is a i<j in the chosen list with chosen[i] < chosen[j], throw out i, and append battery
        let mut already_inserted = false;
        for i in 0..N - 1 {
            if chosen[i] < chosen[i + 1] {
                for j in i..N - 1 {
                    chosen[j] = chosen[j + 1];
                }
                chosen[N - 1] = *battery;
                already_inserted = true;
                break;
            }
        }
        // Otherwise, check if we can replace the last element with the new battery
        if !already_inserted && chosen[N - 1] < *battery {
            chosen[N - 1] = *battery;
        }
    }

    let mut result = 0;
    for battery in chosen.iter() {
        result = result * 10 + battery;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bank_input_to_batteries() {
        assert_eq!(bank_input_to_batteries("12345"), vec![1, 2, 3, 4, 5])
    }

    #[test]
    fn test_bank_largest_joultage_2() {
        assert_eq!(
            bank_largest_joultage::<2>(&bank_input_to_batteries("987654321111111")),
            98
        );
        assert_eq!(
            bank_largest_joultage::<2>(&bank_input_to_batteries("811111111111119")),
            89
        );
        assert_eq!(
            bank_largest_joultage::<2>(&bank_input_to_batteries("234234234234278")),
            78
        );
        assert_eq!(
            bank_largest_joultage::<2>(&bank_input_to_batteries("818181911112111")),
            92
        );
    }

    #[test]
    fn test_bank_largest_joultage_12() {
        assert_eq!(
            bank_largest_joultage::<12>(&bank_input_to_batteries("987654321111111")),
            987654321111
        );
        assert_eq!(
            bank_largest_joultage::<12>(&bank_input_to_batteries("811111111111119")),
            811111111119
        );
        assert_eq!(
            bank_largest_joultage::<12>(&bank_input_to_batteries("234234234234278")),
            434234234278
        );
        assert_eq!(
            bank_largest_joultage::<12>(&bank_input_to_batteries("818181911112111")),
            888911112111
        );
    }

    #[test]
    fn test_part_1_simple() {
        let banks = vec![
            bank_input_to_batteries("987654321111111"),
            bank_input_to_batteries("811111111111119"),
            bank_input_to_batteries("234234234234278"),
            bank_input_to_batteries("818181911112111"),
        ];

        assert_eq!(part_1(&banks), 357);
    }

    #[test]
    fn test_part_1_final() {
        let banks: Vec<Vec<usize>> = INPUT
            .iter()
            .map(|line| bank_input_to_batteries(line))
            .collect();
        assert_eq!(part_1(&banks), 17524);
    }

    #[test]
    fn test_part_2_simple() {
        let banks = vec![
            bank_input_to_batteries("987654321111111"),
            bank_input_to_batteries("811111111111119"),
            bank_input_to_batteries("234234234234278"),
            bank_input_to_batteries("818181911112111"),
        ];

        assert_eq!(part_2(&banks), 3121910778619);
    }

    #[test]
    fn test_part_2_final() {
        let banks: Vec<Vec<usize>> = INPUT
            .iter()
            .map(|line| bank_input_to_batteries(line))
            .collect();
        assert_eq!(part_2(&banks), 173848577117276);
    }
}
