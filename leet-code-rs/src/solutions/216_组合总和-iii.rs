/*
 * @lc app=leetcode.cn id=216 lang=rust
 *
 * [216] 组合总和 III
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        fn dfs(
            k: usize,
            n: i32,
            start_idx: i32,
            sum: i32,
            path: &mut Vec<i32>,
            res: &mut Vec<Vec<i32>>,
        ) {
            if sum > n || path.len() > k {
                return;
            }

            if sum == n && path.len() == k {
                res.push(path.clone());
                return;
            }

            // for 这里同样可以剪枝，let end = 9 - (k - path.len()) + 1
            // 接下来只需要遍历到 start_idx..=end 就可以了
            for i in start_idx..=9 {
                path.push(i);
                dfs(k, n, i + 1, sum + i, path, res);
                path.pop();
            }
        }

        let mut path = vec![];
        let mut res = vec![];
        dfs(k as usize, n, 1, 0, &mut path, &mut res);

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
    fn test_combination_sum3() {
        //assert_eq!(Solution::combination_sum3(vec![]),[]);
        assert!(true)
    }
}
