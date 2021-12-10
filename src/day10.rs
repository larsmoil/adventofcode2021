use std::collections::HashMap;
use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, inp: &str) -> String {
        let score_mapping: HashMap<char, u64> = [(')', 3), (']', 57), ('}', 1197), ('>', 25137)].into();

        inp
            .split('\n')
            .map(|line| (line, chunks(line)))
            .filter(|(line, (chunks, _))| line.len() > chunks.len())
            .map(|(_, (chunks, _))| chunks.chars().last().unwrap())
            .map(|character| *score_mapping.get(&character).unwrap())
            .sum::<u64>()
            .to_string()
    }
    fn pt2(&self, inp: &str) -> String {
        let score_mapping: HashMap<char, u64> = [(')', 1), (']', 2), ('}', 3), ('>', 4)].into();

        let mut scores: Vec<u64> = inp
            .split('\n')
            .map(|line| (line, chunks(line)))
            .filter(|(line, (chunks, _))| chunks.len() == line.len())
            .map(|(_, (_, completion))| completion)
            .map(|completion| completion
                .iter()
                .map(|character| *score_mapping.get(character).unwrap())
                .fold(0, |acc, score| acc * 5 + score))
            .collect();
        scores.sort_unstable();
        scores[scores.len() / 2].to_string()
    }
}

fn chunks(line: &str) -> (&str, Vec<char>) {
    let mut chunks_starts = vec![];
    let markers: HashMap<char, char> = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')].into();
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let char = chars[i];
        i += 1;
        if markers.contains_key(&char) {
            chunks_starts.push(char);
        } else if markers.values().any(|c| *c == char) {
            let chunk_start = chunks_starts.pop().unwrap();
            let expected = *markers.get(&chunk_start).unwrap();
            if char != expected {
                break;
            }
        }
    }
    (
        &line[..i],
        chunks_starts.iter().rev().map(|start| *markers.get(start).unwrap()).collect()
    )
}

pub(crate) fn input() -> &'static str {
    include_str!("input10.txt").trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]".trim()
    }

    #[test]
    fn test_chunks() {
        assert_eq!(
            chunks("[({(<(())[]>[[{[]{<()<>>"),
            ("[({(<(())[]>[[{[]{<()<>>", vec!['}', '}', ']', ']', ')', '}', ')', ']'])
        );
        assert_eq!(
            chunks("[(()[<>])]({[<{<<[]>>("),
            ("[(()[<>])]({[<{<<[]>>(", vec![')', '}', '>', ']', '}', ')'])
        );
        assert_eq!(
            chunks("(((({<>}<{<{<>}{[]{[]{}"),
            ("(((({<>}<{<{<>}{[]{[]{}", vec!['}', '}', '>', '}', '>', ')', ')', ')', ')'])
        );
        assert_eq!(
            chunks("{<[[]]>}<{[{[{[]{()[[[]"),
            ("{<[[]]>}<{[{[{[]{()[[[]", vec![']', ']', '}', '}', ']', '}', ']', '}', '>'])
        );
        assert_eq!(
            chunks("<{([{{}}[<[[[<>{}]]]>[]]"),
            ("<{([{{}}[<[[[<>{}]]]>[]]", vec![']', ')', '}', '>'])
        );

        assert_eq!(
            chunks("{([(<{}[<>[]}>{[]{[(<()>"),
            ("{([(<{}[<>[]}", vec!['>', ')', ']', ')', '}'])
        );
        assert_eq!(
            chunks("[[<[([]))<([[{}[[()]]]"),
            ("[[<[([]))", vec!['>', ']', ']'])
        );
        assert_eq!(
            chunks("[{[{({}]{}}([{[{{{}}([]"),
            ("[{[{({}]", vec!['}', ']', '}', ']'])
        );
        assert_eq!(
            chunks("[<(<(<(<{}))><([]([]()"),
            ("[<(<(<(<{})", vec![')', '>', ')', '>', ')', '>', ']'])
        );
        assert_eq!(
            chunks("<{([([[(<>()){}]>(<<{{"),
            ("<{([([[(<>()){}]>", vec![')', ']', ')', '}', '>'])
        );
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!(Day {}.pt1(example_input()), "26397")
    }

    #[test]
    fn test_pt1() {
        assert_eq!(Day {}.pt1(input()), "339411")
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!(Day {}.pt2(example_input()), "288957")
    }

    #[test]
    fn test_pt2() {
        assert_eq!(Day {}.pt2(input()), "2289754624")
    }
}
