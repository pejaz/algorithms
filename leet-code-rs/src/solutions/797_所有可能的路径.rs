/*
 * @lc app=leetcode.cn id=797 lang=rust
 *
 * [797] 所有可能的路径
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    // DFS
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn backtrace(graph: &Vec<Vec<i32>>, n: i32, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            // 到达 n-1 节点，收集结果
            if n == (graph.len() - 1) as i32 {
                res.push(path.clone());
                return;
            }

            // 遍历 n 节点所有可达节点
            for i in graph[n as usize].iter() {
                path.push(*i);
                // DFS，n->i, 接着以 i 为起始节点继续往下深度搜
                backtrace(graph, *i, path, res);
                path.pop();
            }
        }

        let mut path = vec![];
        let mut res = vec![];

        path.push(0); // 起始节点 0
        backtrace(&graph, 0, &mut path, &mut res);

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
    fn test_all_paths_source_target() {
        //println!("res is {:?}", Solution::all_paths_source_target(vec![]));
        //assert_eq!(Solution::all_paths_source_target(vec![]),[]);
        assert!(true)
    }
}
