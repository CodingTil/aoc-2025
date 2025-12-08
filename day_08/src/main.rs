const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct JunctionBox {
    x: usize,
    y: usize,
    z: usize,
}

impl JunctionBox {
    #[cfg(test)]
    fn new(x: usize, y: usize, z: usize) -> Self {
        JunctionBox { x, y, z }
    }

    fn euclidean_distance(&self, other: &JunctionBox) -> f64 {
        ((self.x as f64 - other.x as f64).powi(2)
            + (self.y as f64 - other.y as f64).powi(2)
            + (self.z as f64 - other.z as f64).powi(2))
        .sqrt()
    }
}

fn main() {
    let junction_boxes = parse_input(INPUT);
    println!("Part 1: {}", part_1::<997, 3>(&junction_boxes));
    println!("Part 2: {}", part_2(&junction_boxes));
}

fn parse_input(input: &str) -> Vec<JunctionBox> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            JunctionBox {
                x: parts[0].parse().unwrap(),
                y: parts[1].parse().unwrap(),
                z: parts[2].parse().unwrap(),
            }
        })
        .collect()
}

fn sorted_pairs(input: &[JunctionBox]) -> Vec<(JunctionBox, JunctionBox)> {
    let mut pairs = Vec::new();
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            pairs.push((input[i], input[j]));
        }
    }
    pairs.sort_by(|(a_1, a_2), (b_1, b_2)| {
        a_1.euclidean_distance(a_2)
            .partial_cmp(&b_1.euclidean_distance(b_2))
            .unwrap()
    });
    pairs
}

fn determine_clusters<const N: usize>(input: &[JunctionBox]) -> Vec<Vec<JunctionBox>> {
    let mut clusters = input
        .iter()
        .map(|junction_box| vec![*junction_box])
        .collect::<Vec<Vec<JunctionBox>>>();
    for (a, b) in sorted_pairs(input).iter().take(N) {
        if clusters
            .iter()
            .any(|cluster| cluster.contains(a) && cluster.contains(b))
        {
            continue;
        }
        let merged_clusters = clusters
            .iter()
            .filter(|cluster| cluster.contains(a) || cluster.contains(b))
            .cloned()
            .reduce(|mut acc, mut cluster| {
                acc.append(&mut cluster);
                acc
            })
            .unwrap();
        let len = clusters.len();
        clusters.retain(|cluster| !cluster.contains(a) && !cluster.contains(b));
        assert_eq!(len - clusters.len(), 2);
        clusters.push(merged_clusters);
    }
    clusters
}

fn part_1<const N: usize, const M: usize>(input: &[JunctionBox]) -> usize {
    let clusters = determine_clusters::<N>(input);
    let mut cluster_lengths = clusters
        .iter()
        .map(|cluster| cluster.len())
        .collect::<Vec<usize>>();
    cluster_lengths.sort_by(|a, b| b.cmp(a));
    cluster_lengths.iter().take(M).product()
}

fn get_connecting_pair(input: &[JunctionBox]) -> Option<(JunctionBox, JunctionBox)> {
    let mut clusters = input
        .iter()
        .map(|junction_box| vec![*junction_box])
        .collect::<Vec<Vec<JunctionBox>>>();
    for (a, b) in sorted_pairs(input).iter() {
        if clusters
            .iter()
            .any(|cluster| cluster.contains(a) && cluster.contains(b))
        {
            continue;
        }
        let merged_clusters = clusters
            .iter()
            .filter(|cluster| cluster.contains(a) || cluster.contains(b))
            .cloned()
            .reduce(|mut acc, mut cluster| {
                acc.append(&mut cluster);
                acc
            })
            .unwrap();
        let len = clusters.len();
        clusters.retain(|cluster| !cluster.contains(a) && !cluster.contains(b));
        assert_eq!(len - clusters.len(), 2);
        clusters.push(merged_clusters);
        if clusters.len() == 1 {
            return Some((*a, *b));
        }
    }
    None
}

fn part_2(input: &[JunctionBox]) -> usize {
    match get_connecting_pair(input) {
        Some((a, b)) => a.x * b.x,
        None => panic!("No connecting pair found"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
    162,817,812
    57,618,57
    906,360,560
    592,479,940
    352,342,300
    466,668,158
    542,29,236
    431,825,988
    739,650,466
    52,470,668
    216,146,977
    819,987,18
    117,168,530
    805,96,715
    346,949,466
    970,615,88
    941,993,340
    862,61,35
    984,92,344
    425,690,689
    ";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(TEST_INPUT),
            vec![
                JunctionBox::new(162, 817, 812),
                JunctionBox::new(57, 618, 57),
                JunctionBox::new(906, 360, 560),
                JunctionBox::new(592, 479, 940),
                JunctionBox::new(352, 342, 300),
                JunctionBox::new(466, 668, 158),
                JunctionBox::new(542, 29, 236),
                JunctionBox::new(431, 825, 988),
                JunctionBox::new(739, 650, 466),
                JunctionBox::new(52, 470, 668),
                JunctionBox::new(216, 146, 977),
                JunctionBox::new(819, 987, 18),
                JunctionBox::new(117, 168, 530),
                JunctionBox::new(805, 96, 715),
                JunctionBox::new(346, 949, 466),
                JunctionBox::new(970, 615, 88),
                JunctionBox::new(941, 993, 340),
                JunctionBox::new(862, 61, 35),
                JunctionBox::new(984, 92, 344),
                JunctionBox::new(425, 690, 689),
            ]
        );
    }

    #[test]
    fn test_sorted_pairs() {
        assert_eq!(
            sorted_pairs(&parse_input(TEST_INPUT))[0..4],
            vec![
                (
                    JunctionBox::new(162, 817, 812),
                    JunctionBox::new(425, 690, 689)
                ),
                (
                    JunctionBox::new(162, 817, 812),
                    JunctionBox::new(431, 825, 988)
                ),
                (
                    JunctionBox::new(906, 360, 560),
                    JunctionBox::new(805, 96, 715)
                ),
                (
                    JunctionBox::new(431, 825, 988),
                    JunctionBox::new(425, 690, 689)
                ),
            ]
        );
    }

    #[test]
    fn test_part_1_simple() {
        let junction_boxes = parse_input(TEST_INPUT);
        assert_eq!(part_1::<10, 3>(&junction_boxes), 40);
    }

    #[test]
    fn test_part_1_final() {
        let junction_boxes = parse_input(INPUT);
        assert_eq!(part_1::<1000, 3>(&junction_boxes), 72150);
    }

    #[test]
    fn test_part_2_simple() {
        let junction_boxes = parse_input(TEST_INPUT);
        assert_eq!(part_2(&junction_boxes), 25272);
    }

    #[test]
    fn test_part_2_final() {
        let junction_boxes = parse_input(INPUT);
        assert_eq!(part_2(&junction_boxes), 3926518899);
    }
}
