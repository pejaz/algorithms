/*
 * @lc app=leetcode.cn id=37 lang=rust
 *
 * [37] 解数独
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        fn is_valid(num: char, board: &mut Vec<Vec<char>>, row: usize, col: usize) -> bool {
            // 行+列
            for i in 0..9 {
                if board[row][i] == num || board[i][col] == num {
                    return false;
                }
            }

            // 3*3
            let row = (row / 3 | 0) * 3;
            let col = (col / 3 | 0) * 3;
            for i in 0..3 {
                for j in 0..3 {
                    if board[row + i][col + j] == num {
                        return false;
                    }
                }
            }

            return true;
        }

        fn backtrack(board: &mut Vec<Vec<char>>, nums: &[char; 9]) -> bool {
            // 不需要终止条件，棋盘填满就不会再走递归了

            // 这里每次都要从 0 开始，因为双层递归。
            // 如果由 start_row start_col 控制的话，如果 start_row, start_col..9 都不满足时，
            // 下一个数不会从 (start_row + 1, 0) 开始，而是从 (start_row , start_col)开始。
            // 这样会跳过一些空格顺序错乱，不是从左到右从上到下每个空格依次递归。

            for i in 0..9 {
                for j in 0..9 {
                    if board[i][j] == '.' {
                        for num in nums {
                            if is_valid(*num, board, i, j) {
                                board[i][j] = *num;

                                if backtrack(board, nums) {
                                    return true;
                                }

                                board[i][j] = '.';
                            }
                        }

                        // !!! 所有数字都不合法，表示当前空格无解，说明前面空格填错了，
                        // 此时需要返回 false 给上一层回溯
                        return false;
                    }
                }
            }

            return true;
        }

        let nums = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
        backtrack(board, &nums);
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
    fn test_solve_sudoku() {
        //assert_eq!(Solution::solve_sudoku(vec![]),[]);
        let board = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];

        fn is_solve_sudoku(board: &Vec<Vec<char>>) -> bool {
            if board.iter().any(|row| row.iter().any(|n| n == &'.')) {
                return false;
            }

            for i in 0..9 {
                // 行
                let mut vec1 = [0; 9];
                // 列
                let mut vec2 = [0; 9];
                for j in 0..9 {
                    let row = board[j][i] as usize - '1' as usize;
                    let col = board[i][j] as usize - '1' as usize;
                    if vec1[row] == 1 || vec2[col] == 1 {
                        return false;
                    } else {
                        vec1[row] = 1;
                        vec2[col] = 1;
                    }
                }

                // 3*3,从左到右；r:000111222，c:012012012
                let (r, c) = (i / 3, i % 3);
                let mut vec3 = [0; 9];
                for m in 0..3 {
                    for n in 0..3 {
                        let k = board[r * 3 + m][c * 3 + n] as usize - '1' as usize;
                        if vec3[k] == 1 {
                            return false;
                        } else {
                            vec3[k] = 1;
                        }
                    }
                }
            }

            return true;
        }

        let res = is_solve_sudoku(&board);
        assert_eq!(res, true);
        assert!(true)
    }
}
