use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

// Bingo board structure
#[derive(Debug, Clone)]
struct BoardSquare {
    value: i32,
    marked: bool,
}

impl BoardSquare {
    pub fn mark_if_match(&mut self, value: i32) {
        if self.value == value {
            self.marked = true;
        }
    }
}

impl Display for BoardSquare {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let square = &self;
        let value = if square.value < 10 {
            format!("0{}", square.value)
        } else {
            format!("{}", square.value)
        };

        if square.marked {
            write!(f, "({})", value)
        } else {
           write!(f, " {} ", value)
        }
    }
}

#[derive(Debug, Clone)]
struct BoardRow {
    squares: Vec<BoardSquare>,
}

impl Display for BoardRow {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let row = &self;
        let mut row_string: String = String::new();

        for square in &row.clone().squares {
            row_string = format!("{} {}", row_string, square)
        }
        write!(f, "{}", row_string)
    }
}

#[derive(Debug, Clone)]
struct BingoBoard {
    name: String,
    rows: Vec<BoardRow>,
    has_won: bool,
}

impl Display for BingoBoard {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let board = &self;
        write!(f, "## ## ## ## ##\n");
        write!(f, "## {} ##\n", board.name);
        write!(f, "## ## ## ## ##");
        for row in &board.clone().rows {
            write!(f, "\n{}", row);
        }
        write!(f, "\n## ## ## ## ##")
    }
}


impl BingoBoard {
    fn mark_squares(&mut self, value: i32) {
        for row in &mut self.rows {
            for square in &mut row.squares {
                square.mark_if_match(value);
            }
        }
    }

    fn check_for_win(&mut self) -> bool {
        // check for row win
        for row in &self.rows {
            let mut squares = row.clone().squares;
            squares.retain(|s| s.marked == true);
            if squares.len() == 5 {
                self.has_won = true;
                return true
            }
        }

        // check for column win
        for i in 0..5 {
            let mut win = true;
            println!("{}",i);
            for row in &self.rows {
                if row.squares[i].marked == false {
                    win = false;
                }
            }

            if win {
                self.has_won = true;
                return true
            }
        }

        false
    }

    fn calculate_score(&self) -> i32 {
        let mut total: i32 = 0;

        for row in &self.rows {
            let mut squares = row.clone().squares;
            // Sum all squares
            squares.retain(|s| s.marked == false);
            total += squares.iter().map(|s| s.value).sum::<i32>();
        }

        total
    }
}

fn read_bingo_draws_from_file(path: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let file = BufReader::new(
        File::open(path)?
    );

    let line = file.lines().next().expect("No draws found in file").unwrap();
    let bingo_draws: Vec<i32> = line.split(',')
        .map(|val| val.parse::<i32>().unwrap())
        .collect();

    Ok(bingo_draws)
}

fn read_and_construct_bingo_boards_from_file(path: &str) -> Result<Vec<BingoBoard>, Box<dyn Error>> {
    let file = File::open(path).expect("no such file");
    let buf = BufReader::new(file);

    let lines: Vec<String> = buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let mut boards: Vec<BingoBoard> = vec![];

    for (index, chunk) in lines.chunks(6).enumerate() {
        let mut board = BingoBoard { name: format!("Board #{}", index+1), rows: vec![], has_won: false, };

        for line in chunk {
            let mut row = BoardRow { squares: vec![] };

            for part in line.split(' ') {
                if part == "" { continue };

                row.squares.push(BoardSquare {
                    value: part.parse::<i32>().expect("Could not parse to i32"),
                    marked: false,
                });
            }

            if row.squares.len() == 0 {
                continue
            }

            board.rows.push(row);
        }

        boards.push(board);
    }

    Ok(boards)
}

pub fn run() {
    println!("## Day 04: Bingov ##");
    println!("\nPart 1");
    part_1();

    println!("\nPart 2");
    part_2();
}

/**
 * Part 1: Regular bingo. First board to get a full row or column wins
 */
fn part_1() {
    println!("Welcome to Bingo!\n");
    let bingo_draws = read_bingo_draws_from_file("./days/day_04/data/bingo_draws.txt").unwrap();

    let mut bingo_boards = read_and_construct_bingo_boards_from_file("./days/day_04/data/bingo_boards.txt")
        .unwrap().clone();

    let mut winner = false;

    for drawn_number in bingo_draws {
        println!("Drawing number {}!\n", drawn_number);

        for bingo_board in &mut bingo_boards {
            bingo_board.mark_squares(drawn_number);

            let has_bingo = bingo_board.check_for_win();
            if has_bingo {
                println!("BINGOOO!! \n{}", bingo_board);
                println!("Final Score ({} * {}): {}\n", bingo_board.calculate_score(), drawn_number, bingo_board.calculate_score() * drawn_number);
                winner = true;
            }
        }

        if winner {
            break
        }
    }
}

/**
 * Part 2: Cheat bingo. Plays a full set of draws and wins to find the very last board to win, i. e. the losing board
 */
fn part_2() {
    println!("Welcome to Bingo!\n");
    let bingo_draws = read_bingo_draws_from_file("./days/day_04/data/bingo_draws.txt").unwrap();

    let mut bingo_boards = read_and_construct_bingo_boards_from_file("./days/day_04/data/bingo_boards.txt")
        .unwrap().clone();

    for drawn_number in bingo_draws {
        println!("Drawing number {}!\n", drawn_number);

        for bingo_board in &mut bingo_boards {
            bingo_board.mark_squares(drawn_number);

            let has_bingo = bingo_board.check_for_win();
            if has_bingo {
                println!("BINGOOO!! \n{}", bingo_board);
                println!("Final Score ({} * {}): {}\n", bingo_board.calculate_score(), drawn_number, bingo_board.calculate_score() * drawn_number);
            }
        }

        // Remove the board form the list after winning
        bingo_boards.retain(|b  | b.has_won == false);
        println!("Board count: {}", bingo_boards.len());

        if bingo_boards.len() == 0 {
            break;
        }
    }
}