use std::cmp::{max, min};
use std::fmt::{Display, Formatter};
use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, inp: &str) -> String {
        Grid::new(inp)
            .simulate(100)
            .octopuses
            .iter()
            .map(|octopus| octopus.flashes)
            .sum::<u64>()
            .to_string()
    }
    fn pt2(&self, inp: &str) -> String {
        let mut grid = Grid::new(inp);
        let mut i = 0;
        while !grid.zeroed() {
            grid.simulate(1);
            i += 1;
        }
        i.to_string()
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("input11.txt").trim()
}

#[derive(Debug, Eq, PartialEq)]
struct Octopus {
    energy: u8,
    flashes: u64,
    flashed_this_round: bool,
}

impl Octopus {
    fn new(energy: u8) -> Octopus {
        Octopus {
            energy,
            flashes: 0,
            flashed_this_round: false,
        }
    }
    fn charge(&mut self) -> Option<u8> {
        if !self.flashed_this_round {
            self.energy += 1;
            Option::Some(self.energy)
        } else {
            Option::None
        }
    }
    fn flash(&mut self) -> Option<u64> {
        if !self.flashed_this_round && self.energy > 9 {
            self.flashed_this_round = true;
            self.flashes += 1;
            Option::Some(self.flashes)
        } else {
            Option::None
        }
    }
    fn reset(&mut self) {
        self.flashed_this_round = false;
        if self.energy > 9 {
            self.energy = 0;
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Grid {
    octopuses: Vec<Octopus>,
    width: usize,
    height: usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.octopuses
                .chunks(self.width)
                .map(|os| os.iter().map(|o| o.energy.to_string()).collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Grid {
    fn new(inp: &str) -> Grid {
        let lines: Vec<&str> = inp.split('\n').collect();
        let height = lines.len();
        let width = lines.first().unwrap().len();
        let energy_levels: Vec<u8> = lines
            .iter()
            .flat_map(|line| line.chars().map(|c| c.to_string().parse::<u8>().unwrap()))
            .collect();
        let octopuses: Vec<Octopus> = energy_levels
            .iter()
            .map(|energy| Octopus::new(*energy))
            .collect();
        Grid {
            octopuses,
            width,
            height,
        }
    }

    fn zeroed(&self) -> bool {
        self.octopuses.iter().all(|o| o.energy == 0)
    }

    fn simulate(&mut self, steps: u8) -> &Grid {
        let mut steps = steps;
        while steps > 0 {
            for i in 0..self.octopuses.len() {
                let octopus = self.octopuses.get_mut(i).unwrap();
                octopus.charge();
            }
            for i in 0..self.octopuses.len() {
                self.flash(i);
            }
            for i in 0..self.octopuses.len() {
                let octopus = self.octopuses.get_mut(i).unwrap();
                octopus.reset();
            }
            steps -= 1
        }
        self
    }

    fn flash(&mut self, index: usize) {
        let octopus = self.octopuses.get_mut(index).unwrap();
        if octopus.flash().is_some() {
            for i in self.adjacent(index) {
                let o = self.octopuses.get_mut(i).unwrap();
                o.charge();
                self.flash(i);
            }
        }
    }

    fn adjacent(&self, index: usize) -> Vec<usize> {
        let mut adjacent: Vec<usize> = vec![];
        let row = index / self.width;
        let column = index % self.width;

        for r in (max(row, 1) - 1)..=min(self.height - 1, row + 1) {
            for c in (max(1, column) - 1)..=min(self.width - 1, column + 1) {
                let octopus = &self.octopuses[r * self.width + c];
                if (r != row || c != column) && !octopus.flashed_this_round {
                    adjacent.push(r * self.width + c);
                }
            }
        }

        adjacent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526".trim()
    }

    #[test]
    fn test_simulate() {
        let mut grid = Grid::new(example_input());
        assert_eq!(
            Grid {
                octopuses: example_input()
                    .replace("\n", "")
                    .chars()
                    .map(|c| c.to_string().parse::<u8>().unwrap())
                    .map(|energy| Octopus { energy, flashes: 0, flashed_this_round: false })
                    .collect(),
                height: 10,
                width: 10,
            },
            grid
        );

        grid.simulate(1);
        assert_eq!(
            "
6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637".trim(),
            format!("{}", grid)
        );

        grid.simulate(1);
        assert_eq!(
            "
8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848".trim(),
            format!("{}", grid)
        );

        grid.simulate(1);
        assert_eq!(
            "
0050900866
8500800575
9900000039
9700000041
9935080063
7712300000
7911250009
2211130000
0421125000
0021119000".trim(),
            format!("{}", grid)
        );
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!(Day {}.pt1(example_input()), "1656")
    }

    #[test]
    fn test_pt1() {
        assert_eq!(Day {}.pt1(input()), "1599")
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!(Day {}.pt2(example_input()), "195")
    }

    #[test]
    fn test_pt2() {
        assert_eq!(Day {}.pt2(input()), "418")
    }
}
