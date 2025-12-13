use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use russcip::{minimal_model, prelude::*};

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, PartialEq, Eq)]
enum Light {
    On,
    Off,
}

impl Light {
    fn toggle(&mut self) {
        *self = match self {
            Light::On => Light::Off,
            Light::Off => Light::On,
        };
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Action {
    toogles: Vec<bool>,
}

impl Action {
    fn apply_lights(&self, lights: &mut [Light]) {
        assert_eq!(lights.len(), self.toogles.len());
        for (i, light) in lights.iter_mut().enumerate() {
            if self.toogles[i] {
                light.toggle();
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Machine {
    lights: Vec<Light>,
    actions: Vec<Action>,
    joltages: Vec<usize>,
}

impl Machine {
    fn determine_least_actions_to_turn_on(&self) -> usize {
        let mut nodes_to_check = vec![SearchTreeNodeLights::root(self.lights.len())];
        let mut current_action_count = 0;
        while !nodes_to_check.is_empty() {
            let mut new_nodes_to_check = Vec::new();
            for node in &nodes_to_check {
                if node.get_state() == self.lights {
                    return current_action_count;
                }
                new_nodes_to_check.append(&mut node.children(&self.actions));
            }
            nodes_to_check.clear();
            nodes_to_check.append(&mut new_nodes_to_check);
            current_action_count += 1;
        }
        panic!()
    }

    fn determine_least_actions_to_set_joltage(&self) -> usize {
        let mut model = minimal_model().minimize().hide_output();

        let variables = self
            .actions
            .iter()
            .map(|_| model.add(var().int(0..).obj(1.)))
            .collect::<Vec<_>>();

        let constraints = self
            .joltages
            .iter()
            .enumerate()
            .map(|(i, &joltage)| {
                let mut constraint = cons().eq(joltage as f64);
                for (_, variable) in self
                    .actions
                    .iter()
                    .zip(variables.iter())
                    .filter(|(action, _)| action.toogles[i])
                {
                    constraint = constraint.coef(variable, 1.);
                }
                constraint
            })
            .collect::<Vec<_>>();

        model.add(constraints);

        let solved_model = model.solve();

        let sol = solved_model.best_sol().unwrap();
        let value = sol.obj_val();

        value.round() as usize
    }
}

struct SearchTreeNodeLights {
    lights: Vec<Light>,
    previous_actions: Vec<Action>,
}

impl SearchTreeNodeLights {
    fn root(size: usize) -> SearchTreeNodeLights {
        SearchTreeNodeLights {
            lights: vec![Light::Off; size],
            previous_actions: Vec::new(),
        }
    }
}

impl SearchTreeNodeLights {
    fn get_state(&self) -> Vec<Light> {
        self.lights.clone()
    }

    fn children(&self, all_actions: &[Action]) -> Vec<SearchTreeNodeLights> {
        all_actions
            .iter()
            .filter_map(|action| match self.previous_actions.contains(action) {
                true => None,
                false => {
                    let mut new_actions = self.previous_actions.clone();
                    new_actions.push(action.clone());
                    let mut new_lights = self.lights.clone();
                    action.apply_lights(&mut new_lights);
                    Some(SearchTreeNodeLights {
                        lights: new_lights,
                        previous_actions: new_actions,
                    })
                }
            })
            .collect()
    }
}

fn main() {
    let machines = parse_input(INPUT);
    println!("Part 1: {}", part_1(&machines));
    println!("Part 2: {}", part_2(&machines));
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let lights: Vec<Light> = parts[0]
                .chars()
                .filter_map(|c| match c {
                    '[' => None,
                    ']' => None,
                    '.' => Some(Light::Off),
                    '#' => Some(Light::On),
                    _ => panic!("Invalid light state"),
                })
                .collect();
            let actions = parts[1..parts.len() - 1]
                .iter()
                .map(|&s| {
                    let mut toogles = vec![false; lights.len()];
                    let s = s.trim_start_matches('(').trim_end_matches(')');
                    let numbers: Vec<usize> = s.split(',').map(|s| s.parse().unwrap()).collect();
                    for number in numbers {
                        toogles[number] = true;
                    }
                    Action { toogles }
                })
                .collect();
            let joltages = parts[parts.len() - 1]
                .trim_start_matches('{')
                .trim_end_matches('}')
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            Machine {
                lights,
                actions,
                joltages,
            }
        })
        .collect()
}

fn part_1(machines: &[Machine]) -> usize {
    machines
        .par_iter()
        .map(|machine| machine.determine_least_actions_to_turn_on())
        .sum()
}

fn part_2(machines: &[Machine]) -> usize {
    machines
        .par_iter()
        .map(|machine| machine.determine_least_actions_to_set_joltage())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_INPUT: &str = "
    [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    ";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(TEST_INPUT),
            vec![
                Machine {
                    lights: vec![Light::Off, Light::On, Light::On, Light::Off],
                    actions: vec![
                        Action {
                            toogles: vec![false, false, false, true]
                        },
                        Action {
                            toogles: vec![false, true, false, true]
                        },
                        Action {
                            toogles: vec![false, false, true, false]
                        },
                        Action {
                            toogles: vec![false, false, true, true]
                        },
                        Action {
                            toogles: vec![true, false, true, false]
                        },
                        Action {
                            toogles: vec![true, true, false, false]
                        },
                    ],
                    joltages: vec![3, 5, 4, 7],
                },
                Machine {
                    lights: vec![Light::Off, Light::Off, Light::Off, Light::On, Light::Off],
                    actions: vec![
                        Action {
                            toogles: vec![true, false, true, true, true]
                        },
                        Action {
                            toogles: vec![false, false, true, true, false]
                        },
                        Action {
                            toogles: vec![true, false, false, false, true]
                        },
                        Action {
                            toogles: vec![true, true, true, false, false]
                        },
                        Action {
                            toogles: vec![false, true, true, true, true]
                        },
                    ],
                    joltages: vec![7, 5, 12, 7, 2],
                },
                Machine {
                    lights: vec![
                        Light::Off,
                        Light::On,
                        Light::On,
                        Light::On,
                        Light::Off,
                        Light::On
                    ],
                    actions: vec![
                        Action {
                            toogles: vec![true, true, true, true, true, false]
                        },
                        Action {
                            toogles: vec![true, false, false, true, true, false]
                        },
                        Action {
                            toogles: vec![true, true, true, false, true, true]
                        },
                        Action {
                            toogles: vec![false, true, true, false, false, false]
                        },
                    ],
                    joltages: vec![10, 11, 11, 5, 10, 5],
                }
            ]
        );
    }

    #[test]
    fn test_determine_least_actions_to_turn_on() {
        let machines = parse_input(TEST_INPUT);
        assert_eq!(
            machines
                .iter()
                .map(|machine| machine.determine_least_actions_to_turn_on())
                .collect::<Vec<_>>(),
            vec![2, 3, 2]
        );
    }

    #[test]
    fn test_determine_least_actions_to_set_joltage() {
        let machines = parse_input(TEST_INPUT);
        assert_eq!(
            machines
                .iter()
                .map(|machine| machine.determine_least_actions_to_set_joltage())
                .collect::<Vec<_>>(),
            vec![10, 12, 11]
        );
    }

    #[test]
    fn test_part_1_simple() {
        let machines = parse_input(TEST_INPUT);
        assert_eq!(part_1(&machines), 7);
    }

    #[test]
    fn test_part_1_final() {
        let machines = parse_input(INPUT);
        assert_eq!(part_1(&machines), 399);
    }

    #[test]
    fn test_part_2_simple() {
        let machines = parse_input(TEST_INPUT);
        assert_eq!(part_2(&machines), 33);
    }

    #[test]
    fn test_part_2_final() {
        let machines = parse_input(INPUT);
        assert_eq!(part_2(&machines), 15631);
    }
}
