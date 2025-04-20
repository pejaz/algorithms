/*
 * @lc app=leetcode.cn id=541 lang=rust
 *
 * [541] 反转字符串 II
 */

// @lc code=start
#[allow(unused)]
impl Solution {
    pub fn reverse_str(mut s: String, k: i32) -> String {
        let s_bytes = unsafe { s.as_bytes_mut() };
        let k = k as usize;
        let n = s_bytes.len();
        for i in (0..n).step_by(k * 2) {
            s_bytes[i..n.min(i + k)].reverse();
        }
        s

        // let mut v_c = s.chars().collect::<Vec<char>>();

        // let mut is_k = true;
        // // 按 k 拆分 chunk
        // for chunk in v_c.chunks_mut(k as usize) {
        //     if is_k {
        //         chunk.reverse();
        //     }
        //     is_k = !is_k;
        // }

        // v_c.into_iter().collect()
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
    fn test_reverse_str() {
        //assert_eq!(Solution::reverse_str(vec![]),[]);
        assert!(true)
    }
}
