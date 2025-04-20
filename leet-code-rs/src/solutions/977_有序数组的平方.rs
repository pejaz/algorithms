/*
 * @lc app=leetcode.cn id=977 lang=rust
 *
 * [977] 有序数组的平方
 */

// @lc code=start
#[allow(unused)]
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        // 因为原数组是递增的。
        // 且可能存在负数，负数的平方有可能比右边的大。
        // 所以用类似选择排序，每次只需要对比左边负数的平方和右边的平方谁比较大。
        // 选择比较大的放后面擂主位置，并把擂主往前移动一位，从最后下标依次往前放入。
        // 这里用 res 新数组来存最后结果，每次比较把大的放入数组
        let len = nums.len();
        let mut res = vec![0; len];
        // p 代表放入数组的元素位置（擂台）
        let mut p = len as i32 - 1;
        let mut l = 0;
        let mut r = len - 1;

        // for p in (0..len).rev(){}
        while p >= 0 {
            let l_sq = nums[l].pow(2);
            let r_sq = nums[r].pow(2);

            if l_sq > r_sq {
                res[p as usize] = l_sq;
                l += 1;
            } else {
                res[p as usize] = r_sq;
                r -= 1;
            }

            // 更新擂主下标
            p -= 1;
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
    fn test_sorted_squares() {
        assert_eq!(
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
        assert!(true)
    }
}
