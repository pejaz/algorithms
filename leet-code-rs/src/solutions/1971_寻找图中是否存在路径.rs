/*
 * @lc app=leetcode.cn id=1971 lang=rust
 *
 * [1971] 寻找图中是否存在路径
 */

// @lc code=start
pub struct DSU {
    parent: Vec<usize>,
}

impl DSU {
    /// 创建一个包含 n 个元素的新并查集
    pub fn new(n: usize) -> Self {
        DSU {
            parent: (0..n).collect(), // 每个元素初始指向自己
        }
    }

    /// 查找元素 x 的根节点
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // 路径压缩
        }
        self.parent[x]
    }

    /// 合并元素 x 和 y 所在的集合
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // 已经在同一个集合中
        }

        self.parent[root_y] = root_x;
        true
    }

    /// 检查 x 和 y 是否在同一个集合
    pub fn same_set(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}
#[allow(unused)]
impl Solution {
    /**
     * 思路一：用 BFS 或 DFS 判断能否抵达节点
     * 思路二：用并查集直接判断是否在同一集合中
     */
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut dsu = DSU::new(n as usize);
        for edge in edges {
            dsu.union(edge[0] as usize, edge[1] as usize);
        }

        return dsu.same_set(source as usize, destination as usize);
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
    fn test_valid_path() {
        //println!("res is {:?}", Solution::valid_path(vec![]));
        //assert_eq!(Solution::valid_path(vec![]),[]);
        assert!(true)
    }
}
