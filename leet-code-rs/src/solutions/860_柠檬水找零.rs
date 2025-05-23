/*
 * @lc app=leetcode.cn id=860 lang=rust
 *
 * [860] 柠檬水找零
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut five = 0;
        let mut ten = 0;

        for n in bills {
            match n {
                5 => five += 1,
                10 => {
                    five -= 1;
                    ten += 1;
                }
                20 => {
                    // 优先用 10 的找零
                    if ten > 0 {
                        ten -= 1;
                        five -= 1;
                    } else {
                        five -= 3;
                    }
                }
                _ => {}
            };

            if five < 0 {
                return false;
            }
        }

        return true;
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
    fn test_lemonade_change() {
        //assert_eq!(Solution::lemonade_change(vec![]),[]);
        assert!(true)
    }
}
