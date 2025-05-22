/*
 * @lc app=leetcode.cn id=134 lang=rust
 *
 * [134] 加油站
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut cur_sum = 0; // 以 start 为起点的区间和
        let mut sum = 0; // 总的亏损和

        for i in 0..gas.len() {
            let diff = gas[i] - cost[i];

            sum += diff;
            cur_sum += diff;

            if cur_sum < 0 {
                // sum < 0 ，说明此时走不到 i 位置， 尝试从 i+1 作为起点走
                start = i + 1;
                cur_sum = 0;
            }
        }

        return if sum < 0 { -1 } else { start as i32 };
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
    fn test_can_complete_circuit() {
        //assert_eq!(Solution::can_complete_circuit(vec![]),[]);
        assert!(true)
    }
}
