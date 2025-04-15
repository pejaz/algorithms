/*
 * @lc app=leetcode.cn id=42 lang=rust
 *
 * [42] 接雨水
 */

// @lc code=start
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut area = 0;

        for i in 0..height.len() {
            // 出栈并计算横向雨水面积
            while stack.len() > 0 && height[i] > height[stack[stack.len() - 1]] {
                let idx = stack.pop().unwrap();

                if stack.len() <= 0 {
                    // 无法形成 栈顶 -> pop -> i 凹形接雨水
                    break;
                }

                let h = height[i].min(height[stack[stack.len() - 1]]) - height[idx];
                let w = (i - stack[stack.len() - 1] - 1) as i32;
                area += h * w;
            }

            stack.push(i);
        }
        return area;
    }
}
// @lc code=end
#[allow(unused)]
struct Solution;
