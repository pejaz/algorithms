/*
 * @lc app=leetcode.cn id=209 lang=rust
 *
 * [209] 长度最小的子数组
 */

// @lc code=start
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut slow = 0;
        let mut sum = 0;
        let mut res = usize::MAX;
        for fast in 0..nums.len() {
            sum += nums[fast];
            // 缩短数组长度
            while sum >= target {
                if sum - nums[slow] >= target {
                    sum -= nums[slow];
                    slow += 1;
                } else {
                    res = res.min(fast - slow + 1);
                    break;
                }
            }
        }

        return if res == usize::MAX { 0 } else { res } as i32;
    }
}
// @lc code=end

#[allow(unused)]
struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_sub_array_len() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(Solution::min_sub_array_len(3, vec![1, 4, 4]), 1);
        assert_eq!(
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
            0
        );
        assert!(true)
    }
}
