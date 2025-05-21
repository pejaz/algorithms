/*
 * @lc app=leetcode.cn id=122 lang=rust
 *
 * [122] 买卖股票的最佳时机 II
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // 股票买卖收益
        let mut res = 0;
        let end = prices.len() - 1;

        for i in 0..end {
            let diff = prices[i + 1] - prices[i];

            if diff > 0 {
                res += diff;
            }
        }

        // let mut diff = vec![0; end];
        // for i in 0..end {
        //     diff[i] = prices[i + 1] - prices[i];
        // }

        // for n in diff {
        //     // 收益为正的累加
        //     if n > 0 {
        //         res += n;
        //     }
        // }

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
    fn test_max_profit() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
        assert!(true)
    }
}
