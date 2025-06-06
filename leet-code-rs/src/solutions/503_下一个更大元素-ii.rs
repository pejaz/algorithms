/*
 * @lc app=leetcode.cn id=503 lang=rust
 *
 * [503] 下一个更大元素 II
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    /**
     * 将 nums 复制一段拼到后面，用单调递减栈遍历查找大的元素
     */
    pub fn next_greater_elements(mut nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![-1; nums.len()];
        let mut stack = vec![];

        nums.append(&mut nums.clone());
        // 单调递减栈
        for (i, n) in nums.iter().enumerate() {
            while !stack.is_empty() && nums[*stack.last().unwrap()] < *n {
                let idx = stack.pop().unwrap();
                if idx >= res.len() {
                    continue;
                }

                res[idx] = *n;
            }

            stack.push(i);
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
    fn test_next_greater_elements() {
        //println!("res is {:?}", Solution::next_greater_elements(vec![]));
        //assert_eq!(Solution::next_greater_elements(vec![]),[]);
        assert!(true)
    }
}
