/*
 * @lc app=leetcode.cn id=56 lang=rust
 *
 * [56] 合并区间
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut res = vec![];
        for i in 0..intervals.len() {
            if i == 0 {
                res.push(intervals[i].clone());
                continue;
            }

            let mut pre = res.last_mut().unwrap();
            if intervals[i][0] <= pre[1] {
                // 有交集
                pre[1] = pre[1].max(intervals[i][1]);
            } else {
                res.push(intervals[i].clone());
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
    fn test_merge() {
        //assert_eq!(Solution::merge(vec![]),[]);
        assert!(true)
    }
}
