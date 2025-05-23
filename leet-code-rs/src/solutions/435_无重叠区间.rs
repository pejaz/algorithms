/*
 * @lc app=leetcode.cn id=435 lang=rust
 *
 * [435] 无重叠区间
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut res = 0;
        for i in 1..intervals.len() {
            if intervals[i][0] < intervals[i - 1][1] {
                // 有交集
                res += 1;
                // 更新右区间
                intervals[i][1] = intervals[i][1].min(intervals[i - 1][1]);
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
    fn test_erase_overlap_intervals() {
        //assert_eq!(Solution::erase_overlap_intervals(vec![]),[]);
        assert!(true)
    }
}
