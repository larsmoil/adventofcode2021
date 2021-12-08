use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, inp: &str) -> String {
        let initial_fish = fish(inp);
        solve(80, initial_fish).to_string()
    }
    fn pt2(&self, inp: &str) -> String {
        let initial_fish = fish(inp);
        solve(256, initial_fish).to_string()
    }
}

fn solve(days: i64, mut fish: Vec<i64>) -> i64 {
    if days < 1 {
        fish.len() as i64
    } else {
        let new_fish: Vec<i64> = fish
            .iter()
            .filter(|fishy| **fishy == 0)
            .map(|_| 8)
            .collect();
        for i in 0..fish.len() {
            let fishy = fish[i];
            let new_val = fishy - 1;
            let new_val = if new_val >= 0 { new_val } else { 6 };
            fish[i] = new_val;
        }
        for fishy in new_fish {
            fish.push(fishy)
        }
        solve(days - 1, fish)
    }
}

fn fish(inp: &str) -> Vec<i64> {
    inp.split(",")
        .map(|v| i64::from_str_radix(v, 10).unwrap())
        .collect()
}

pub(crate) fn input() -> &'static str {
    include_str!("input06.txt").trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "3,4,3,1,2"
    }

    #[test]
    fn test_fish() {
        assert_eq!(
            vec![3, 4, 3, 1, 2],
            fish(example_input())
        )
    }

    #[test]
    fn test_solve_example_18_days() {
        assert_eq!(solve(18, fish(example_input())), 26)
    }

    #[test]
    fn test_solve_example_80_days() {
        assert_eq!(solve(80, fish(example_input())), 5934)
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!(Day {}.pt1(example_input()), "5934")
    }

    #[test]
    fn test_pt1() {
        assert_eq!(Day {}.pt1(input()), "373378")
    }

    // #[test]
    // fn test_pt2_example() {
    //     assert_eq!(Day {}.pt2(example_input()), "26984457539")
    // }
    //
    // #[test]
    // fn test_pt2() {
    //     assert_eq!(Day {}.pt2(input()), "!")
    // }
}
