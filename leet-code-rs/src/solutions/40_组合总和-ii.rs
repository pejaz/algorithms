/*
 * @lc app=leetcode.cn id=40 lang=rust
 *
 * [40] 组合总和 II
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn backtracking(
            nums: &Vec<i32>,
            start_idx: usize,
            target: i32,
            sum: i32,
            path: &mut Vec<i32>,
            res: &mut Vec<Vec<i32>>,
        ) {
            if sum > target {
                return;
            }

            if sum == target {
                res.push(path.clone())
            }

            for i in start_idx..nums.len() {
                // 树层去重
                if i > start_idx && nums[i] == nums[i - 1] {
                    continue;
                }

                path.push(nums[i]);
                backtracking(nums, i + 1, target, sum + nums[i], path, res);
                path.pop();
            }
        }

        let mut path = vec![];
        let mut res = vec![];
        candidates.sort();
        backtracking(&candidates, 0, target, 0, &mut path, &mut res);

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
    fn test_combination_sum2() {
        //assert_eq!(Solution::combination_sum2(vec![]),[]);
        assert!(true)
    }
}
