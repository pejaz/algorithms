/*
 * @lc app=leetcode.cn id=242 lang=rust
 *
 * [242] 有效的字母异位词
 */

// @lc code=start
#[allow(unused)]
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // 用数组下标做哈希表
        let mut cnt = [0; 26];
        for c in s.bytes() {
            cnt[(c - b'a') as usize] += 1;
        }
        for c in t.bytes() {
            cnt[(c - b'a') as usize] -= 1;
        }
        cnt.iter().all(|&x| x == 0)
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
    fn test_is_anagram() {
        //assert_eq!(Solution::is_anagram(vec![]),[]);
        assert!(true)
    }
}
