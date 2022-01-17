fn main() {
    return run()
}

#[derive(Debug)]
pub enum BoardStatus {
    Solved,
    Incomplete,
    Invalid
}

pub struct Sudoku {
    board: [[i8; 9]; 9]
}

impl Sudoku {

    pub fn new(board: [[i8; 9]; 9]) -> Sudoku {
        return Sudoku {
            board: board
        }
    }

    pub fn row(board: &[[i8; 9];9], r:usize) -> [i8; 9]{
        return board[r].clone()
    }

    pub fn col(&board: &[[i8; 9];9], c:usize) -> [i8; 9]{
        return [
            board[0][c].clone(),
            board[1][c].clone(),
            board[2][c].clone(),
            board[3][c].clone(),
            board[4][c].clone(),
            board[5][c].clone(),
            board[6][c].clone(),
            board[7][c].clone(),
            board[8][c].clone()
        ]
    }

    // TODO make something proper for this, want to support 4x4 eventually
    pub fn squares(board: &[[i8; 9];9]) -> [[i8; 9];9] {
        return [
            [
                board[0][0].clone(),
                board[1][0].clone(),
                board[2][0].clone(),
                board[0][1].clone(),
                board[1][1].clone(),
                board[2][1].clone(),
                board[0][2].clone(),
                board[1][2].clone(),
                board[2][2].clone()
            ],
            [
                board[3][0].clone(),
                board[4][0].clone(),
                board[5][0].clone(),
                board[3][1].clone(),
                board[4][1].clone(),
                board[5][1].clone(),
                board[3][2].clone(),
                board[4][2].clone(),
                board[5][2].clone()
            ],
            [
                board[6][0].clone(),
                board[7][0].clone(),
                board[8][0].clone(),
                board[6][1].clone(),
                board[7][1].clone(),
                board[8][1].clone(),
                board[6][2].clone(),
                board[7][2].clone(),
                board[8][2].clone()
            ],
            [
                board[0][3].clone(),
                board[1][3].clone(),
                board[2][3].clone(),
                board[0][4].clone(),
                board[1][4].clone(),
                board[2][4].clone(),
                board[0][5].clone(),
                board[1][5].clone(),
                board[2][5].clone()
            ],
            [
                board[3][3].clone(),
                board[4][3].clone(),
                board[5][3].clone(),
                board[3][4].clone(),
                board[4][4].clone(),
                board[5][4].clone(),
                board[3][5].clone(),
                board[4][5].clone(),
                board[5][5].clone()
            ],
            [
                board[6][3].clone(),
                board[7][3].clone(),
                board[8][3].clone(),
                board[6][4].clone(),
                board[7][4].clone(),
                board[8][4].clone(),
                board[6][5].clone(),
                board[7][5].clone(),
                board[8][5].clone()
            ],
            [
                board[0][6].clone(),
                board[1][6].clone(),
                board[2][6].clone(),
                board[0][7].clone(),
                board[1][7].clone(),
                board[2][7].clone(),
                board[0][8].clone(),
                board[1][8].clone(),
                board[2][8].clone()
            ],
            [
                board[3][6].clone(),
                board[4][6].clone(),
                board[5][6].clone(),
                board[3][7].clone(),
                board[4][7].clone(),
                board[5][7].clone(),
                board[3][8].clone(),
                board[4][8].clone(),
                board[5][8].clone()
            ],
            [
                board[6][6].clone(),
                board[7][6].clone(),
                board[8][6].clone(),
                board[6][7].clone(),
                board[7][7].clone(),
                board[8][7].clone(),
                board[6][8].clone(),
                board[7][8].clone(),
                board[8][8].clone()
            ]
        ]
    }

    pub fn get_empty_field(board: &[[i8;9]; 9]) -> (usize, usize ) {
        // empty fields have 0 for now.
        for r in 0..9{
            for c in 0..9 {
                if *&board[r][c] == 0_i8{
                    return (r, c)
                }
            }
        }
        return (0,0)
    }

    pub fn valid(self, board: [[i8;9];9]) -> BoardStatus {

        for r in 0..9{
            let row = &self.row(&board, r);
            for val in row {
                // Duplicate checking
                if *val != 0_i8 && row.into_iter().filter(|x| *x == val).count() > 1{
                    return BoardStatus::Invalid
                }
            }
        }
        for c in 0..9{
            let col = &self.col(&board, c);
            for val in col {
                // Duplicate checking
                if *val != 0_i8 && col.into_iter().filter(|x| *x == val).count() > 1{
                    return BoardStatus::Invalid
                }
            }
        }
        for square in &self.squares(&board) {
            for val in square {
                // Duplicate checking
                if *val != 0_i8 && square.into_iter().filter(|x| *x == val).count() > 1{
                    return BoardStatus::Invalid
                }
            }
        }

        if *&self.get_empty_field(&board) != (0_usize, 0_usize) {
            return BoardStatus::Incomplete
        } else {
            return BoardStatus::Solved
        }
    }
    /*
    pub fn solve(self) -> [[i8; 9];9]{
        let possible_solutions = vector::new([[i8;9];9]);

        while self.valid() <>  {

        }
    } */
}
/*
fn validboard(sudokuboard: [[i8; 9]; 9]) -> BoardStatus{

    for r in 0..9{
        for c in 0..9{
            println!("{:?}",&sudokuboard[r][c]);
            if *&sudokuboard[r][c] != 0_i8 {
                println!("{:?}",&sudokuboard[r][c+1..9]);
                println!("{:?}",&sudokuboard[r+1..9][c]);
                if sudokuboard[r][c+1..9].contains(&sudokuboard[r][c]){
                    return BoardStatus::Invalid
                } else if sudokuboard[r][c+1..9].contains(&sudokuboard[r][c]){

                }
            }
        }
    }
    return BoardStatus::Solved
}
*/

fn run(){

    let sudokuboard:[[i8; 9]; 9]  =
        [
            [1,2,3,4,5,6,7,8,9],
            [4,5,6,7,8,9,1,2,3],
            [7,8,9,1,2,3,4,5,6],
            [2,1,4,3,6,5,8,9,7],
            [3,6,5,8,9,7,2,1,4],
            [8,9,7,2,1,4,3,6,5],
            [5,3,1,6,4,2,9,7,8],
            [6,4,8,9,7,1,5,3,2],
            [9,7,2,5,3,8,6,4,1]
        ];

    let s = Sudoku::new(sudokuboard);

    let result = s.valid(&sudokuboard);
    println!("{:?}", result)
}