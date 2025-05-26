/*
 * @lc app=leetcode.cn id=1049 lang=rust
 *
 * [1049] 最后一块石头的重量 II
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    // 解题思路：
    //     粉碎剩余最小重量，可以转化为： 石头总重量 weight 分为 2 半，
    //     此时有一个重量为 weight/2 的背包，物品对应各个石头，物品重量等于价值，物品只能使用 1 次
    //     尽可能的装满背包重量，此时每个背包剩余 weight/2 - dp[weight/2] 重量。
    //     那么剩余的石头重量就等于  weight/2 - dp[weight/2] * 2 = weight - dp[weight/2] * 2。
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let weight: i32 = stones.iter().sum();

        // 1. 初始化dp
        let bagweight = weight as usize / 2;
        let mut dp = vec![0; bagweight + 1];

        // 2. 递推公式 dp[j] = max(dp[j],dp[j-weight[i]]+value[i])。
        //      weight[i] == value[i] == stones[i]
        // 3. 先遍历物品
        for i in 0..stones.len() {
            // 倒序遍历背包：确保每个物品只能放入 1 次
            let stone = stones[i] as usize;
            for j in (stone..=bagweight).rev() {
                dp[j] = dp[j].max(dp[j - stone] + stone);
            }
        }

        return weight - dp[bagweight] as i32 * 2;
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
    fn test_last_stone_weight_ii() {
        //assert_eq!(Solution::last_stone_weight_ii(vec![]),[]);
        assert!(true)
    }
}
