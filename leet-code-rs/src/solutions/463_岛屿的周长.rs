/*
 * @lc app=leetcode.cn id=463 lang=rust
 *
 * [463] 岛屿的周长
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    // 思路一：根据 DFS 遍历岛屿，然后每个岛屿面积代表多少个陆地，
    // 此时每个陆地理论周长为 4，接着每两个陆地挨着就减去 2(a和b相连，则相连的那条边,a和b都要减去 1，即一共减 2)
    //   count* 4 - connect * 2
    // 思路二：不需要DFS 和 BFS，直接根据陆地周围情况，如果是海洋或者边界则周长加一
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let direction = [(0, -1), (0, 1), (-1, 0), (1, 0)];

        let mut perimeter = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    // 遍历陆地周围
                    for (r, c) in direction {
                        let next_x = i as i32 + r;
                        let next_y = j as i32 + c;
                        if (0 <= next_x && next_x < grid.len() as i32)
                            && (0 <= next_y && next_y < grid[0].len() as i32)
                        {
                            // 没有超出边界，则判断是否是海洋
                            if grid[next_x as usize][next_y as usize] == 0 {
                                perimeter += 1;
                            }
                        } else {
                            // 超出边界
                            perimeter += 1;
                        }
                    }
                }
            }
        }

        return perimeter;
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
    fn test_island_perimeter() {
        //println!("res is {:?}", Solution::island_perimeter(vec![]));
        //assert_eq!(Solution::island_perimeter(vec![]),[]);
        assert!(true)
    }
}
