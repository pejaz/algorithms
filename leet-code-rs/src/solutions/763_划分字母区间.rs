/*
 * @lc app=leetcode.cn id=763 lang=rust
 *
 * [763] 划分字母区间
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        // 另一个简单的思路：
        // 1. 使用 map，遍历一边字符串，如果 map 没有字符说明是新增的，此时新增 key，值设为【i,i】代表区间长度为 1，
        // 后续如果碰到 key，则更新值的右区间为【i,j】。遍历一遍之后就会生成所有字符的区间（即每个字符的[最左下标,最右下标]）了。
        // 2. 此时就可以转为合并重叠区间的问题，将所有区间按左边或者右边排序。然后遍历区间，如果直接有重叠就合并为大的区间。
        // 最后剩下的区间就是没有重叠的，然后区间【i,j】长度就是切割长度
        let n = s.len(); 
        let mut last = [0; 26];
        for (i, c) in s.bytes().enumerate() {
            last[(c - b'a') as usize] = i; // 每个字母最后出现的下标
        }

        let mut ans = vec![];
        let mut start = 0;
        let mut end = 0;
        for (i, c) in s.bytes().enumerate() {
            end = end.max(last[(c - b'a') as usize]); // 更新当前区间右端点的最大值
            if end == i {
                // 当前区间合并完毕
                ans.push((end - start + 1) as i32); // 区间长度加入答案
                start = i + 1; // 下一个区间的左端点
            }
        }
        ans
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
    fn test_partition_labels() {
        // aacabbccddbbccbbdd
        // 38
        //assert_eq!(Solution::partition_labels(vec![]),[]);
        assert!(true)
    }
}
