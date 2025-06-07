/*
 * @lc app=leetcode.cn id=84 lang=rust
 *
 * [84] 柱状图中最大的矩形
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    // 每根柱子的扩充边界为左边比它矮的以及右边比它矮的。
    // 单调递增栈，出栈的时候触发计算，计算当前出栈元素的最大面积，
    // 左边比它矮的就是栈顶（单调递增栈）,右边比它矮的就是当前遍历元素
    // 防止边界处理在栈两边填 0，左边 0 是防止元素出栈时，栈顶为空，右边 0是保证最后触发一次计算
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut stack = vec![];
        heights.insert(0, 0); // 左边插入 0，防止栈顶为空 
        heights.push(0); // 右边插入 0，触发最后一次计算

        for (i, h) in heights.iter().enumerate() {
            while !stack.is_empty() && heights[*stack.last().unwrap()] > *h {
                // 计算 cur 当前出栈元素所能扩充的最大面积
                let cur = heights[stack.pop().unwrap()];
                // 左边的 0 入栈后不可能大于 h，所以栈内一定有元素
                let w = i - stack.last().unwrap() - 1;
                res = res.max(w as i32 * cur);
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
    fn test_largest_rectangle_area() {
        //println!("res is {:?}", Solution::largest_rectangle_area(vec![]));
        //assert_eq!(Solution::largest_rectangle_area(vec![]),[]);
        assert!(true)
    }
}
