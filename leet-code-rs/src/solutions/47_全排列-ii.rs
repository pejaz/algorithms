/*
 * @lc app=leetcode.cn id=47 lang=rust
 *
 * [47] 全排列 II
 */

// @lc code=start

use std::collections::HashSet;

#[allow(unused)]
impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtracking(
            nums: &Vec<i32>,
            used: &mut Vec<i32>,
            path: &mut Vec<i32>,
            res: &mut Vec<Vec<i32>>,
        ) {
            if path.len() == nums.len() {
                res.push(path.clone());
                return;
            }

            let mut set = HashSet::new();
            for i in 0..nums.len() {
                // 这里 set 也可以用 used代替
                // 先对 nums 排序，然后判断 i > 0 && nums[i] == nums[i-1] 代表同层相等，同时 used[i-1] != 1 代表当前不是在树枝递归上。
                //   即 (i > 0 && nums[i] == nums[i - 1] && used[i - 1] == 0)
                // if used[i] == 1 || (i > 0 && nums[i] == nums[i - 1] && used[i - 1] == 0) {
                //     continue;
                // };
                if used[i] == 1 || set.contains(&nums[i]) {
                    continue;
                }
                set.insert(nums[i]);

                path.push(nums[i]);
                used[i] = 1;

                backtracking(nums, used, path, res);

                path.pop();
                used[i] = 0
            }
        }

        let mut path = vec![];
        let mut res = vec![];
        let mut used = vec![0; nums.len()];

        // 如果只用 used 来做同层判断去重的话需要先排序，方便判断nums[i] == nums[i - 1]相等
        // nums.sort();
        backtracking(&nums, &mut used, &mut path, &mut res);

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
    fn test_permute_unique() {
        println!("{:?}", Solution::permute_unique(vec![1, 2, 1]));
        //assert_eq!(Solution::permute_unique(vec![]),[]);
        assert!(true)
    }
}
