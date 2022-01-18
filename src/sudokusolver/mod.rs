
pub mod sudokusolver {

    enum BoardStatus {
        Solved,
        Incomplete,
        Invalid
    }

    fn row(board: &[[i8; 9];9], r:usize) -> [i8; 9]{
        return board[r].clone()
    }

    fn col(&board: &[[i8; 9];9], c:usize) -> [i8; 9]{
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
    fn squares(board: &[[i8; 9];9]) -> [[i8; 9];9] {
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

    fn get_empty_field(board: &[[i8;9]; 9]) -> (usize, usize ) {
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

    fn valid(board: [[i8;9];9]) -> BoardStatus {

        for r in 0..9{
            let row = &row(&board, r);
            for val in row {
                // Duplicate checking
                if *val != 0_i8 && row.into_iter().filter(|x| *x == val).count() > 1{
                    return BoardStatus::Invalid
                }
            }
        }
        for c in 0..9{
            let col = &col(&board, c);
            for val in col {
                // Duplicate checking
                if *val != 0_i8 && col.into_iter().filter(|x| *x == val).count() > 1{
                    return BoardStatus::Invalid
                }
            }
        }
        for square in &squares(&board) {
            for val in square {
                // Duplicate checking
                if *val != 0_i8 && square.into_iter().filter(|x| *x == val).count() > 1{
                    return BoardStatus::Invalid
                }
            }
        }

        if *&get_empty_field(&board) != (0_usize, 0_usize) {
            return BoardStatus::Incomplete
        } else {
            return BoardStatus::Solved
        }
    }

    pub fn solve(board: [[i8;9];9]) -> Result<[[i8; 9]; 9], &'static str> {

        let mut potential_solutions = Vec::new();

        potential_solutions.push(board);

        while potential_solutions.len() > 0 {

            match potential_solutions.pop() {
                None => {
                    return Err("Unsolvable")
                }
                Some(sudokuboard) => {
                    match valid(sudokuboard) {
                        BoardStatus::Solved => {
                            return Ok(sudokuboard)
                        }
                        BoardStatus::Incomplete => {
                            let nextfield = get_empty_field(&sudokuboard);

                            for v in 1..10 {
                                let mut new_potential_solution = &mut sudokuboard.clone();
                                new_potential_solution[nextfield.0][nextfield.1] = v;

                                potential_solutions.push(*new_potential_solution);

                            }
                            ()
                        }
                        BoardStatus::Invalid => {
                            //Ignore, move onto the next solution
                            ()
                        }
                    }
                }
            }
        }
        return Err("No Solution Found")

    }

}