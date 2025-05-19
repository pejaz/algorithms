/*
 * @lc app=leetcode.cn id=90 lang=rust
 *
 * [90] 子集 II
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtracking(nums: &Vec<i32>, idx: usize, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            //  子集问题，每进入一个递归节点就收集结果。
            res.push(path.clone());

            for i in idx..nums.len() {
                if i > idx && nums[i] == nums[i - 1] {
                    continue;
                }

                path.push(nums[i]);
                backtracking(nums, i + 1, path, res);
                path.pop();
            }
        }

        let mut path = vec![];
        let mut res = vec![];
        nums.sort();
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
    fn test_subsets_with_dup() {
        //assert_eq!(Solution::subsets_with_dup(vec![]),[]);
        assert!(true)
    }
}
