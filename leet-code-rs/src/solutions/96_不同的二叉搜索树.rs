/* 🔖
 * @lc app=leetcode.cn id=96 lang=rust
 *
 * [96] 不同的二叉搜索树
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    // n:1=1,2=2,3=5
    // dp[n]推导过程：
    //  头节点为 1： 左子树 0 个节点的所有情况 * 右子树 n-1 个节点的所有情况
    //  头节点为 2： 左子树 1 个节点的所有情况 * 右子树 n-2 个节点的所有情况
    //  ...
    //  头节点为 n： 左子树 n -1 个节点的所有情况 * 右子树 0 个节点的所有情况
    // 以上所有情况相加为 dp[n] 的所有情况
    pub fn num_trees(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        // 初始化 dp: dp[n] 依赖dp 0..n-1
        dp[0] = 1; // 空二叉树 1 种情况，也符合 dp[1] = dp[0] * dp[0] = 1

        for i in 1..=n as usize {
            // 遍历左边节点数量
            for j in 0..i {
                dp[i] += dp[j] * dp[i - 1 - j];
            }
        }
        return dp[n as usize];
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
    fn test_num_trees() {
        //assert_eq!(Solution::num_trees(vec![]),[]);
        assert!(true)
    }
}
