impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (m, n) = (board.len(), board[0].len());
        // check perimeter for non-surrounded regions and mark them
        for i in 0..m {
            if board[i][0] == 'O' {Self::mark_region(i, 0, board);}
            if board[i][n-1] == 'O' {Self::mark_region(i, n-1, board);}
        }
        for j in 0..n {
            if board[0][j] == 'O' {Self::mark_region(0, j, board);}
            if board[m-1][j] == 'O' {Self::mark_region(m-1, j, board);}
        }
        // iterate over all cells, returning 'M's to 'O's and 'O's to 'X's
        for row in board {
            for cell in row {
                if *cell == 'M' {
                    *cell = 'O';
                } else if *cell == 'O' {
                    *cell = 'X';
                }
            }
        }
    }

    fn mark_region(i: usize, j: usize, board: &mut Vec<Vec<char>>) {
        let (m, n) = (board.len(), board[0].len());
        let mut unmarked = Vec::with_capacity(m);
        unmarked.push( (i, j) );
        while let Some( (i, j) ) = unmarked.pop() {
            board[i][j] = 'M';
            if i > 0 && board[i-1][j] == 'O' {unmarked.push( (i-1, j) );}
            if i < m-1 && board[i+1][j] == 'O' {unmarked.push( (i+1, j) );}
            if j > 0 && board[i][j-1] == 'O' {unmarked.push( (i, j-1) );}
            if j < n-1 && board[i][j+1] == 'O' {unmarked.push( (i, j+1) );}
        }
    }
}
