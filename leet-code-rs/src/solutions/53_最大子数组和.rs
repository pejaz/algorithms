/*
 * @lc app=leetcode.cn id=53 lang=rust
 *
 * [53] 最大子数组和
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut last_max = 0;
        let mut res = i32::MIN;

        for n in nums {
            // 判断是否要累加前面的最大值
            let cur_max = n.max(last_max + n);
            if cur_max > res {
                res = cur_max;
            }

            last_max = cur_max;
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
    fn test_max_sub_array() {
        //assert_eq!(Solution::max_sub_array(vec![]),[]);
        assert!(true)
    }
}
