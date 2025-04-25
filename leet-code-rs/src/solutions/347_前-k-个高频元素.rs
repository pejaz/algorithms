/*
 * @lc app=leetcode.cn id=347 lang=rust
 *
 * [347] 前 K 个高频元素
 */

// @lc code=start
use std::collections::HashMap;
#[allow(unused)]
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // 第一步：统计每个元素的出现次数
        let mut cnt = HashMap::new();
        for x in nums {
            *cnt.entry(x).or_insert(0) += 1;
        }
        let max_cnt = *cnt.values().max().unwrap() as usize;

        // 第二步：把出现次数相同的元素，放到同一个桶中
        let mut buckets = vec![vec![]; max_cnt + 1];
        for (x, c) in cnt {
            buckets[c as usize].push(x);
        }

        // 第三步：倒序遍历 buckets，把出现次数前 k 大的元素加入答案
        let k = k as usize;
        let mut ans = Vec::with_capacity(k); // 预分配空间
        for bucket in buckets.into_iter().rev() {
            ans.extend(bucket);
            // 题目保证答案唯一，一定会出现恰好等于 k 的情况
            if ans.len() == k {
                return ans;
            }
        }
        unreachable!()
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
    fn test_top_k_frequent() {
        //assert_eq!(Solution::top_k_frequent(vec![]),[]);
        assert!(true)
    }
}
