/*
 * @lc app=leetcode.cn id=93 lang=rust
 *
 * [93] 复原 IP 地址
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        fn backtracking(s: &String, idx: usize, path: &mut Vec<String>, res: &mut Vec<String>) {
            if idx == s.len() && path.len() == 4 {
                res.push(path.join("."));
            }

            for i in idx..s.len() {
                let ip = &s[idx..=i];

                if i - idx > 3 {
                    // ip 不能超过 3 位
                    break;
                }

                if ip.len() > 1 && ip.starts_with('0') {
                    // 2 位ip 不能以 0 开头
                    continue;
                }

                if ip.parse::<u16>().unwrap() <= 255 {
                    path.push(ip.to_string());
                    backtracking(s, i + 1, path, res);
                    path.pop();
                }
            }
        }

        let mut path = vec![];
        let mut res = vec![];

        backtracking(&s, 0, &mut path, &mut res);

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
    fn test_restore_ip_addresses() {
        println!(
            "{:?}",
            Solution::restore_ip_addresses(String::from("101023"))
        );
        //assert_eq!(Solution::restore_ip_addresses(vec![]),[]);
        assert!(true)
    }
}
