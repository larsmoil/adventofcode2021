use std::cmp::max;
use std::fmt::{Display, Formatter};
use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, inp: &str) -> String {
        let mut manual = Manual::new(inp);
        manual.fold();
        manual.coordinates.len().to_string()
    }
    fn pt2(&self, inp: &str) -> String {
        let mut manual = Manual::new(inp);
        while !manual.folds.is_empty() {
            manual.fold();
        }
        format!("\n{}", manual)
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("input13.txt").trim()
}

#[derive(Eq, PartialEq)]
struct Manual {
    coordinates: Vec<(i16, i16)>,
    folds: Vec<(u8, i16)>,
}

impl Display for Manual {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let width = 1 + self.coordinates.iter().fold(0_usize, |a, b| max(a, b.0 as usize));
        let height = 1 + self.coordinates.iter().fold(0_usize, |a, b| max(a, b.1 as usize));
        let mut map = vec!['.'; width * height];

        for coordinate in &self.coordinates {
            let (x, y) = coordinate;
            map[(*x as usize) + (*y as usize) * width] = '#';
        }

        let lines = map.chunks_exact(width)
            .map(|line| line.iter().map(|dot| dot.to_string()).collect::<String>())
            .collect::<Vec<String>>();
        write!(
            f,
            "{}",
            lines.join("\n")
        )
    }
}

impl Manual {
    fn new(inp: &str) -> Manual {
        let (coordinates, folds) = inp.split_once("\n\n").unwrap();
        let folds = folds
            .lines()
            .map(|line| line.trim_start_matches("fold along ").split_once('=').unwrap())
            .map(|(xy, i)| (xy.as_bytes()[0], i.parse::<i16>().unwrap()))
            .collect::<Vec<(u8, i16)>>();
        let coordinates = coordinates
            .lines()
            .map(|line| line.split_once(',').unwrap())
            .map(|(x, y)| (x.parse::<i16>().unwrap(), y.parse::<i16>().unwrap()))
            .collect::<Vec<(i16, i16)>>();

        Manual { coordinates, folds }
    }
    fn fold(&mut self) {
        let fold = self.folds.remove(0);
        let (direction, i) = fold;
        let mut new_coordinates: Vec<(i16, i16)> = self
            .coordinates
            .iter()
            .filter_map(|(mut x, mut y)| {
                match direction {
                    b'x' if x == i => return None,
                    b'x' if x > i => x = i - (x - i),
                    b'y' if y == i => return None,
                    b'y' if y > i => y = i - (y - i),
                    _ => {}
                }
                Some((x, y))
            })
            .collect();
        new_coordinates.sort_unstable();
        new_coordinates.dedup();
        self.coordinates = new_coordinates;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5".trim()
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!(Day {}.pt1(example_input()), "17")
    }

    #[test]
    fn test_pt1() {
        assert_eq!(Day {}.pt1(input()), "693")
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!(Day {}.pt2(example_input()), "
#####
#...#
#...#
#...#
#####
".trim_end())
    }

    #[test]
    fn test_pt2() {
        assert_eq!(Day {}.pt2(input()), "
#..#..##..#....####.###...##..####.#..#
#..#.#..#.#.......#.#..#.#..#....#.#..#
#..#.#....#......#..#..#.#..#...#..#..#
#..#.#....#.....#...###..####..#...#..#
#..#.#..#.#....#....#.#..#..#.#....#..#
.##...##..####.####.#..#.#..#.####..##.
".trim_end())
    }
}
