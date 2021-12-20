use std::collections::HashMap;
use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, inp: &str) -> String {
        let mut polymerization_device = PolymerizationDevice::new(inp);
        let elements = polymerization_device.polymerize(10);

        let least = elements[0];
        let most = elements.iter().next_back().unwrap();

        format!("{:?}", most.0 - least.0)
    }
    fn pt2(&self, inp: &str) -> String {
        let mut polymerization_device = PolymerizationDevice::new(inp);
        let elements = polymerization_device.polymerize(40);

        let least = elements[0];
        let most = elements.iter().next_back().unwrap();

        format!("{:?}", most.0 - least.0)
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("input14.txt").trim()
}

#[derive(Debug)]
struct PolymerizationDevice {
    template: String,
    rules: HashMap<(char, char), char>,
}

impl PolymerizationDevice {
    fn new(inp: &str) -> PolymerizationDevice {
        let (template, pairs) = inp.split_once("\n\n").unwrap();
        let rules_inp = pairs
            .split('\n')
            .map(|line| line.split_once(" -> ").unwrap())
            .map(|(polymer, insert)| ((polymer.chars().next().unwrap(), polymer.chars().next_back().unwrap()), insert.chars().next().unwrap()))
            .collect::<Vec<((char, char), char)>>();
        let mut rules = HashMap::new();
        for rule in rules_inp {
            rules.insert(rule.0, rule.1);
        }
        PolymerizationDevice {
            template: template.to_string(),
            rules,
        }
    }

    fn polymerize(&mut self, steps: usize) -> Vec<(u64, char)> {
        let template = self.template.chars().collect::<Vec<char>>();
        let mut single_counts: HashMap<char, u64> = HashMap::new();
        let mut pair_counts: HashMap<(char, char), u64> = HashMap::new();
        let mut pair_counts_old: HashMap<(char, char), u64> = HashMap::new();

        single_counts.insert(template[0], 1);
        for i in 0..self.template.len() - 1 {
            *single_counts.entry(template[i + 1]).or_insert(0) += 1;
            let pair = (template[i], template[i + 1]);
            *pair_counts.entry(pair).or_insert(0) += 1;
        }

        for _ in 0..steps {
            std::mem::swap(&mut pair_counts, &mut pair_counts_old);
            pair_counts.clear();

            for (pair, count) in &pair_counts_old {
                let &to_insert = self.rules.get(pair).unwrap();
                *single_counts.entry(to_insert).or_insert(0) += count;

                let pair_left = (pair.0, to_insert);
                let pair_right = (to_insert, pair.1);

                *pair_counts.entry(pair_left).or_insert(0) += count;
                *pair_counts.entry(pair_right).or_insert(0) += count;
            }
        }

        let mut elements: Vec<(u64, char)> = vec![];
        for (element, count) in single_counts {
            elements.push((count, element));

        }
        elements.sort_by(|a, b| (a.0.cmp(&b.0)));
        elements
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C".trim()
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!(Day {}.pt1(example_input()), "1588")
    }

    #[test]
    fn test_pt1() {
        assert_eq!(Day {}.pt1(input()), "2657")
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!(Day {}.pt2(example_input()), "2188189693529")
    }

    #[test]
    fn test_pt2() {
        assert_eq!(Day {}.pt2(input()), "2911561572630")
    }
}
