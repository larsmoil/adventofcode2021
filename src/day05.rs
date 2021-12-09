use std::cmp::{max, min};
use std::ops::RangeInclusive;
use crate::problem::Solver;

pub struct Day {}

type Point = (i64, i64);
type Line = (Point, Point);

impl Solver for Day {
    fn pt1(&self, inp: &str) -> String {
        let lines = hor_vert_lines(lines(inp));
        solve(lines).to_string()
    }
    fn pt2(&self, inp: &str) -> String {
        let hor_vert_lines = hor_vert_lines(lines(inp));
        let diagonal_lines = diagonal_lines(lines(inp));
        let lines = vec![hor_vert_lines, diagonal_lines].iter()
            .flat_map(|a| a.iter())
            .map(|v| v.to_owned())
            .collect();
        solve(lines).to_string()
    }
}

fn solve(lines: Vec<Line>) -> i64 {
    let map = map(&lines);
    let x_min = map.0.0;
    let x_max = map.0.1;
    let xs = x_max - x_min + 1;
    let mut points = points(map);
    for line in lines {
        let x1 = min(line.0.0, line.1.0);
        let x2 = max(line.0.0, line.1.0);
        let y1 = min(line.0.1, line.1.1);
        let y2 = max(line.0.1, line.1.1);

        for i in RangeInclusive::new(0, max(x2 - x1, y2 - y1)) {
            let x = line.0.0 + i * ((line.1.0 - line.0.0) / max(1, x2 - x1));
            let y = line.0.1 + i * ((line.1.1 - line.0.1) / max(1, y2 - y1));
            let index = (xs * y + x) as usize;
            points[index] += 1;
        }
    }
    let at_least_twice = points
        .iter()
        .filter(|point| **point > 1)
        .count();
    at_least_twice as i64
}

fn lines(inp: &str) -> Vec<Line> {
    inp
        .split('\n')
        .map(|line| {
            let coordinates: Vec<Point> = line
                .split(" -> ")
                .map(|coordinate| {
                    let points: Vec<i64> = coordinate.split(',').map(|point| point.parse::<i64>().unwrap()).collect();
                    let (x, y) = (points[0], points[1]);
                    (x, y)
                })
                .collect();
            let line: (Point, Point) = (coordinates[0], coordinates[1]);
            line
        })
        .collect()
}

fn diagonal_lines(lines: Vec<Line>) -> Vec<Line> {
    lines
        .iter()
        .filter(|line| {
            let x1 = min(line.0.0, line.1.0);
            let x2 = max(line.0.0, line.1.0);
            let y1 = min(line.0.1, line.1.1);
            let y2 = max(line.0.1, line.1.1);

            ((x2 - x1) as i64).abs() == ((y2 - y1) as i64).abs()
        })
        .copied()
        .collect()
}

fn hor_vert_lines(lines: Vec<Line>) -> Vec<Line> {
    lines
        .iter()
        .filter(|line| line.0.0 == line.1.0 || line.0.1 == line.1.1)
        .copied()
        .collect()
}


/// Returns the outer points of the map on the format ((x_min, x_max), (y_min, y_max)
fn map(lines: &[Line]) -> (Point, Point) {
    let mut xs: Vec<i64> = lines.iter().flat_map(|line| vec![line.0.0, line.1.0]).collect();
    let mut ys: Vec<i64> = lines.iter().flat_map(|line| vec![line.0.1, line.1.1]).collect();
    xs.sort_unstable();
    ys.sort_unstable();
    ((*xs.first().unwrap(), *xs.last().unwrap()), (*ys.first().unwrap(), *ys.last().unwrap()))
}

fn points(map: (Point, Point)) -> Vec<i64> {
    let xs = map.0.1 /*- map.0.0*/ + 1;
    let ys = map.1.1 /*- map.1.0*/ + 1;
    [0].repeat((xs * ys) as usize)
}

pub(crate) fn input() -> &'static str {
    include_str!("input05.txt").trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
    }

    #[test]
    fn test_lines() {
        assert_eq!(
            vec![
                ((0, 9), (5, 9)),
                ((8, 0), (0, 8)),
                ((9, 4), (3, 4)),
                ((2, 2), (2, 1)),
                ((7, 0), (7, 4)),
                ((6, 4), (2, 0)),
                ((0, 9), (2, 9)),
                ((3, 4), (1, 4)),
                ((0, 0), (8, 8)),
                ((5, 5), (8, 2)),
            ],
            lines(example_input())
        );
    }

    #[test]
    fn test_diagonal_lines() {
        assert_eq!(
            vec![
                ((8, 0), (0, 8)),
                ((6, 4), (2, 0)),
                ((0, 0), (8, 8)),
                ((5, 5), (8, 2)),
            ],
            diagonal_lines(lines(example_input()))
        )
    }

    #[test]
    fn test_hor_vert_lines() {
        assert_eq!(
            vec![
                ((0, 9), (5, 9)),
                ((9, 4), (3, 4)),
                ((2, 2), (2, 1)),
                ((7, 0), (7, 4)),
                ((0, 9), (2, 9)),
                ((3, 4), (1, 4)),
            ],
            hor_vert_lines(lines(example_input()))
        );
    }

    #[test]
    fn test_map() {
        assert_eq!(
            ((0, 9), (0, 9)),
            map(&lines(example_input()))
        );
    }

    #[test]
    fn test_points() {
        assert_eq!(
            [0].repeat(100),
            points(map(&lines(example_input())))
        );
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!(Day {}.pt1(example_input()), "5")
    }

    #[test]
    fn test_pt1() {
        assert_eq!(Day {}.pt1(input()), "5576")
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!(Day {}.pt2(example_input()), "12")
    }

    #[test]
    fn test_pt2() {
        assert_eq!(Day {}.pt2(input()), "18144")
    }
}
