/*
 * @lc app=leetcode.cn id=695 lang=rust
 *
 * [695] 岛屿的最大面积
 */

// @lc code=start

use std::collections::VecDeque;

#[allow(unused)]
impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        fn bfs(
            grid: &mut Vec<Vec<i32>>,
            (i, j): (usize, usize),
            (row, col): (usize, usize),
        ) -> i32 {
            // 上下左右
            let directs: [(i32, i32); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];
            let mut lands = 1; // (i,j)
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
                        && grid[x as usize][y as usize] == 1
                    {
                        // 面积加一
                        lands += 1;
                        // 标记为访问过
                        grid[x as usize][y as usize] = 2;
                        queue.push_back((x as usize, y as usize));
                    }
                }
            }

            return lands;
        }

        let mut area = 0;
        let row = grid.len();
        let col = grid[0].len();

        for i in 0..row {
            for j in 0..col {
                if grid[i][j] == 1 {
                    grid[i][j] = 2;
                    let lands = bfs(&mut grid, (i, j), (row, col));
                    area = area.max(lands);
                }
            }
        }

        return area;
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
    fn test_max_area_of_island() {
        //println!("res is {:?}", Solution::max_area_of_island(vec![]));
        //assert_eq!(Solution::max_area_of_island(vec![]),[]);
        assert!(true)
    }
}
