pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }
    pub fn root(&mut self, x: usize) -> usize {
        let parent = self.parent[x];
        if x == parent {
            x // root node
        } else {
            let new_root = self.root(parent);
            self.parent[x] = new_root;
            new_root
        }
    }
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    pub fn unite(&mut self, x: usize, y: usize) {
        let x_root = self.root(x);
        let y_root = self.root(y);
        if x_root == y_root {
            return;
        }

        if self.rank[x_root] > self.rank[y_root] {
            self.parent[y_root] = x_root;
        } else if self.rank[x_root] == self.rank[y_root] {
            self.parent[x_root] = y_root;
            self.rank[y_root] += 1;
        } else {
            self.parent[x_root] = y_root;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn union_find() {
        // {0}, {1}, {2}, {3}, {4}, {5}, {6}
        let mut uf = UnionFind::new(7);

        // {0}, {1, 2}, {3}, {4}, {5}, {6}
        uf.unite(1, 2);
        // {0}, {1, 2, 3}, {4}, {5}, {6}
        uf.unite(2, 3);
        // {0}, {1, 2, 3}, {4}, {5, 6}
        uf.unite(5, 6);
        assert!(uf.is_same(1, 3));
        assert!(uf.is_same(5, 6));
        assert!(!uf.is_same(2, 5));

        uf.unite(1, 6);
        assert!(uf.is_same(2, 5));
    }
}
