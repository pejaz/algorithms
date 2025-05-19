/*
 * @lc app=leetcode.cn id=46 lang=rust
 *
 * [46] 全排列
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
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

            for i in 0..nums.len() {
                if used[i] == 1 {
                    continue;
                };

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
    fn test_permute() {
        //assert_eq!(Solution::permute(vec![]),[]);
        assert!(true)
    }
}
