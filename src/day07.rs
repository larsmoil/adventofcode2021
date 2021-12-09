use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, inp: &str) -> String {
        align(inp, &FuelCost::Constant).to_string()
    }
    fn pt2(&self, inp: &str) -> String {
        align(inp, &FuelCost::Increasing).to_string()
    }
}

enum FuelCost {
    Constant,
    Increasing,
}

fn fuel_between(a: i64, b: i64, fuel_cost: &FuelCost) -> i64 {
    let distance = (a - b).abs();
    match fuel_cost {
        FuelCost::Constant => distance,
        FuelCost::Increasing => (distance + 1).pow(2) / 2 - (distance + 1) / 2,
    }
}

fn align(inp: &str, fuel_cost: &FuelCost) -> i64 {
    let mut coordinates: Vec<i64> = inp
        .split(',')
        .map(|v| v.parse::<i64>().unwrap())
        .collect();
    coordinates.sort_unstable();
    let coordinate_extremes: (i64, i64) = (*coordinates.first().unwrap(), *coordinates.last().unwrap());

    let mut costs: Vec<i64> = vec![];
    for candidate in coordinate_extremes.0..=coordinate_extremes.1 {
        costs.push(
            coordinates
                .iter()
                .map(|coordinate| fuel_between(*coordinate, candidate, fuel_cost))
                .sum()
        );
    }
    costs.sort_unstable();
    *costs.first().unwrap()
}

pub(crate) fn input() -> &'static str {
    include_str!("input07.txt").trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "16,1,2,0,4,2,7,1,2,14"
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!(Day {}.pt1(example_input()), "37")
    }

    #[test]
    fn test_pt1() {
        assert_eq!(Day {}.pt1(input()), "351901")
    }

    #[test]
    fn test_fuel_between() {
        assert_eq!(14, fuel_between(16, 2, &FuelCost::Constant));
        assert_eq!(1, fuel_between(1, 2, &FuelCost::Constant));
        assert_eq!(0, fuel_between(2, 2, &FuelCost::Constant));
        assert_eq!(2, fuel_between(0, 2, &FuelCost::Constant));
        assert_eq!(2, fuel_between(4, 2, &FuelCost::Constant));
        assert_eq!(0, fuel_between(2, 2, &FuelCost::Constant));
        assert_eq!(5, fuel_between(7, 2, &FuelCost::Constant));
        assert_eq!(1, fuel_between(1, 2, &FuelCost::Constant));
        assert_eq!(0, fuel_between(2, 2, &FuelCost::Constant));
        assert_eq!(12, fuel_between(14, 2, &FuelCost::Constant));

        assert_eq!(14, fuel_between(2, 16, &FuelCost::Constant));
        assert_eq!(1, fuel_between(2, 1, &FuelCost::Constant));
        assert_eq!(0, fuel_between(2, 2, &FuelCost::Constant));
        assert_eq!(2, fuel_between(2, 0, &FuelCost::Constant));
        assert_eq!(2, fuel_between(2, 4, &FuelCost::Constant));
        assert_eq!(0, fuel_between(2, 2, &FuelCost::Constant));
        assert_eq!(5, fuel_between(2, 7, &FuelCost::Constant));
        assert_eq!(1, fuel_between(2, 1, &FuelCost::Constant));
        assert_eq!(0, fuel_between(2, 2, &FuelCost::Constant));
        assert_eq!(12, fuel_between(2, 14, &FuelCost::Constant));


        assert_eq!(66, fuel_between(16, 5, &FuelCost::Increasing));
        assert_eq!(10, fuel_between(1, 5, &FuelCost::Increasing));
        assert_eq!(6, fuel_between(2, 5, &FuelCost::Increasing));
        assert_eq!(15, fuel_between(0, 5, &FuelCost::Increasing));
        assert_eq!(1, fuel_between(4, 5, &FuelCost::Increasing));
        assert_eq!(6, fuel_between(2, 5, &FuelCost::Increasing));
        assert_eq!(3, fuel_between(7, 5, &FuelCost::Increasing));
        assert_eq!(10, fuel_between(1, 5, &FuelCost::Increasing));
        assert_eq!(6, fuel_between(2, 5, &FuelCost::Increasing));
        assert_eq!(45, fuel_between(14, 5, &FuelCost::Increasing));

        assert_eq!(66, fuel_between(5, 16, &FuelCost::Increasing));
        assert_eq!(10, fuel_between(5, 1, &FuelCost::Increasing));
        assert_eq!(6, fuel_between(5, 2, &FuelCost::Increasing));
        assert_eq!(15, fuel_between(5, 0, &FuelCost::Increasing));
        assert_eq!(1, fuel_between(5, 4, &FuelCost::Increasing));
        assert_eq!(6, fuel_between(5, 2, &FuelCost::Increasing));
        assert_eq!(3, fuel_between(5, 7, &FuelCost::Increasing));
        assert_eq!(10, fuel_between(5, 1, &FuelCost::Increasing));
        assert_eq!(6, fuel_between(5, 2, &FuelCost::Increasing));
        assert_eq!(45, fuel_between(5, 14, &FuelCost::Increasing));
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!(Day {}.pt2(example_input()), "168")
    }

    #[test]
    fn test_pt2() {
        assert_eq!(Day {}.pt2(input()), "101079875")
    }
}
