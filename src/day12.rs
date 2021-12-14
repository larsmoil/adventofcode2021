use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, inp: &str) -> String {
        Map::new(inp, false).go((false, vec![]), "start", "end").len().to_string()
    }
    fn pt2(&self, inp: &str) -> String {
        Map::new(inp, true).go((false, vec![]), "start", "end").len().to_string()
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("input12.txt").trim()
}

struct Cave {
    name: String,
    connections: Vec<String>,
}

impl Cave {
    fn connect(&mut self, cave: String) {
        self.connections.push(cave);
        self.connections.sort();
        self.connections.dedup();
    }
}

struct Map {
    caves: Vec<Cave>,
    allow_duplicate: bool,
}

impl Map {
    fn new(inp: &str, allow_duplicate: bool) -> Map {
        let mut map = Map { caves: vec![], allow_duplicate };
        for line in inp.split('\n') {
            let from_to = line.split('-').collect::<Vec<&str>>();
            let (from, to) = (from_to.first().unwrap(), from_to.last().unwrap());
            map.add(from.to_string());
            map.add(to.to_string());
            map.connect(from.to_string(), to.to_string());
            map.connect(to.to_string(), from.to_string());
        }
        map
    }

    fn add(&mut self, cave_name: String) {
        if !self.caves.iter().any(|c| c.name == cave_name) {
            self.caves.push(Cave { name: cave_name, connections: vec![] });
        }
    }

    fn connect(&mut self, cave_1: String, cave_2: String) {
        let first = self.caves.iter_mut().find(|c| c.name == cave_1).unwrap();
        first.connect(cave_2);
    }

    /// Get all possible paths from `from` to `to`.
    fn go(&self, path: (bool, Vec<String>), from: &str, to: &str) -> Vec<(bool, Vec<String>)> {
        let path = (path.0, vec![path.1, vec![from.to_string()]].concat());
        if from == to {
            vec![path]
        } else {
            let from_cave = self.caves.iter().find(|c| c.name == from).unwrap();
            from_cave.connections
                .iter()
                .filter(|neighbor| *neighbor != "start")
                .map(|neighbor| {
                    let is_big = neighbor.chars().all(char::is_uppercase);
                    if is_big {
                        Option::Some((path.clone(), neighbor))
                    } else {
                        let not_visited = !path.1.contains(neighbor);
                        if not_visited {
                            Option::Some((path.clone(), neighbor))
                        } else if self.allow_duplicate && !path.0 {
                            Option::Some(((true, path.1.clone()), neighbor))
                        } else {
                            Option::None
                        }
                    }
                })
                .flatten()
                .flat_map(|(path, neighbor)| self.go(path, neighbor, to))
                .collect::<Vec<(bool, Vec<String>)>>()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input_1() -> &'static str {
        "
start-A
start-b
A-c
A-b
b-d
A-end
b-end".trim()
    }

    fn example_input_2() -> &'static str {
        "
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc".trim()
    }

    fn example_input_3() -> &'static str {
        "
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW".trim()
    }

    #[test]
    fn test_pt1_example_1() {
        assert_eq!(Day {}.pt1(example_input_1()), "10")
    }

    #[test]
    fn test_pt1_example_2() {
        assert_eq!(Day {}.pt1(example_input_2()), "19")
    }

    #[test]
    fn test_pt1_example_3() {
        assert_eq!(Day {}.pt1(example_input_3()), "226")
    }

    #[test]
    fn test_pt1() {
        assert_eq!(Day {}.pt1(input()), "5874")
    }

    #[test]
    fn test_pt2_example_1() {
        assert_eq!(Day {}.pt2(example_input_1()), "36")
    }

    #[test]
    fn test_pt2_example_2() {
        assert_eq!(Day {}.pt2(example_input_2()), "103")
    }

    #[test]
    fn test_pt2_example_3() {
        assert_eq!(Day {}.pt2(example_input_3()), "3509")
    }

    #[test]
    fn test_pt2() {
        assert_eq!(Day {}.pt2(input()), "153592")
    }
}
