/*
 * @lc app=leetcode.cn id=55 lang=rust
 *
 * [55] 跳跃游戏
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        // 默认走到数组第一个下标
        let mut jump = 1;
        // 从 0 而不是从 1开始是因为可能 nums[0]就为 0，此时就应该返回 false。
        // 如：[0,1]
        for i in 0..nums.len() {
            jump = nums[i].max(jump - 1);

            // 步数为 0且没走到最后一个下标，不能再走下去了
            if jump == 0 && i != nums.len() - 1 {
                return false;
            }
        }

        // 另一个思路，判断覆盖范围 jump >= nums.len() - 1 则返回 true，
        // 如果 i > jump 则说明无法抵达

        return true;
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
    fn test_can_jump() {
        //assert_eq!(Solution::can_jump(vec![]),[]);
        assert!(true)
    }
}
