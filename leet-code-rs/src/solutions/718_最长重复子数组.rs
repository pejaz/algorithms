/*
 * @lc app=leetcode.cn id=718 lang=rust
 *
 * [718] 最长重复子数组
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    /**
     * 解题思路：本题是子数组问题，用枚举思考
     *  1. dp[i][j]: 代表下标为 i-1 结尾的 nums1 和下标为 j-1 结尾的 nums2 的最长子数组长度。
     *  2. 递推公式：
     *     - 如果 nums1[i] == nums2[j]：dp[i+1][j+1] = dp[i][j] + 1
     *     - 如果 nums1[i] != nums2[j]：dp[i+1][j+1] = 0 （重置为 0 重新开始累加）
     *  3. 初始化：由递推公式可知，依赖 i-1，j-1 的状态。因为 dp[i][j]后续会被重新复制，所以这里随便初始化为 0 即可。
     *     同时dp[i][0] 和 dp[0][j]状态无意义无需特殊处理。下标为 0 代表 nums[-1]为空数组，结果肯定为 0。
     *  4. 结果：枚举问题结果在过程，需要用一个变量来获取最大值。
     */
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];
        let mut res = 0;

        for i in 0..nums1.len() {
            for j in 0..nums2.len() {
                // 因为dp初始化都为 0 了，所以不用处理 nums1[i] != nums2[j]
                if nums1[i] == nums2[j] {
                    dp[i + 1][j + 1] = dp[i][j] + 1;
                    res = res.max(dp[i + 1][j + 1]);
                };
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
    //  [0,1,1,1,1]
    //  [1,0,1,0,1]
    //     1  0  1  0  1
    // 0: [0, 1, 0, 1, 0],
    // 1: [1, 1, 2, 2, 2],
    // 1: [1, 1, 2, 2, 3],
    // 1: [1, 1, 2, 2, 3],
    // 1: [1, 1, 2, 2, 3]
    #[test]
    fn test_find_length() {
        //assert_eq!(Solution::find_length(vec![]),[]);
        assert!(true)
    }
}
