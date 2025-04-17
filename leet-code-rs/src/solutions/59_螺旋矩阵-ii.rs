/*
 * @lc app=leetcode.cn id=59 lang=rust
 *
 * [59] 螺旋矩阵 II
 */

// @lc code=start
enum Direction {
    TOP,
    RIGHT,
    BOTTOM,
    LEFT,
}
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut res = vec![vec![0; n as usize]; n as usize];
        let direcs = vec![
            Direction::TOP,
            Direction::RIGHT,
            Direction::BOTTOM,
            Direction::LEFT,
        ];
        let mut d_idx = 0;
        // 3: 2 ，1 + 1。 4: 3 ，2 + 0。5: 4 , 2 + 1 。n: n-1, n/2 ..1。
        let mut start = 0 as usize;
        let mut end = (n as usize) - 1;
        let mut row = start;
        let mut col = start;
        for i in 1..=n.pow(2) {
            let num = i;
            res[row][col] = num;

            match direcs[d_idx] {
                Direction::TOP => {
                    col += 1;

                    if col == end {
                        // 转方向
                        d_idx = (d_idx + 1) % direcs.len();
                    }
                }
                Direction::RIGHT => {
                    row += 1;

                    if row == end {
                        d_idx = (d_idx + 1) % direcs.len();
                    }
                }
                Direction::BOTTOM => {
                    col -= 1;

                    if col == start {
                        d_idx = (d_idx + 1) % direcs.len();
                    }
                }
                Direction::LEFT => {
                    row -= 1;

                    if row == start {
                        d_idx = (d_idx + 1) % direcs.len();
                        // 下一轮开始，重置边界
                        start += 1;
                        end -= 1;
                        row = start;
                        col = start;
                    }
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
    use super::*;

    #[test]
    fn test_generate_matrix() {
        assert_eq!(
            Solution::generate_matrix(3),
            [[1, 2, 3], [8, 9, 4], [7, 6, 5]]
        );
        println!("4 => {:?}", Solution::generate_matrix(4));
        println!("5 => {:?}", Solution::generate_matrix(5));
        println!("1 => {:?}", Solution::generate_matrix(1));
        assert!(true)
    }
}
