use std::collections::HashMap;
use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, inp: &str) -> String {
        decode(inp)
            .iter()
            .map(|vec|
                vec
                    .iter()
                    .filter(|e| [1, 4, 7, 8].contains(e))
                    .count()
            )
            .sum::<usize>()
            .to_string()
    }
    fn pt2(&self, inp: &str) -> String {
        decode(inp)
            .iter()
            .map(|vec|
                vec.iter().map(|d| d.to_string()).collect::<String>().parse::<u32>().unwrap()
            )
            .sum::<u32>()
            .to_string()
    }
}

fn resolve_digit(digits: &HashMap<u32, String>, input: &str) -> Option<u32> {
    fn missing(haystack: Option<&String>, needle: &str) -> Option<u32> {
        haystack.map(|hay| needle.chars().filter(|c| hay.contains(*c)).count() as u32)
    }

    let missing_from_four = missing(digits.get(&4), input);
    let missing_from_seven = missing(digits.get(&7), input);

    match input.len() {
        2 => Some(1),
        4 => Some(4),
        3 => Some(7),
        7 => Some(8),
        6 => {
            if missing_from_seven == Some(2) {
                Some(6)
            } else if missing_from_four == Some(4) {
                Some(9)
            } else {
                Some(0)
            }
        }
        5 => {
            if missing_from_seven == Some(3) {
                Some(3)
            } else if missing_from_four == Some(3) {
                Some(5)
            } else {
                Some(2)
            }
        }
        _ => None,
    }
}

fn decode(inp: &str) -> Vec<Vec<u32>> {
    let entries: Vec<Vec<Vec<String>>> = inp
        .split('\n')
        .map(|line|
            line
                .split(" | ")
                .map(|entries| entries.split(' ').collect::<Vec<&str>>())
                .map(|v| v.iter().map(|s| {
                    let mut chars: Vec<char> = s.chars().collect();
                    chars.sort_unstable();
                    chars.into_iter().collect::<String>()
                }).collect())
                .collect::<Vec<Vec<String>>>()
        )
        .collect();

    let mut mapped_outputs: Vec<Vec<u32>> = vec![];
    for entry in &entries {
        let mut digits: HashMap<u32, String> = HashMap::new();
        let inputs = entry.first().unwrap();
        let outputs = entry.last().unwrap();
        for _ in 0..=1 {
            for input in inputs {
                let digit: Option<u32> = resolve_digit(&digits, input);
                if let Some(d) = digit {
                    let mut chars: Vec<char> = input.chars().collect();
                    chars.sort_unstable();
                    digits.insert(d, chars.into_iter().collect::<String>());
                }
            }
        }

        let mapped_output: Vec<u32> = outputs
            .iter()
            .map(|output| {
                digits.iter().find(|e| **e.1 == **output).unwrap().0
            })
            .copied()
            .collect();
        mapped_outputs.push(mapped_output);
    }

    mapped_outputs
}

pub(crate) fn input() -> &'static str {
    include_str!("input08.txt").trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!(Day {}.pt1(example_input()), "26")
    }

    #[test]
    fn test_pt1() {
        assert_eq!(Day {}.pt1(input()), "452")
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!(Day {}.pt2(example_input()), "61229")
    }

    #[test]
    fn test_pt2() {
        assert_eq!(Day {}.pt2(input()), "1096964")
    }
}
