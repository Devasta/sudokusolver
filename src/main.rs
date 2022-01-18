mod sudokusolver;
use crate::sudokusolver::sudokusolver::solve;

fn main(){

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

    let result = crate::sudokusolver::sudokusolver::solve(sudokuboard);
    println!("{:?}", result)
}