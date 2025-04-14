/*
 * @lc app=leetcode.cn id=704 lang=rust
 *
 * [704] 二分查找
 */

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        while left <= right {
            let idx = left + right >> 1;
            if target == nums[idx as usize] {
                return idx;
            }

            if target < nums[idx as usize] {
                right = idx - 1;
            } else {
                left = idx + 1;
            }
        }
        -1
    }
}
// @lc code=end
