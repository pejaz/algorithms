/*
 * @lc app=leetcode.cn id=151 lang=rust
 *
 * [151] 反转字符串中的单词
 */

// @lc code=start
#[allow(unused)]
impl Solution {
    pub fn remove_extra_spaces(s: &mut String) {
        let mut fast = 0;
        let mut slow = 0;
        let s_bytes = unsafe { s.as_bytes_mut() };
        let len = s_bytes.len();

        while fast < len {
            if s_bytes[fast] != b' ' {
                if slow != 0 {
                    s_bytes[slow] = b' ';
                    slow += 1;
                }

                while fast < len && s_bytes[fast] != b' ' {
                    s_bytes[slow] = s_bytes[fast];
                    fast += 1;
                    slow += 1;
                }

            }
            fast += 1;
        }

        s.truncate(slow);
    }

    pub fn reverse_words(s: String) -> String {
        let mut s = s;
        Self::remove_extra_spaces(&mut s);

        let s_bytes = unsafe { s.as_bytes_mut() };
        s_bytes.reverse();

        let mut start = 0;
        for i in (0..=s_bytes.len()) {
            if i == s_bytes.len() || s_bytes[i] == b' ' {
                s_bytes[start..i].reverse();
                start = i + 1;
            }
        }

        s
    }
}
// @lc code=end

#[allow(unused)]
struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_words() {
        let s = Solution::reverse_words(String::from("  hello   world  "));
        assert_eq!(
            s,
            String::from("world hello")
        );
        assert!(true)
    }
}
