use std::fmt::{Display, Formatter};
use crate::problem::Solver;
use pathfinding::prelude::dijkstra;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, inp: &str) -> String {
        let map = Map::new(inp);
        let (_path, risk) = map.navigate(0, map.risks.len() - 1);
        format!("{}", risk)
    }
    fn pt2(&self, inp: &str) -> String {
        let mut map = Map::new(inp);
        map.grow(5);
        let (_path, risk) = map.navigate(0, map.risks.len() - 1);
        format!("{}", risk)
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("input15.txt").trim()
}

#[derive(Debug, Eq, PartialEq)]
struct Map {
    risks: Vec<u32>,
    width: usize,
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let height = self.risks.len() / self.width;
        let str = (0..height).into_iter().map(|y| {
            (0..self.width).into_iter().map(|x| {
                let index = self.index(x, y, &self.risks, self.width).unwrap();
                self.risks[index].to_string()
            }).collect::<String>()
        }).collect::<Vec<String>>().join("\n");
        write!(f, "{}", str)
    }
}

impl Map {
    fn new(inp: &str) -> Map {
        let lines = inp.split('\n').collect::<Vec<&str>>();
        let width = lines.first().unwrap().len();
        let risks = lines
            .iter()
            .map(|line| line.chars().map(|c| c.to_string().parse::<u32>().unwrap()).collect::<Vec<u32>>())
            .collect::<Vec<Vec<u32>>>()
            .concat();
        Map {
            risks,
            width,
        }
    }

    fn grow(&mut self, times: usize) {
        let height = self.risks.len() / self.width;
        let height_new = height * times;
        let width_new = self.width * times;
        let mut risks: Vec<u32> = vec![0; self.risks.len() * times.pow(2)];
        for target_x in 0..width_new {
            for target_y in 0..height_new {
                let source_x = target_x % self.width;
                let source_y = target_y % height;
                let source_index = self.index(source_x, source_y, &self.risks, self.width).unwrap();
                let source_risk = self.risks[source_index];
                let target_index = self.index(target_x, target_y, &risks, width_new).unwrap();
                let target_risk = source_risk + (target_x as u32) / (self.width as u32) + (target_y as u32) / (height as u32);
                let target_risk = if target_risk > 9 {
                    target_risk % 9
                } else {
                    target_risk
                };
                risks[target_index] = target_risk;
            }
        }
        self.width = width_new;
        self.risks = risks;
    }

    fn navigate(&self, from: usize, to: usize) -> (Vec<usize>, u32) {
        dijkstra(&from,
                 |&index| self.adjacent(index).into_iter().map(|a| (a, self.risks[a])),
                 |&p| p == to,
        ).unwrap()
    }

    fn xy(&self, index: usize, width: usize) -> (usize, usize) {
        let x = index % width;
        let y = index / width;
        (x, y)
    }

    fn index(&self, x: usize, y: usize, risks: &[u32], width: usize) -> Option<usize> {
        let height = risks.len() / width;
        if x < width && y < height {
            let index = x + y * width;
            Some(index)
        } else {
            None
        }
    }

    fn adjacent(&self, index: usize) -> Vec<usize> {
        let (x, y) = self.xy(index, self.width);
        vec![(x, y + 1), (x + 1, y), (x, y - 1), (x - 1, y)]
            .into_iter()
            .map(|(px, py)| self.index(px, py, &self.risks, self.width))
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581".trim()
    }

    fn example_input_grown_2() -> &'static str {
        "
11637517422274862853
13813736722492484783
21365113283247622439
36949315694715142671
74634171118574528222
13191281372421239248
13599124212461123532
31254216394236532741
12931385212314249632
23119445813422155692
22748628533385973964
24924847833513595894
32476224394358733541
47151426715826253782
85745282229685639333
24212392483532341359
24611235323572234643
42365327415347643852
23142496323425351743
34221556924533266713".trim()
    }

    fn example_input_grown_5() -> &'static str {
        "
11637517422274862853338597396444961841755517295286
13813736722492484783351359589446246169155735727126
21365113283247622439435873354154698446526571955763
36949315694715142671582625378269373648937148475914
74634171118574528222968563933317967414442817852555
13191281372421239248353234135946434524615754563572
13599124212461123532357223464346833457545794456865
31254216394236532741534764385264587549637569865174
12931385212314249632342535174345364628545647573965
23119445813422155692453326671356443778246755488935
22748628533385973964449618417555172952866628316397
24924847833513595894462461691557357271266846838237
32476224394358733541546984465265719557637682166874
47151426715826253782693736489371484759148259586125
85745282229685639333179674144428178525553928963666
24212392483532341359464345246157545635726865674683
24611235323572234643468334575457944568656815567976
42365327415347643852645875496375698651748671976285
23142496323425351743453646285456475739656758684176
34221556924533266713564437782467554889357866599146
33859739644496184175551729528666283163977739427418
35135958944624616915573572712668468382377957949348
43587335415469844652657195576376821668748793277985
58262537826937364893714847591482595861259361697236
96856393331796741444281785255539289636664139174777
35323413594643452461575456357268656746837976785794
35722346434683345754579445686568155679767926678187
53476438526458754963756986517486719762859782187396
34253517434536462854564757396567586841767869795287
45332667135644377824675548893578665991468977611257
44961841755517295286662831639777394274188841538529
46246169155735727126684683823779579493488168151459
54698446526571955763768216687487932779859814388196
69373648937148475914825958612593616972361472718347
17967414442817852555392896366641391747775241285888
46434524615754563572686567468379767857948187896815
46833457545794456865681556797679266781878137789298
64587549637569865174867197628597821873961893298417
45364628545647573965675868417678697952878971816398
56443778246755488935786659914689776112579188722368
55172952866628316397773942741888415385299952649631
57357271266846838237795794934881681514599279262561
65719557637682166874879327798598143881961925499217
71484759148259586125936169723614727183472583829458
28178525553928963666413917477752412858886352396999
57545635726865674683797678579481878968159298917926
57944568656815567976792667818781377892989248891319
75698651748671976285978218739618932984172914319528
56475739656758684176786979528789718163989182927419
67554889357866599146897761125791887223681299833479".trim()
    }

    #[test]
    fn test_xy_example() {
        let map = Map::new(example_input());
        assert_eq!((0, 0), map.xy(0, map.width));
        assert_eq!((0, 1), map.xy(10, map.width));
        assert_eq!((9, 9), map.xy(99, map.width));
    }

    #[test]
    fn test_adjacent_example() {
        let map = Map::new(example_input());
        assert_eq!(vec![10, 1], map.adjacent(0));
        assert_eq!(vec![19, 8], map.adjacent(9));
        assert_eq!(vec![20, 11, 0], map.adjacent(10));
        assert_eq!(vec![35, 26, 15, 24], map.adjacent(25));
    }

    #[test]
    fn test_navigate_example() {
        let map = Map::new(example_input());
        assert_eq!(
            (vec![0, 10, 20, 21, 22, 23, 24, 25, 26, 36, 37, 47, 57, 58, 68, 78, 88, 89, 99], 40),
            map.navigate(0, 99)
        );
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!(Day {}.pt1(example_input()), "40")
    }

    #[test]
    fn test_pt1() {
        assert_eq!(Day {}.pt1(input()), "403")
    }

    #[test]
    fn test_grow_2() {
        let expected = Map::new(example_input_grown_2());
        let mut map = Map::new(example_input());
        map.grow(2);
        assert_eq!(map.to_string(), expected.to_string())
    }

    #[test]
    fn test_grow_5() {
        let expected = Map::new(example_input_grown_5());
        let mut map = Map::new(example_input());
        map.grow(5);
        assert_eq!(map.to_string(), expected.to_string())
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!(Day {}.pt2(example_input()), "315")
    }

    #[test]
    fn test_pt2() {
        assert_eq!(Day {}.pt2(input()), "2840")
    }
}
