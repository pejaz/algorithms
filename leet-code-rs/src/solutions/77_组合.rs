/*
 * @lc app=leetcode.cn id=77 lang=rust
 *
 * [77] 组合
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn dfs(n: i32, s: i32, k: usize, cur: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            // 剪枝方法一： 需要满足 n - s + 1 >= k - cur.len() 条件才继续
            // if cur.len() + ((n - s + 1) as usize) < k {
            //     // 剩余个数全部搜集也不满足k，提前剪枝
            //     return;
            // }

            if cur.len() == k {
                res.push(cur.clone());
                return;
            }

            // 方法二：水平方向上剪枝，即在 for 循环结束条件上判断
            // 加入 n=4,k=3,cur=0 ，即最多遍历到 2（此时搜集 234）。
            let end = n - (k - cur.len()) as i32 + 1;
            for i in s..=end {
                cur.push(i);
                dfs(n, i + 1, k, cur, res);
                cur.pop();
            }
        }

        let mut res = vec![];
        let mut cur = vec![];
        dfs(n, 1, k as usize, &mut cur, &mut res);

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
    fn test_combine() {
        //assert_eq!(Solution::combine(vec![]),[]);
        assert!(true)
    }
}
