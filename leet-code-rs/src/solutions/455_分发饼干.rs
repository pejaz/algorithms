/*
 * @lc app=leetcode.cn id=455 lang=rust
 *
 * [455] 分发饼干
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        s.sort();
        g.sort();

        let mut res = 0;
        let mut i = 0;
        for f in s {
            if i < g.len() && f >= g[i] {
                // 饼干分配给小孩
                res += 1;
                i += 1;
            }
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
    fn test_find_content_children() {
        //assert_eq!(Solution::find_content_children(vec![]),[]);
        assert!(true)
    }
}
