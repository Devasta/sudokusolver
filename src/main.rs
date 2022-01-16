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

    pub fn row(&self, r:usize) -> [i8; 9]{
        return self.board[r].clone()
    }

    pub fn col(&self, c:usize) -> [i8; 9]{
        return [
            self.board[0][c].clone(),
            self.board[1][c].clone(),
            self.board[2][c].clone(),
            self.board[3][c].clone(),
            self.board[4][c].clone(),
            self.board[5][c].clone(),
            self.board[6][c].clone(),
            self.board[7][c].clone(),
            self.board[8][c].clone()
        ]
    }

    pub fn valid(&self) -> BoardStatus {

        println!("rows");
        for r in 0..9{
            println!("{:?}", &self.row(r));
        }
        println!("cols");
        for c in 0..9{
            println!("{:?}", &self.col(c));
        }
        for r in 0..9{
            row = &self.row(r);
            for val in row{

            }

        }

            return BoardStatus::Solved
    }
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
            [9,7,2,5,3,8,6,4,0]
        ];

    let s = Sudoku::new(sudokuboard);

    let result = s.valid();
    println!("{:?}", result)
}