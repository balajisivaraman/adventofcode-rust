use std::{cell::RefCell, collections::HashMap};

#[derive(Debug, PartialEq)]
enum State {
    Marked,
    Unmarked,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Cell {
    num: i32,
    state: State,
}

impl Cell {
    fn new(num: i32) -> Cell {
        Cell {
            num,
            state: State::Unmarked,
        }
    }

    fn mark(&mut self) {
        self.state = State::Marked
    }

    fn is_marked(&self) -> bool {
        self.state == State::Marked
    }
}

#[derive(Debug)]
struct Board {
    internal: HashMap<Position, RefCell<Cell>>,
}

impl Board {
    fn new(lines: &[&String]) -> Board {
        let mut internal: HashMap<Position, RefCell<Cell>> = HashMap::new();
        let mut row = 0;
        for line in lines {
            let mut col = 0;
            for num in line.trim().split_whitespace() {
                let cell = Cell::new(i32::from_str_radix(num, 10).unwrap());
                let pos = Position { row, col };
                internal.insert(pos, RefCell::new(cell));
                col += 1;
            }
            row += 1;
        }
        Board { internal }
    }

    fn mark_number_as_seen(&self, num: i32) {
        if let Some(cell) = self.internal.values().find(|v| v.borrow().num == num) {
            cell.borrow_mut().mark();
        }
    }

    fn is_row_filled(&self) -> bool {
        (0..5)
            .into_iter()
            .map(|col| {
                (0..5)
                    .into_iter()
                    .map(|row| Position { row, col })
                    .map(|p| self.internal.get(&p).unwrap())
                    .all(|c| c.borrow().is_marked())
            })
            .find(|b| *b)
            .unwrap_or(false)
    }

    fn is_col_filled(&self) -> bool {
        (0..5)
            .into_iter()
            .map(|row| {
                (0..5)
                    .into_iter()
                    .map(|col| Position { row, col })
                    .map(|p| self.internal.get(&p).unwrap())
                    .all(|c| c.borrow().is_marked())
            })
            .find(|b| *b)
            .unwrap_or(false)
    }

    fn sum_unmarked_numbers(&self) -> i32 {
        self.internal
            .values()
            .filter(|c| !c.borrow().is_marked())
            .map(|c| c.borrow().num)
            .sum()
    }
}

struct BingoResult<'a> {
    matched_board: &'a Board,
    matching_number: &'a i32,
}

#[derive(Debug)]
struct Bingo {
    numbers_to_be_drawn: Vec<i32>,
    boards: Vec<Board>,
}

impl Bingo {
    fn new(input: Vec<String>) -> Bingo {
        let numbers_to_be_drawn: Vec<i32> = input
            .get(0)
            .unwrap()
            .split(|c: char| c == ',')
            .into_iter()
            .map(|s| i32::from_str_radix(s, 10).unwrap())
            .collect();
        let boards: Vec<Board> = input[2..]
            .into_iter()
            .collect::<Vec<&String>>()
            .split(|s| *s == "")
            .map(|lines| Board::new(lines))
            .collect();
        Bingo {
            numbers_to_be_drawn,
            boards,
        }
    }

    fn play(&self) -> Result<BingoResult<'_>, String> {
        for num in self.numbers_to_be_drawn.iter() {
            self.boards.iter().for_each(|b| b.mark_number_as_seen(*num));
            if let Some(matched_board) = self
                .boards
                .iter()
                .find(|b| b.is_row_filled() || b.is_col_filled())
            {
                return Ok(BingoResult {
                    matched_board,
                    matching_number: num,
                });
            } else {
                continue;
            }
        }
        Err("No matching board found!!!".to_string())
    }
}

pub fn day04a(input: Vec<String>) -> i32 {
    let bingo = Bingo::new(input);
    let bingo_result = bingo.play().unwrap();
    bingo_result.matched_board.sum_unmarked_numbers() * bingo_result.matching_number
}

pub fn day04b(_input: Vec<String>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day04a() {
        let input = vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "",
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ];
        assert_eq!(
            day04a(input.into_iter().map(|s| s.to_string()).collect()),
            4512
        );
    }
}
