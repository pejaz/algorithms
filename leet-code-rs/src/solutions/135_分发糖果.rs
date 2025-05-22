/*
 * @lc app=leetcode.cn id=135 lang=rust
 *
 * [135] 分发糖果
 */

// @lc code=start
#[allow(unused)]
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut candys = vec![1; ratings.len()];
        // 思路：每个孩子需要考虑左右两边的情况，此时可以优先确认一边，在基础上再考虑另一边，做到降维的目的
        //  1. 从左到右，先考虑右边比左边大的情况
        for i in 1..ratings.len() {
            if ratings[i] > ratings[i - 1] {
                // 比左边大，则在左边基础+1
                candys[i] = candys[i - 1] + 1;
            } else {
                candys[i] = 1;
            }
        }

        // 2.从右到左，考虑左边比右边大的情况
        for i in (0..ratings.len()).rev() {
            if i == ratings.len() - 1 {
                // 要同时满足第一步情况，此时 candys = candys[i].max(1)
                // 但是 candys[i]肯定是满足大于等于 1 的。所以可以不用修改
                continue;
            }

            if ratings[i] > ratings[i + 1] {
                // 比右边大，则在右边基础+1,
                // 此时要注意满足第一步比左边大的情况，所以取最大值
                candys[i] = candys[i].max(candys[i + 1] + 1)
            }
        }

        return candys.iter().sum();
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
    fn test_candy() {
        assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
        // 1,2,3,3,2,1
        let res1 = Solution::candy(vec![1, 2, 3, 3, 2, 1]);
        // 1,2,3,2,1,1
        let res2 = Solution::candy(vec![1, 2, 3, 3, 1, 1]);
        println!("{res1} --- {res2}");

        assert!(true)
    }
}
