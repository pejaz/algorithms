/*
 * @lc app=leetcode.cn id=200 lang=rust
 *
 * [200] 岛屿数量
 */

// @lc code=start

use std::collections::VecDeque;

#[allow(unused)]
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        fn bfs(grid: &mut Vec<Vec<char>>, (i, j): (usize, usize), (row, col): (usize, usize)) {
            // 上下左右
            let directs: [(i32, i32); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];
            let mut queue = VecDeque::new();
            queue.push_back((i, j));

            while !queue.is_empty() {
                let (i, j) = queue.pop_front().unwrap();
                for (x, y) in directs {
                    let x = i as i32 + x;
                    let y = j as i32 + y;

                    // bfs遍历节点周围
                    if (x >= 0 && x < row as i32)
                        && (y >= 0 && y < col as i32)
                        && grid[x as usize][y as usize] == '1'
                    {
                        grid[x as usize][y as usize] = '2';
                        queue.push_back((x as usize, y as usize));
                    }
                }
            }
        }

        let mut lanscount = 0;
        let row = grid.len();
        let col = grid[0].len();

        for i in 0..row {
            for j in 0..col {
                if grid[i][j] == '1' {
                    grid[i][j] = '2';
                    bfs(&mut grid, (i, j), (row, col));
                    lanscount += 1;
                }
            }
        }

        return lanscount;
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
    fn test_num_islands() {
        //println!("res is {:?}", Solution::num_islands(vec![]));
        //assert_eq!(Solution::num_islands(vec![]),[]);
        assert!(true)
    }
}
