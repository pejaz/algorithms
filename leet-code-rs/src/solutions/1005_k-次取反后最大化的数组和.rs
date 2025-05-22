/*
 * @lc app=leetcode.cn id=1005 lang=rust
 *
 * [1005] K 次取反后最大化的数组和
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, mut k: i32) -> i32 {
        nums.sort();

        for i in 0..nums.len() {
            if nums[i] < 0 {
                nums[i] = -nums[i];
                k -= 1;

                //  [-4,-3,-2] k=4
                // k消耗完了，或者数组不够被 k 消耗，此时最右边的即为最小值
                if (i == nums.len() - 1 || k == 0) {
                    let min_idx = i;

                    // 可以直接判断 k % 2 == 1，k为奇数则进行取反
                    while k > 0 {
                        nums[min_idx] = -nums[min_idx];
                        k -= 1;
                    }

                    break;
                }
            } else {
                // 排序了，所以往后剩下的都是正数 ,选取一个最小值,消耗 k 反复取反
                // 此时最小值有可能在当前，也有可能在上一个负数。
                // 例如 （-2,3）：最小值在 i-1 取反后的 2，（-2，1)最小值在当前 i
                let min_idx = if i == 0 {
                    0
                } else {
                    if nums[i] > nums[i - 1] { i - 1 } else { i }
                };

                // 可以直接判断 k % 2 == 1，k为奇数则进行取反
                while k > 0 {
                    nums[min_idx] = -nums[min_idx];
                    k -= 1;
                }
                break;
            }
        }

        return nums.iter().sum();
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
    fn test_largest_sum_after_k_negations() {
        assert_eq!(
            Solution::largest_sum_after_k_negations(vec![-4, -2, -3], 4),
            5
        );
        assert!(true)
    }
}
