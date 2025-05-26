/*
 * @lc app=leetcode.cn id=416 lang=rust
 *
 * [416] 分割等和子集
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    // 解题思路：
    //     1. 可以用回溯暴力。
    //     2. 转化为背包问题，两个自己元素和相等，即判断是否有元素可以装满背包，背包重量为：总元素和/2,物品重量和价值相等
    pub fn can_partition(nums: Vec<i32>) -> bool {
        // 分割为 2 个背包，背包重量为总元素和/2，判断是否有物体能装满，
        // 每个元素只能用 1 次，所以是 01 背包问题。
        let sum:i32 = nums.iter().sum();
        let bagweight = sum / 2;

        if sum % 2 != 0 {
            // sum为奇数，不可能相等
            return false;
        }

        // 是否有元素和为背包
        let mut dp = vec![0;bagweight as usize +1];
        // 递推公式：dp[j] = max(dp[j],dp[j-weight[i]])+value[i]。weight[i] == value[i] == nums[i]
        // 2 层 for ，先遍历物品
        for i in 0..nums.len(){
            // 再遍历背包，背包倒序
            for j in (nums[i]..=bagweight).rev(){
                dp[j as usize] = dp[j as usize].max(dp[(j-nums[i]) as usize]+nums[i]);
            }
        }

        return dp[bagweight as usize].eq(&bagweight);
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
    fn test_can_partition() {
        //assert_eq!(Solution::can_partition(vec![]),[]);
        assert!(true)
    }
}
