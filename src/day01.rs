use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, inp: &str) -> String {
        let measurements = measurements(inp);
        let mut increased = 0;
        let mut last = measurements[0];
        for measurement in measurements {
            if last < measurement {
                increased += 1
            }
            last = measurement
        }
        increased.to_string()
    }
    fn pt2(&self, inp: &str) -> String {
        let measurements = measurements(inp);
        let windows: Vec<(i32, i32, i32)> = measurements
            .iter()
            .enumerate()
            .filter(|(i, _e)| *i > 0 && *i < measurements.len() - 1)
            .map(|(i, _e)| (measurements[i - 1], measurements[i], measurements[i + 1]))
            .collect();
        let sums: Vec<i32> = windows
            .iter()
            .map(|window| window.0 + window.1 + window.2)
            .collect();

        let mut increased = 0;
        let mut last = windows[0].0 + windows[0].1 + windows[0].2;
        for sum in sums {
            if last < sum {
                increased += 1
            }
            last = sum
        }
        increased.to_string()
    }
}

fn measurements(inp: &str) -> Vec<i32> {
    inp
        .split('\n')
        .map(|a| a.parse::<i32>().unwrap())
        .collect()
}

pub(crate) fn input() -> &'static str {
    include_str!("input01.txt").trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input_1() -> &'static str {
        "199
200
208
210
200
207
240
269
260
263"
    }

    fn example_input_2() -> &'static str {
        "607
618
618
617
647
716
769
792"
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!(Day {}.pt1(example_input_1()), "7")
    }

    #[test]
    fn test_pt1() {
        assert_eq!(Day {}.pt1(input()), "1715")
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!(Day {}.pt2(example_input_2()), "5")
    }

    #[test]
    fn test_pt2() {
        assert_eq!(Day {}.pt2(input()), "1739")
    }
}
