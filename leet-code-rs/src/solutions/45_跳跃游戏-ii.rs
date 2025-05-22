/*
 * @lc app=leetcode.cn id=45 lang=rust
 *
 * [45] 跳跃游戏 II
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return 0;
        }

        let mut res = 0;
        let mut jump = 0;
        let mut next_jump = 0;

        let mut i = 0;
        while i <= jump {
            // 能到达终点，返回次数
            if jump as usize >= nums.len() - 1 {
                return res;
            }

            next_jump = next_jump.max(i + nums[i as usize]);

            if i == jump {
                // 更新下一步的最大范围
                jump = next_jump;
                res += 1;
            }

            i += 1;
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
    fn test_jump() {
        assert_eq!(Solution::jump(vec![1, 2]), 1);
        assert_eq!(Solution::jump(vec![0]), 0);
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
        assert!(true)
    }
}
