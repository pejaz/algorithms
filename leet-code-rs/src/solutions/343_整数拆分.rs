/*
 * @lc app=leetcode.cn id=343 lang=rust
 *
 * [343] 整数拆分
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        // 1. dp[n] 代表对 n 进行拆分后的最大值
        // 2. 递推公式: 可以对 n 进行拆 2 个数或者 2 个数以上
        //      dp[n] = max(i*(n-i), i * dp[n-i] )
        let mut dp = vec![0; n as usize + 1];
        // 3. 初始化,0，1 不能拆，因为要拆正整数，0不属于。
        for i in 2..=n as usize {
            let mut i_max = 0;
            // i 要拆正整数，从 1 拆起，到 i - 1结束
            // 这里可以优化不用拆到 i-1，因为拆分最大值就应该尽可能给他拆分为相同的数
            // 100： max(50*50,33*33*34,25*25*25*25,20*20*20*20*20...)
            // 所以这里最多拆分 2 个相等值就可以了，也就是最多拆到 i/2
            // 遍历范围也就是1..=i/2即可
            for j in 1..=(i / 2) {
                i_max = i_max.max(j * (i - j)).max(j * dp[i - j])
            }
            dp[i] = i_max;
        }

        return dp[n as usize] as i32;
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
    fn test_integer_break() {
        //assert_eq!(Solution::integer_break(vec![]),[]);
        assert!(true)
    }
}
