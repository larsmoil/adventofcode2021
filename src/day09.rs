use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, inp: &str) -> String {
        let map = map(inp);
        let low_points = low_points(&map);
        low_points.iter().map(|p| map.values[*p as usize] + 1).sum::<i16>().to_string()
    }
    fn pt2(&self, inp: &str) -> String {
        let basins = basins(&map(inp));
        let mut sizes: Vec<usize> = basins.iter().map(|basin| basin.len()).collect();
        sizes.sort();
        sizes.reverse();
        sizes[..3].iter().product::<usize>().to_string()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Map {
    values: Vec<i16>,
    width: i16,
    height: i16,
}

fn adjacent(row: i16, column: i16, map: &Map) -> Vec<i16> {
    let mut candidates: Vec<i16> = vec![];
    if row > 0 {
        let north = (row - 1) * map.width + column;
        candidates.push(north);
    }
    if column < (map.width - 1) {
        let east = row * map.width + column + 1;
        candidates.push(east);
    }
    if row < (map.height - 1) {
        let south = (row + 1) * map.width + column;
        candidates.push(south);
    }
    if column > 0 {
        let west = row * map.width + column - 1;
        candidates.push(west);
    }

    candidates
}

fn basin(map: &Map, point: i16) -> Vec<i16> {
    let val = map.values[point as usize];
    let (row, column) = row_column(map, &(point as usize));
    let mut basin = vec![
        vec![point],
        adjacent(row, column, map)
            .iter()
            .filter(|a| map.values[**a as usize] < 9 && map.values[**a as usize] > val)
            .flat_map(|a| basin(map, *a))
            .collect(),
    ].concat();
    basin.sort();
    basin.dedup();
    basin
}

fn basins(map: &Map) -> Vec<Vec<i16>> {
    low_points(map)
        .iter()
        .map(|p| basin(map, *p))
        .collect()
}

fn low_points(map: &Map) -> Vec<i16> {
    map
        .values
        .iter()
        .enumerate()
        .filter(|(i, e)| {
            let (row, column) = row_column(&map, i);
            let adjacent = adjacent(row, column, &map);
            adjacent.iter().all(|a| map.values[*a as usize] > **e)
        })
        .map(|(i, _)| i as i16)
        .collect()
}

fn row_column(map: &Map, point: &usize) -> (i16, i16) {
    (*point as i16 / map.width, *point as i16 % map.width)
}

fn map(inp: &str) -> Map {
    let lines: Vec<&str> = inp.split("\n").collect();
    let height = lines.len() as i16;
    let width = lines.first().unwrap().len() as i16;
    let values: Vec<i16> = lines.iter().flat_map(|s| s.chars().map(|c| i16::from_str_radix(&c.to_string(), 10).unwrap()).collect::<Vec<i16>>()).collect();
    Map {
        width,
        height,
        values,
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("input09.txt").trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "
2199943210
3987894921
9856789892
8767896789
9899965678".trim()
    }

    #[test]
    fn test_basins_example() {
        assert_eq!(
            vec![
                vec![0, 1, 10],
                vec![5, 6, 7, 8, 9, 16, 18, 19, 29],
                vec![12, 13, 14, 21, 22, 23, 24, 25, 30, 31, 32, 33, 34, 41],
                vec![27, 36, 37, 38, 45, 46, 47, 48, 49],
            ],
            basins(&map(example_input()))
        );
    }

    #[test]
    fn test_adjacent_example() {
        assert_eq!(vec![1, 10], adjacent(0, 0, &map(example_input())));
        assert_eq!(vec![2, 11, 0], adjacent(0, 1, &map(example_input())));
    }

    #[test]
    fn test_low_points_example() {
        assert_eq!(
            low_points(&map(example_input())),
            vec![1, 9, 22, 46]
        )
    }

    #[test]
    fn test_map_example() {
        assert_eq!(
            map(example_input()),
            Map {
                height: 5,
                width: 10,
                values: vec![
                    2, 1, 9, 9, 9, 4, 3, 2, 1, 0,
                    3, 9, 8, 7, 8, 9, 4, 9, 2, 1,
                    9, 8, 5, 6, 7, 8, 9, 8, 9, 2,
                    8, 7, 6, 7, 8, 9, 6, 7, 8, 9,
                    9, 8, 9, 9, 9, 6, 5, 6, 7, 8,
                ],
            }
        )
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!(Day {}.pt1(example_input()), "15")
    }

    #[test]
    fn test_pt1() {
        assert_eq!(Day {}.pt1(input()), "631")
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!(Day {}.pt2(example_input()), "1134")
    }

    #[test]
    fn test_pt2() {
        assert_eq!(Day {}.pt2(input()), "821560")
    }
}
