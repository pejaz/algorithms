/*
 * @lc app=leetcode.cn id=239 lang=rust
 *
 * [239] 滑动窗口最大值
 */

// @lc code=start
use std::collections::VecDeque;
#[allow(unused)]
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut res = vec![];
        let mut q = VecDeque::new();
        let k = k as usize;
        // 单调递减队列，队头为窗口内最大值
        for (i, n) in nums.iter().enumerate() {
            // 入 push
            while !q.is_empty() && &nums[*q.back().unwrap()] < n {
                q.pop_back();
            }
            q.push_back(i);

            // 出 pop,离开窗口要删除元素
            // k 和 i 为 3 时，下标为 0 的元素要离开窗口
            if i - q.front().unwrap() >= k {
                q.pop_front();
            }

            // 记录 getMaxValue
            if i >= k - 1 {
                res.push(nums[*q.front().unwrap()])
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
    fn test_max_sliding_window() {
        //assert_eq!(Solution::max_sliding_window(vec![]),[]);
        assert!(true)
    }
}
