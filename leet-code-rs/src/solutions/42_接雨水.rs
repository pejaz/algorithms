/*
 * @lc app=leetcode.cn id=42 lang=rust
 *
 * [42] 接雨水
 */

// @lc code=start
#[allow(unused)]
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        // let mut stack = vec![];
        let mut area = 0;

        // for i in 0..height.len() {
        //     // 出栈并计算横向雨水面积
        //     while stack.len() > 0 && height[i] > height[stack[stack.len() - 1]] {
        //         let idx = stack.pop().unwrap();

        //         if stack.len() <= 0 {
        //             // 无法形成 栈顶 -> pop -> i 凹形接雨水
        //             break;
        //         }

        //         let h = height[i].min(height[stack[stack.len() - 1]]) - height[idx];
        //         let w = (i - stack[stack.len() - 1] - 1) as i32;
        //         area += h * w;
        //     }

        //     stack.push(i);
        // }
        // return area;

        // 每次计算前缀最大值和后缀最大值小的那一格，接水取决于他们的最小值-高度
        // 这里也可以用两个数组，分别从左到右算出前缀最大值、从右到左算出后缀最大值，
        // 每一格的雨水 min(pre[i],suf[i]) - height[i]
        let mut left = 0;
        let mut right = height.len() -1;
        let mut pre_max = 0;
        let mut suf_max = 0;

        while left < right {
            pre_max = pre_max.max(height[left]);
            suf_max = suf_max.max(height[right]);

            if pre_max < suf_max {
                // 接 left 下标的雨水
                area += pre_max - height[left];
                left +=1;
            }else{
                // 接 right 下标的雨水
                area += suf_max - height[right];
                right -=1;
            }
        }

        return area;

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
    fn test_trap() {
        //assert_eq!(Solution::trap(vec![]),[]);
        assert!(true)
    }
}
