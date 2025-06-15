/*
 * @lc app=leetcode.cn id=684 lang=rust
 *
 * [684] 冗余连接
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
    // 通过并查集，找出加入时已经同属一个集合的边即为冗余连接
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut dsu = DSU::new(edges.len() + 1);
        for edge in edges {
            if dsu.same_set(edge[0] as usize, edge[1] as usize) {
                return edge;
            }

            dsu.union(edge[0] as usize, edge[1] as usize);
        }

        // 因为一定存在冗余连接，这里不会走到
        return vec![];
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
    fn test_find_redundant_connection() {
        //println!("res is {:?}", Solution::find_redundant_connection(vec![]));
        //assert_eq!(Solution::find_redundant_connection(vec![]),[]);
        assert!(true)
    }
}
