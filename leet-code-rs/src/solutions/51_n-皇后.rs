/*
 * @lc app=leetcode.cn id=51 lang=rust
 *
 * [51] N 皇后
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        fn is_valid(chessboard: &Vec<Vec<char>>, row: i32, col: i32) -> bool {
            // 每一列
            for i in 0..row {
                if chessboard[i as usize][col as usize] == 'Q' {
                    return false;
                }
            }

            // 左上角
            for (i, j) in (0..row).rev().zip((0..col).rev()) {
                if chessboard[i as usize][j as usize] == 'Q' {
                    return false;
                }
            }

            // 右上角
            let n = chessboard.len();
            for (i, j) in (0..row).rev().zip((col + 1..n as i32)) {
                if chessboard[i as usize][j as usize] == 'Q' {
                    return false;
                }
            }

            return true;
        }
        fn backtracking(
            n: i32,
            row: i32,
            chessboard: &mut Vec<Vec<char>>,
            res: &mut Vec<Vec<String>>,
        ) {
            if row == n {
                let chess_str = chessboard
                    .iter()
                    .map(|row| row.iter().collect::<String>())
                    .collect();
                res.push(chess_str);
                return;
            }

            for i in 0..n {
                if is_valid(chessboard, row, i) {
                    chessboard[row as usize][i as usize] = 'Q';
                    backtracking(n, row + 1, chessboard, res);
                    chessboard[row as usize][i as usize] = '.';
                }
            }
        }

        let mut chessboard = vec![vec!['.'; n as usize]; n as usize];
        let mut res = vec![];

        backtracking(n, 0, &mut chessboard, &mut res);

        return res;
    }
}
// @lc code=end

#[allow(unused)]
struct Solution;

#[cfg(test)]
mod test {
    #[allow(unused)]
    use super::*;

    #[test]
    fn test_solve_n_queens() {
        //assert_eq!(Solution::solve_n_queens(vec![]),[]);
        assert!(true)
    }
}
