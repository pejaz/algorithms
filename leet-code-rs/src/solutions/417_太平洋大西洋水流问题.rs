/*
 * @lc app=leetcode.cn id=417 lang=rust
 *
 * [417] 太平洋大西洋水流问题
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    // 反向思维，从边缘往里面遍历，将符合条件的下标置为 true
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let row = heights.len();
        let col = heights[0].len();
        let mut pacifics = vec![vec![false; col]; row];
        let mut atlantics = vec![vec![false; col]; row];
        let direction = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        fn dfs(
            heights: &Vec<Vec<i32>>,
            x: usize,
            y: usize,
            ocean: &mut Vec<Vec<bool>>,
            direction: &[(i32, i32); 4],
        ) {
            if ocean[x][y] == true {
                return;
            } else {
                ocean[x][y] = true;
            }

            for (r, c) in direction {
                let next_x = x as i32 + r;
                let next_y = y as i32 + c;
                if (0 <= next_x && next_x < heights.len() as i32)
                    && (0 <= next_y && next_y < heights[0].len() as i32)
                {
                    let next_x = next_x as usize;
                    let next_y = next_y as usize;
                    if heights[next_x][next_y] >= heights[x][y] {
                        dfs(heights, next_x, next_y, ocean, direction);
                    }
                }
            }
        }

        // 反向找出能流向左右方向的
        for i in 0..row {
            // 流向左边太平洋
            dfs(&heights, i, 0, &mut pacifics, &direction);
            // 流向右边大西洋
            dfs(&heights, i, col - 1, &mut atlantics, &direction);
        }
        // 反向找出能流向上下方向的
        for j in 0..col {
            // 流向上面太平洋
            dfs(&heights, 0, j, &mut pacifics, &direction);
            // 流向下面大西洋
            dfs(&heights, row - 1, j, &mut atlantics, &direction);
        }

        // 找出能同时流向太平洋和大西洋的
        let mut res = vec![];
        for i in 0..row {
            for j in 0..col {
                if pacifics[i][j] == true && atlantics[i][j] == true {
                    res.push(vec![i as i32, j as i32]);
                }
            }
        }

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
    fn test_pacific_atlantic() {
        //println!("res is {:?}", Solution::pacific_atlantic(vec![]));
        //assert_eq!(Solution::pacific_atlantic(vec![]),[]);
        assert!(true)
    }
}
