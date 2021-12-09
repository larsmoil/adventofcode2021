use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, inp: &str) -> String {
        let vals = epsilon_gamma(inp);
        (vals.0 * vals.1).to_string()
    }
    fn pt2(&self, inp: &str) -> String {
        let vals = epsilon_gamma(inp);
        (vals.2 * vals.3).to_string()
    }
}

fn epsilon_gamma(inp: &str) -> (u32, u32, u32, u32) {
    let num_columns = inp.split('\n').map(|s| s.trim()).collect::<Vec<&str>>().first().unwrap().len();
    let lines = inp.split('\n').map(|s| s.trim());
    let values: Vec<u32> = lines
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect();

    let half_or_more_ones = |ones, size| if 2 * ones >= size as u32 { 1 } else { 0 };
    let half_or_more_zeros = |ones, size| if 2 * ones < size as u32 { 1 } else { 0 };

    let gamma = mask(&values, num_columns, half_or_more_ones);
    let epsilon = mask(&values, num_columns, half_or_more_zeros);
    let oxygen_generator_rating = rating(&values, num_columns, half_or_more_ones);
    let scrubber_rating = rating(&values, num_columns, half_or_more_zeros);

    (epsilon, gamma, oxygen_generator_rating, scrubber_rating)
}

fn rating(values: &[u32], column: usize, cmp: fn(u32, usize) -> u32) -> u32 {
    if values.len() == 1 || column == 0 {
        *values.first().unwrap()
    } else {
        let shift_by = column - 1;
        let shifts: Vec<(u32, u32)> = values
            .iter()
            .map(|v| {
                let shifted = (*v & (1 << shift_by)) >> shift_by;
                (shifted, *v)
            })
            .collect();
        let ones: u32 = shifts.iter().map(|v| v.0).sum();
        let should_equal = cmp(ones, values.len());
        let filtered: Vec<u32> = shifts
            .iter()
            .filter(|(shift, _)| *shift == should_equal)
            .map(|v| v.1)
            .collect();
        rating(&filtered, column - 1, cmp)
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("input03.txt").trim()
}

fn mask(values: &[u32], columns: usize, cmp: fn(num_ones: u32, num_values: usize) -> u32) -> u32 {
    if columns < 1 {
        0
    } else {
        let shift_by = columns - 1;
        let shifts: Vec<(u32, u32)> = values
            .iter()
            .map(|v| {
                let shifted: u32 = (*v & (1 << shift_by)) >> shift_by;
                (shifted, *v)
            })
            .collect();
        let ones: u32 = shifts.iter().map(|v| v.0).sum();
        let should_equal = cmp(ones, values.len());
        (should_equal << shift_by) + mask(values, columns - 1, cmp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
    }

    #[test]
    fn test_mask() {
        let lines = example_input().split('\n').map(|s| s.trim());
        let values: Vec<u32> = lines
            .map(|l| u32::from_str_radix(l, 2).unwrap())
            .collect();
        assert_eq!(
            mask(&values, 5, |num_ones, num_values| if (num_ones * 2) > num_values as u32 { 1 } else { 0 }),
            u32::from_str_radix("10110", 2).unwrap()
        );
    }

    #[test]
    fn test_epsilon_gamma() {
        assert_eq!(epsilon_gamma(example_input()), (9, 22, 23, 10))
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!(Day {}.pt1(example_input()), "198")
    }

    #[test]
    fn test_pt1() {
        assert_eq!(Day {}.pt1(input()), "2743844")
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!(Day {}.pt2(example_input()), "230")
    }

    #[test]
    fn test_pt2() {
        assert_eq!(Day {}.pt2(input()), "6677951")
    }
}
