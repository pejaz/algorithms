/*
 * @lc app=leetcode.cn id=491 lang=rust
 *
 * [491] 非递减子序列
 */

// @lc code=start

use std::collections::HashSet;

#[allow(unused)]
impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtracking(nums: &Vec<i32>, idx: usize, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            //  子集问题，每进入一个递归节点就收集结果。
            if path.len() >= 2 {
                res.push(path.clone());
            }

            let mut set = HashSet::new();
            for i in idx..nums.len() {
                if set.contains(&nums[i]) {
                    continue;
                }
                set.insert(nums[i]);

                if &nums[i] >= path.last().unwrap_or(&i32::MIN) {
                    path.push(nums[i]);
                    backtracking(nums, i + 1, path, res);
                    path.pop();
                }
            }
        }

        let mut path = vec![];
        let mut res = vec![];
        backtracking(&nums, 0, &mut path, &mut res);

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
    fn test_find_subsequences() {
        //assert_eq!(Solution::find_subsequences(vec![]),[]);
        assert!(true)
    }
}
