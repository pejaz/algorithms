/*
 * @lc app=leetcode.cn id=496 lang=rust
 *
 * [496] 下一个更大元素 I
 */

// @lc code=start

use std::collections::HashMap;

#[allow(unused)]
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut res = vec![-1; nums1.len()];

        let map: HashMap<i32, usize> = nums1
            .iter()
            .enumerate()
            .map(|(idx, &value)| (value, idx))
            .collect();
        let mut stack = vec![];

        for n in nums2 {
            while !stack.is_empty() && stack.last() < Some(&n) {
                let n2 = stack.pop().unwrap();
                if map.contains_key(&n2) {
                    let idx = map.get(&n2).unwrap();
                    res[*idx] = n;
                }
            }
            stack.push(n);
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
    fn test_next_greater_element() {
        //println!("res is {:?}", Solution::next_greater_element(vec![]));
        //assert_eq!(Solution::next_greater_element(vec![]),[]);
        assert!(true)
    }
}
