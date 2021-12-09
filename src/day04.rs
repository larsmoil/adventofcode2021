use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, inp: &str) -> String {
        let mut game = Game::new(inp);
        match game.solve() {
            None => 0,
            Some(scores) => scores.0
        }.to_string()
    }
    fn pt2(&self, inp: &str) -> String {
        let mut game = Game::new(inp);
        match game.solve() {
            None => 0,
            Some(scores) => scores.1
        }.to_string()
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("input04.txt").trim()
}

#[derive(Debug, Eq, PartialEq)]
struct Board {
    numbers: Vec<(u32, bool)>,
    solved: bool,
}

impl Board {
    fn draw(&mut self, number: u32) -> Option<u32> {
        let my_number = self.numbers.iter().enumerate().find(|(_, e)| e.0 == number);
        if let Some(my_number) = my_number {
            let index = my_number.0;
            self.numbers[index].1 = true;
        }

        self.solved = self.solved || vec![self.cols(), self.rows()]
            .iter()
            .flat_map(|a| a.iter())
            .map(|v| v.to_owned())
            .filter(|row_or_col| row_or_col.iter().all(|n| n.1))
            .count() > 0;

        self.score()
    }

    fn dimensions(&self) -> u32 {
        (self.numbers.len() as f64).sqrt() as u32
    }

    fn score(&mut self) -> Option<u32> {
        if self.solved {
            let non_drawn: Vec<u32> = self.numbers
                .iter()
                .filter(|(_, drawn)| !*drawn)
                .map(|(v, _)| *v)
                .collect();
            Option::Some(non_drawn.iter().sum())
        } else {
            Option::None
        }
    }

    fn rows(&self) -> Vec<Vec<(u32, bool)>> {
        self.numbers.chunks(self.dimensions() as usize).map(|chunk| chunk.to_vec()).collect()
    }

    fn cols(&self) -> Vec<Vec<(u32, bool)>> {
        let mut cols: Vec<Vec<(u32, bool)>> = vec![];
        for column in 0..self.dimensions() {
            cols.push(self.numbers.iter().skip(column as usize).step_by(self.dimensions() as usize).copied().collect());
        }
        cols
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Game {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

impl Game {
    fn solve(&mut self) -> Option<(u32, u32)> {
        let mut winner: Option<u32> = Option::None;
        for number in &self.numbers {
            for i in 0..self.boards.len() {
                let board = self.boards.get_mut(i).unwrap();
                let board_score = board.draw(*number);
                let unfinished_boards = self.boards.iter().filter(|board| !board.solved).count();

                match board_score {
                    None => {}
                    Some(score) => match winner {
                        None => winner = Option::Some(number * score),
                        Some(_) => if unfinished_boards == 0 {
                            return Some((winner.unwrap(), number * score));
                        }
                    }
                }
            }
        }
        Option::None
    }

    fn new(inp: &str) -> Game {
        let (numbers, boards) = inp.split_once("\n\n").unwrap();
        let numbers = numbers
            .split(',')
            .map(|v| v.parse::<u32>().unwrap())
            .collect();
        let board_numbers: Vec<Vec<(u32, bool)>> = boards
            .split("\n\n")
            .map(|board| {
                let board_numbers: Vec<(u32, bool)> = board
                    .split('\n')
                    .map(|row|
                        row
                            .split_whitespace()
                            .map(|num| (num.parse::<u32>().unwrap(), false))
                    )
                    .flatten()
                    .collect();
                board_numbers
            })
            .collect();
        let mut boards: Vec<Board> = Vec::new();
        for board in board_numbers {
            boards.push(Board { numbers: board, solved: false });
        }
        Game {
            numbers,
            boards,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
    }

    fn board1() -> Board {
        Board {
            numbers: vec![
                (22, false), (13, false), (17, false), (11, false), (0, false),
                (8, false), (2, false), (23, false), (4, false), (24, false),
                (21, false), (9, false), (14, false), (16, false), (7, false),
                (6, false), (10, false), (3, false), (18, false), (5, false),
                (1, false), (12, false), (20, false), (15, false), (19, false),
            ],
            solved: false,
        }
    }

    fn board2() -> Board {
        Board {
            numbers: vec![
                (3, false), (15, false), (0, false), (2, false), (22, false),
                (9, false), (18, false), (13, false), (17, false), (5, false),
                (19, false), (8, false), (7, false), (25, false), (23, false),
                (20, false), (11, false), (10, false), (24, false), (4, false),
                (14, false), (21, false), (16, false), (12, false), (6, false),
            ],
            solved: false,
        }
    }

    fn board3() -> Board {
        Board {
            numbers: vec![
                (14, false), (21, false), (17, false), (24, false), (4, false),
                (10, false), (16, false), (15, false), (9, false), (19, false),
                (18, false), (8, false), (23, false), (26, false), (20, false),
                (22, false), (11, false), (13, false), (6, false), (5, false),
                (2, false), (0, false), (12, false), (3, false), (7, false),
            ],
            solved: false,
        }
    }

    #[test]
    fn test_board_dimensions() {
        assert_eq!(5, board1().dimensions());
    }

    #[test]
    fn test_board_rows() {
        assert_eq!(vec![
            vec![(22, false), (13, false), (17, false), (11, false), (0, false)],
            vec![(8, false), (2, false), (23, false), (4, false), (24, false)],
            vec![(21, false), (9, false), (14, false), (16, false), (7, false)],
            vec![(6, false), (10, false), (3, false), (18, false), (5, false)],
            vec![(1, false), (12, false), (20, false), (15, false), (19, false)],
        ], board1().rows());
    }

    #[test]
    fn test_board_cols() {
        let expected = vec![
            vec![(22, false), (8, false), (21, false), (6, false), (1, false)],
            vec![(13, false), (2, false), (9, false), (10, false), (12, false)],
            vec![(17, false), (23, false), (14, false), (3, false), (20, false)],
            vec![(11, false), (4, false), (16, false), (18, false), (15, false)],
            vec![(0, false), (24, false), (7, false), (5, false), (19, false)],
        ];
        assert_eq!(expected, board1().cols());
    }

    #[test]
    fn test_board_draw() {
        let mut board = Board {
            numbers: vec![
                (22, false), (13, false), (17, false), (11, false), (0, false),
                (8, false), (2, true), (23, true), (4, true), (24, true),
                (21, true), (9, false), (14, true), (16, true), (7, true),
                (6, false), (10, false), (3, false), (18, false), (5, false),
                (1, false), (12, false), (20, false), (15, false), (19, false),
            ],
            solved: false,
        };
        assert_eq!(false, board.solved);
        board.draw(8);
        assert_eq!(true, board.solved);
    }

    #[test]
    fn test_game_new() {
        let actual = Game::new(example_input());
        let expected = Game {
            numbers: vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3, 26, 1],
            boards: vec![
                board1(),
                board2(),
                board3(),
            ],
        };
        assert_eq!(actual.numbers, expected.numbers);
        assert_eq!(actual.boards, expected.boards);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_game_solve() {
        let actual = Game::new(example_input()).solve();
        assert_eq!(actual, Some((4512, 1924)));
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!(Day {}.pt1(example_input()), "4512")
    }

    #[test]
    fn test_pt1() {
        assert_eq!(Day {}.pt1(input()), "65325")
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!(Day {}.pt2(example_input()), "1924")
    }

    #[test]
    fn test_pt2() {
        assert_eq!(Day {}.pt2(input()), "4624")
    }
}
