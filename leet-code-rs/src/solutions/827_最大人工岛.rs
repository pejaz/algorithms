/*
 * @lc app=leetcode.cn id=827 lang=rust
 *
 * [827] 最大人工岛
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    // 暴力：反向思维，从海洋节点开始 dfs 遍历，查找最大岛屿面积
    // 方法二：利用标记一次 dfs 找出所有岛屿面积，然后后续通过标记判断最大面积
    pub fn largest_island(mut grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(
            grid: &mut Vec<Vec<i32>>,
            x: usize,
            y: usize,
            dir: &[(i32, i32); 4],
            count: &mut i32,
            mark: usize,
        ) {
            if grid[x][y] != 1 {
                return;
            } else {
                // 进行染色标记，也代表着访问过
                grid[x][y] = mark as i32;
                *count += 1;
            }

            for (r, c) in dir {
                let next_x = x as i32 + r;
                let next_y = y as i32 + c;
                if (0 <= next_x && next_x < grid.len() as i32)
                    && (0 <= next_y && next_y < grid[0].len() as i32)
                {
                    let next_x = next_x as usize;
                    let next_y = next_y as usize;
                    dfs(grid, next_x, next_y, dir, count, mark);
                }
            }
        }

        let direction = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        let mut area = vec![0; 500 * 500]; // 用 vec 来做 map
        let mut mark = 2;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    let mut count = 0;
                    dfs(&mut grid, i, j, &direction, &mut count, mark);
                    area[mark] = count;
                    mark += 1; // 下一次标记+1
                }
            }
        }

        let mut res = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                // 如果是海洋，遍历周围相加最大面积
                if grid[i][j] == 0 {
                    // 至少可以变一个0 为 1
                    let mut cur = 1;
                    let mut visit = vec![];
                    for (r, c) in direction {
                        let next_x = i as i32 + r;
                        let next_y = j as i32 + c;
                        if (0 <= next_x && next_x < grid.len() as i32)
                            && (0 <= next_y && next_y < grid[0].len() as i32)
                            && grid[next_x as usize][next_y as usize] != 0
                        {
                            let mark = grid[next_x as usize][next_y as usize];
                            if !visit.contains(&mark) {
                                cur += area[mark as usize];
                                visit.push(mark);
                            }
                        }
                    }
                    res = res.max(cur);
                }
            }
        }

        return if res == 0 {
            // 说明grid 都是 1，直接返回 n*n;
            grid.len().pow(2) as i32
        } else {
            res
        };
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
    fn test_largest_island() {
        println!(
            "res is {:?}",
            Solution::largest_island(vec![vec![1, 1], vec![1, 0]])
        );
        //assert_eq!(Solution::largest_island(vec![]),[]);
        assert!(true)
    }
}
