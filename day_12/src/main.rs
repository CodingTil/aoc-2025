use std::{
    collections::{HashMap, HashSet},
    ops::Div,
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use russcip::{minimal_model, prelude::*};

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Shape {
    blocks: [[bool; 3]; 3],
}

impl Shape {
    fn has_block(&self, i: usize, j: usize) -> bool {
        self.blocks[i][j]
    }

    fn area(&self) -> usize {
        let mut area = 0;
        for row in &self.blocks {
            for &block in row {
                if block {
                    area += 1;
                }
            }
        }
        area
    }

    fn get_invariants(&self) -> Vec<Shape> {
        let mut invariants = HashSet::new();
        invariants.insert(self.clone());
        let mut shape = self.clone();
        for _ in 0..4 {
            invariants.insert(shape.clone());
            shape = shape.rotate_left();
        }
        shape = shape.flip_horizontal();
        invariants.insert(shape.clone());
        for _ in 0..4 {
            invariants.insert(shape.clone());
            shape = shape.rotate_left();
        }
        invariants.into_iter().collect()
    }

    fn rotate_left(&self) -> Shape {
        let mut rotated = Shape {
            blocks: [[false; 3]; 3],
        };
        for i in 0..3 {
            for j in 0..3 {
                rotated.blocks[j][2 - i] = self.blocks[i][j];
            }
        }
        rotated
    }

    fn flip_horizontal(&self) -> Shape {
        let mut flipped = Shape {
            blocks: [[false; 3]; 3],
        };
        for i in 0..3 {
            for j in 0..3 {
                flipped.blocks[i][2 - j] = self.blocks[i][j];
            }
        }
        flipped
    }
}

struct Region {
    dimensions: (usize, usize),
    required_shape_counts: Vec<usize>,
}

impl Region {
    fn solvable(&self, shapes: &[Vec<Shape>]) -> bool {
        assert_eq!(self.required_shape_counts.len(), shapes.len());
        if let Some(trivial_result) = self.trivial_check(shapes) {
            return trivial_result;
        }
        self.thorough_check(shapes)
    }

    fn trivial_check(&self, shapes: &[Vec<Shape>]) -> Option<bool> {
        if self.dimensions.0.div(3) * self.dimensions.1.div(3)
            >= self.required_shape_counts.iter().sum::<usize>()
        {
            return Some(true);
        }

        if self.dimensions.0 * self.dimensions.1
            < shapes
                .iter()
                .map(|shapes| shapes.first().unwrap())
                .zip(self.required_shape_counts.iter())
                .map(|(shape, count)| shape.area() * count)
                .sum::<usize>()
        {
            return Some(false);
        }

        None
    }

    fn thorough_check(&self, shapes: &[Vec<Shape>]) -> bool {
        let mut model = minimal_model().minimize().hide_output();

        let mut variables = HashMap::new();
        // a variable only for the top left placement of each shape
        for (id, shapes) in shapes.iter().enumerate() {
            for shape in shapes {
                for i in 0..=self.dimensions.0 - 3 {
                    for j in 0..=self.dimensions.1 - 3 {
                        variables.insert((id, shape, i, j), model.add(var().bin().obj(0.)));
                    }
                }
            }
        }

        for i in 0..self.dimensions.0 {
            for j in 0..self.dimensions.1 {
                let mut constraint = cons().le(1.);
                for offset_x in 0..3 {
                    for offset_y in 0..3 {
                        if i < offset_x
                            || j < offset_y
                            || i - offset_x > self.dimensions.0 - 3
                            || j - offset_y > self.dimensions.1 - 3
                        {
                            continue;
                        }

                        for (id, shapes) in shapes.iter().enumerate() {
                            for shape in shapes {
                                if shape.has_block(offset_x, offset_y) {
                                    constraint = constraint.coef(
                                        variables
                                            .get(&(id, shape, i - offset_x, j - offset_y))
                                            .unwrap(),
                                        1.,
                                    );
                                }
                            }
                        }
                    }
                }
                model.add(constraint);
            }
        }

        for (id, (shapes, &count)) in shapes
            .iter()
            .zip(self.required_shape_counts.iter())
            .enumerate()
        {
            let mut constraint = cons().eq(count as f64);
            for shape in shapes {
                for i in 0..=self.dimensions.0 - 3 {
                    for j in 0..=self.dimensions.1 - 3 {
                        constraint =
                            constraint.coef(variables.get(&(id, shape, i, j)).unwrap(), 1.);
                    }
                }
            }
            model.add(constraint);
        }

        let solved_model = model.solve();

        solved_model.n_sols() > 0
    }
}

fn main() {
    let (shapes, regions) = parse_input(INPUT);
    println!("Solvable: {}", count_solvable_regions(shapes, regions));
}

fn parse_input(input: &str) -> (Vec<Vec<Shape>>, Vec<Region>) {
    let mut parts = input.split("\n\n").collect::<Vec<&str>>();
    let regions_part = &parts.remove(parts.len() - 1);
    let shapes_part = parts;

    let shapes = shapes_part
        .iter()
        .map(|shape_str| {
            let mut blocks = [[false; 3]; 3];
            for (y, line) in shape_str
                .split(':')
                .nth(1)
                .unwrap()
                .trim()
                .lines()
                .take(3)
                .enumerate()
            {
                for (x, block) in line.trim().chars().enumerate().take(3) {
                    blocks[y][x] = block == '#';
                }
            }
            Shape { blocks }.get_invariants()
        })
        .collect::<Vec<Vec<Shape>>>();

    let regions = regions_part
        .split('\n')
        .map(|region_str| {
            let (dimensions_str, required_shapes_str) = region_str.split_once(":").unwrap();
            let (dimension_x_str, dimension_y_str) = dimensions_str.split_once('x').unwrap();
            let dimensions = (
                dimension_x_str.parse().unwrap(),
                dimension_y_str.parse().unwrap(),
            );
            let required_shape_counts = required_shapes_str
                .trim()
                .split(' ')
                .map(|count_str| count_str.parse().unwrap())
                .collect::<Vec<usize>>();
            Region {
                dimensions,
                required_shape_counts,
            }
        })
        .collect::<Vec<Region>>();

    (shapes, regions)
}

fn count_solvable_regions(shapes: Vec<Vec<Shape>>, regions: Vec<Region>) -> usize {
    regions
        .par_iter()
        .filter(|region| region.solvable(&shapes))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn test_count_solvable_regions_simple() {
        let (shapes, regions) = parse_input(TEST_INPUT);
        assert_eq!(count_solvable_regions(shapes, regions), 2);
    }

    #[test]
    fn test_count_solvable_regions_final() {
        let (shapes, regions) = parse_input(INPUT);
        assert_eq!(count_solvable_regions(shapes, regions), 495);
    }
}
