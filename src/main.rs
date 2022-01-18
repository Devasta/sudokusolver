mod sudokusolver;

fn main(){

    let sudokuboard:[[i8; 9]; 9]  =
        [
            [0,6,7,4,0,0,0,0,5],
            [0,0,4,0,5,0,7,0,0],
            [8,0,0,0,0,0,0,0,1],
            [6,0,3,0,0,1,0,0,0],
            [0,1,0,0,8,0,0,3,0],
            [0,0,0,6,0,0,2,0,7],
            [9,0,0,0,0,0,0,0,3],
            [0,0,8,0,7,0,4,0,0],
            [7,0,0,0,0,3,6,2,0]
        ];

    let result = crate::sudokusolver::sudokusolver::solve(sudokuboard);

    match result{
        Ok(r) => {
            println!("{:?}", r)
        }
        Err(e) => {
            println!("{:?}", e)
        }
    }
}