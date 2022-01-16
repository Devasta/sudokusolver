fn main() {
    return run()
}

#[derive(Debug)]
enum BoardStatus {
    Solved,
    Incomplete,
    Invalid
}

fn validboard(sudokuboard: [[i8; 9]; 9]) -> BoardStatus{

    for r in 0..9{
        for c in 0..9{
            println!("{:?}",&sudokuboard[r][c]);
            if *&sudokuboard[r][c] != 0_i8 {
                println!("{:?}",&sudokuboard[r][c+1..9]);
                if sudokuboard[r][c+1..9].contains(&sudokuboard[r][c]){
                    return BoardStatus::Invalid
                } else if sudokuboard[r][c+1..9].contains(&sudokuboard[r][c]){

                }
            }
        }
    }
    return BoardStatus::Solved
}

fn run(){

    //let mut possible_solutions = vec![];
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

    let result = validboard(sudokuboard);
    println!("{:?}", result)
}