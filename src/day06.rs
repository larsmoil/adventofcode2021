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

#[derive(Debug, Eq, PartialEq)]
struct Fish {
    timer: i64,
    number: i64,
}

fn solve(days: i64, mut fish: Vec<Fish>) -> i64 {
    if days < 1 {
        fish
            .iter()
            .map(|f| f.number)
            .reduce(|a, b| a + b)
            .unwrap()
    } else {
        let new_fish: Option<Fish> = fish
            .iter()
            .filter(|fishy| fishy.timer == 0)
            .map(|fishy| Fish { timer: 8, number: fishy.number })
            .reduce(|a, b| Fish { timer: 8, number: a.number + b.number });
        for i in 0..fish.len() {
            let mut fishy = fish.get_mut(i).unwrap();
            let new_timer = fishy.timer - 1;
            let new_timer = if new_timer >= 0 { new_timer } else { 6 };
            fishy.timer = new_timer;
        }
        if let Some(new_fish) = new_fish {
            fish.push(new_fish)
        }

        solve(days - 1, fish)
    }
}

fn fish(inp: &str) -> Vec<Fish> {
    inp.split(',')
        .map(|v| v.parse::<i64>().unwrap())
        .map(|v| Fish { timer: v, number: 1 })
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
            vec![3, 4, 3, 1, 2].iter().map(|v| Fish { timer: *v, number: 1 }).collect::<Vec<Fish>>(),
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

    #[test]
    fn test_pt2_example() {
        assert_eq!(Day {}.pt2(example_input()), "26984457539")
    }

    #[test]
    fn test_pt2() {
        assert_eq!(Day {}.pt2(input()), "1682576647495")
    }
}
