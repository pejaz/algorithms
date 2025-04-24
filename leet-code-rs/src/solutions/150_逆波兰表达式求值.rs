/*
 * @lc app=leetcode.cn id=150 lang=rust
 *
 * [150] 逆波兰表达式求值
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        for token in tokens.iter().map(|s| &s[..]) {
            let new_to_push = token.parse::<i32>().unwrap_or_else(|_| {
                let num2 = stack.pop().unwrap();
                let num1 = stack.pop().unwrap();
                match token {
                    "+" => num1 + num2,
                    "-" => num1 - num2,
                    "*" => {
                        return num1 * num2;
                    }
                    "/" => num1 / num2,
                    _ => unreachable!(),
                }
            });
            stack.push(new_to_push);
        }
        return stack.pop().unwrap();
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
    fn test_eval_rpn() {
        //assert_eq!(Solution::eval_rpn(vec![]),[]);
        assert!(true)
    }
}
