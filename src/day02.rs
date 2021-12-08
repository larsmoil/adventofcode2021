use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, inp: &str) -> String {
        let coordinate = coordinate(inp);
        (coordinate.0 * coordinate.1).to_string()
    }
    fn pt2(&self, inp: &str) -> String {
        let coordinate = coordinate_with_aim(inp);
        (coordinate.0 * coordinate.1).to_string()
    }
}

fn course_adjustments(inp: &str) -> Vec<(i32, i32)> {
    inp
        .split("\n")
        .map(|a| {
            let split = a.split(" ").collect::<Vec<&str>>();
            let direction = split.first().unwrap();
            let amount = split.last().unwrap();
            let amount = amount.parse::<i32>().unwrap();
            match direction {
                &"forward" => (amount, 0),
                &"up" => (0, -amount),
                &"down" => (0, amount),
                _ => panic!("Unknown direction: '{}'!", direction)
            }
        })
        .collect()
}

fn coordinate(inp: &str) -> (i32, i32) {
    course_adjustments(inp)
        .iter()
        .fold((0, 0), |a, b| {
            (a.0 + b.0, a.1 + b.1)
        })
}

fn coordinate_with_aim(inp: &str) -> (i32, i32) {
    let coordinate = course_adjustments(inp)
        .iter()
        .fold((0, 0, 0), |a, b| {
            let aim = a.2 + b.1;
            (a.0 + b.0, a.1 + b.0 * aim, aim)
        });
    (coordinate.0, coordinate.1)
}

pub(crate) fn input() -> &'static str {
    include_str!("input02.txt").trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "forward 5
down 5
forward 8
up 3
down 8
forward 2"
    }

    #[test]
    fn test_course_adjustments_example() {
        assert_eq!(course_adjustments(example_input()), [
            (5, 0),
            (0, 5),
            (8, 0),
            (0, -3),
            (0, 8),
            (2, 0),
        ])
    }

    #[test]
    fn test_coordinate_example() {
        assert_eq!(coordinate(example_input()), (15, 10))
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!(Day {}.pt1(example_input()), "150")
    }

    #[test]
    fn test_pt1() {
        assert_eq!(Day {}.pt1(input()), "1580000")
    }

    #[test]
    fn test_coordinate_with_aim_example() {
        assert_eq!(coordinate_with_aim(example_input()), (15, 60))
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!(Day {}.pt2(example_input()), "900")
    }

    #[test]
    fn test_pt2() {
        assert_eq!(Day {}.pt2(input()), "1251263225")
    }
}
