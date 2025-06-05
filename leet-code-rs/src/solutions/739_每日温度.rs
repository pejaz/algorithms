/*
 * @lc app=leetcode.cn id=739 lang=rust
 *
 * [739] 每日温度
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    // 解题思路：单调递减栈
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut single_stack = vec![];
        let mut res = vec![0; temperatures.len()];

        for (i, n) in temperatures.iter().enumerate() {
            while !single_stack.is_empty() && temperatures[*single_stack.last().unwrap()] < *n {
                let pre = single_stack.pop().unwrap();
                res[pre] = (i - pre) as i32;
            }

            single_stack.push(i);
        }

        // 剩下的是单调递减的温度，代表往后不会升温，设置值为 0，初始化即为 0，这里跳过
        // while !single_stack.is_empty() {
        //     let idx = single_stack.pop().unwrap();
        //     res[idx] = 0;
        // }

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
    fn test_daily_temperatures() {
        //println!("res is {:?}", Solution::daily_temperatures(vec![]));
        //assert_eq!(Solution::daily_temperatures(vec![]),[]);
        assert!(true)
    }
}
