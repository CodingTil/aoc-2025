const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq)]
enum Operator {
    Addition,
    Multiplication,
}

impl Operator {
    fn apply(&self, a: isize, b: isize) -> isize {
        match self {
            Operator::Addition => a + b,
            Operator::Multiplication => a * b,
        }
    }
}

fn main() {
    let input = parse_input_part_1(INPUT);
    println!("Part 1: {}", apply_and_sum(input));
    let input = parse_input_part_2(INPUT);
    println!("Part 2: {}", apply_and_sum(input));
}

fn parse_input_part_1(input: &str) -> Vec<(Vec<isize>, Operator)> {
    let lines: Vec<&str> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();

    // Split the lines - last line contains operators, rest contain numbers
    let (number_lines, operator_line) = lines.split_at(lines.len() - 1);
    let operator_line = operator_line[0];

    // Parse operators
    let operators: Vec<Operator> = operator_line
        .split_whitespace()
        .map(|op| match op {
            "*" => Operator::Multiplication,
            "+" => Operator::Addition,
            _ => panic!("Unknown operator: {}", op),
        })
        .collect();

    // Parse numbers by columns
    let mut columns: Vec<Vec<isize>> = vec![Vec::new(); operators.len()];

    for line in number_lines {
        let numbers: Vec<isize> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        for (col_idx, &num) in numbers.iter().enumerate() {
            columns[col_idx].push(num);
        }
    }

    // Combine columns with operators
    columns.into_iter().zip(operators).collect()
}

fn apply_and_sum(input: Vec<(Vec<isize>, Operator)>) -> isize {
    input
        .into_iter()
        .map(|(nums, op)| {
            nums.into_iter()
                .reduce(|acc, num| op.apply(acc, num))
                .unwrap()
        })
        .sum()
}

fn parse_input_part_2(input: &str) -> Vec<(Vec<isize>, Operator)> {
    let lines: Vec<&str> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    let max_line_length = lines.iter().map(|line| line.len()).max().unwrap();

    // Split the lines - last line contains operators, rest contain numbers
    let (number_lines, operator_line) = lines.split_at(lines.len() - 1);
    let operator_line = operator_line[0];

    // Read Operators first
    // Each problem starts with an operator
    // The gap to the next operator - 1 tells us how wide each column is
    let mut operators: Vec<Operator> = Vec::new();
    let mut problem_widths: Vec<usize> = Vec::new();

    for (i, c) in operator_line.chars().enumerate() {
        let operator = match c {
            '+' => Some(Operator::Addition),
            '*' => Some(Operator::Multiplication),
            _ => None,
        };

        if operator.is_none() {
            continue;
        }

        if i > 0 {
            assert!(!operators.is_empty(),);
            problem_widths.push(i - problem_widths.iter().sum::<usize>() - operators.len());
        }
        operators.push(operator.unwrap());
    }
    problem_widths
        .push(max_line_length - problem_widths.iter().sum::<usize>() - operators.len() + 1);
    assert_eq!(operators.len(), problem_widths.len());

    // Now we can parse the individual problems
    let mut problems = Vec::new();
    let mut current_problem = Vec::new();
    let mut current_problem_column_start = 0;
    let mut current_column = 0;

    for problem_width in problem_widths {
        while current_column < current_problem_column_start + problem_width {
            let mut number_str = String::new();
            for number_line in number_lines {
                let n_th = number_line.chars().nth(current_column).unwrap_or(' ');
                if n_th.is_ascii_digit() {
                    number_str.push(n_th);
                }
            }
            current_problem.push(number_str.parse().unwrap());
            current_column += 1;
        }
        problems.push(current_problem.clone());
        current_problem.clear();
        current_column += 1;
        current_problem_column_start = current_column;
    }

    assert_eq!(operators.len(), problems.len());
    problems.into_iter().zip(operators).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

    #[test]
    fn test_parse_input_part_1() {
        assert_eq!(
            parse_input_part_1(TEST_INPUT),
            vec![
                (vec![123, 45, 6], Operator::Multiplication),
                (vec![328, 64, 98], Operator::Addition),
                (vec![51, 387, 215], Operator::Multiplication),
                (vec![64, 23, 314], Operator::Addition),
            ]
        )
    }

    #[test]
    fn test_part_1_simple() {
        assert_eq!(apply_and_sum(parse_input_part_1(TEST_INPUT)), 4277556);
    }

    #[test]
    fn test_part_1_final() {
        assert_eq!(apply_and_sum(parse_input_part_1(INPUT)), 4722948564882);
    }

    #[test]
    fn test_parse_input_part_2() {
        assert_eq!(
            parse_input_part_2(TEST_INPUT),
            vec![
                (vec![1, 24, 356], Operator::Multiplication),
                (vec![369, 248, 8], Operator::Addition),
                (vec![32, 581, 175], Operator::Multiplication),
                (vec![623, 431, 4], Operator::Addition),
            ]
        )
    }

    #[test]
    fn test_part_2_simple() {
        assert_eq!(apply_and_sum(parse_input_part_2(TEST_INPUT)), 3263827);
    }

    #[test]
    fn test_part_2_final() {
        assert_eq!(apply_and_sum(parse_input_part_2(INPUT)), 9581313737063);
    }
}
