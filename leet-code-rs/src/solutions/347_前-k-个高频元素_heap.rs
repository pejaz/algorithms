/*
 * @lc app=leetcode.cn id=347 lang=rust
 *
 * [347] 前 K 个高频元素
 */

// @lc code=start
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};
#[allow(unused)]
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hash = HashMap::new();
        let mut heap = BinaryHeap::with_capacity(k as usize);
        nums.into_iter().for_each(|k| {
            *hash.entry(k).or_insert(0) += 1;
        });

        // k 为 num ,v 为 times
        for (k, v) in hash {
            if heap.len() == heap.capacity() {
                if *heap.peek().unwrap() < (Reverse(v), k) {
                    continue;
                } else {
                    heap.pop();
                }
            }

            // BinaryHeap默认是倒序排序，reverse(v)转为正序排序
            heap.push((Reverse(v), k));
        }
        heap.into_iter().map(|(_, k)| k).collect()
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
