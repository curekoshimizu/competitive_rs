pub struct Monoid<T, F>
where
    F: Fn(&T, &T) -> T,
{
    e: T,
    op: F,
}

impl<T, F> Monoid<T, F>
where
    F: Fn(&T, &T) -> T,
{
    pub fn new(e: T, op: F) -> Self {
        Monoid { e, op }
    }
}

pub struct SegmentTree<T, F>
where
    F: Fn(&T, &T) -> T,
{
    data: Vec<T>,
    monoid: Monoid<T, F>,
    n: usize,
}

impl<T, F> SegmentTree<T, F>
where
    T: Copy,
    F: Fn(&T, &T) -> T,
{
    /// Example.
    /// If op = min, e = MAX.
    /// If op = max,  e = 0.
    /// If op = +,  e = 0.
    /// If op = *,  e = 1.
    pub fn new(n: usize, monoid: Monoid<T, F>) -> Self {
        // n must be power of 2.
        let mut new_n = 1;
        while new_n < n {
            new_n *= 2;
        }

        // data :
        // node_index
        //  index 0 : [0, new_n=2^m) -> 1, 2
        //
        //  index 1 : [0, 2^(m-1)) -> 3, 4
        //  index 2 : [2^(m-2), n) -> 5, 6
        //
        //  index 3 : -> 7, 8
        //  index 4 : -> 9, 10
        //  index 5 : -> 11, 12
        //  index 6 : -> 13, 14
        //     .....
        //  index k : -> 2k+1, 2k+2
        let size = 2 * new_n - 1;
        let data = vec![monoid.e; size];
        SegmentTree {
            data,
            monoid,
            n: new_n,
        }
    }

    pub fn update(&mut self, k: usize, a: T) {
        // because self.n is power of 2
        let mut k = k + self.n - 1;
        self.data[k] = a;

        while k > 0 {
            k = (k - 1) / 2;
            self.data[k] = (self.monoid.op)(&self.data[2 * k + 1], &self.data[2 * k + 2]);
        }
    }

    pub fn get(&self, index: usize) -> T {
        self.query(index, index + 1)
    }

    /// [l, r)
    pub fn query(&self, l: usize, r: usize) -> T {
        assert!(l < r);
        self._query(l, r, 0, 0, self.n)
    }
    /// [l, r)
    /// index k is corresponding to [node_left, node_right)
    fn _query(&self, l: usize, r: usize, k: usize, node_left: usize, node_right: usize) -> T {
        // if [a, b) has no intersection with [node_left, node_right)
        if (node_right <= l) || (r <= node_left) {
            self.monoid.e
        }
        // if [node_left, node_right) is in [a, b)
        else if l <= node_left && node_right <= r {
            self.data[k]
        } else {
            let mid = (node_left + node_right) / 2;
            let vl = self._query(l, r, 2 * k + 1, node_left, mid);
            let rl = self._query(l, r, 2 * k + 2, mid, node_right);
            (self.monoid.op)(&vl, &rl)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn segtree_4element_min() {
        let min = Monoid::new(u32::MAX, |&x, &y| u32::min(x, y));
        let mut tree = SegmentTree::new(4, min);
        tree.update(0, 10);
        tree.update(1, 11);
        tree.update(2, 21);
        tree.update(3, 1);

        assert_eq!(tree.get(2), 21);
        assert_eq!(tree.query(1, 3), 11);
        assert_eq!(tree.query(0, 3), 10);
        assert_eq!(tree.query(0, 4), 1);
    }
    #[test]
    fn segtree_5element_min() {
        let min = Monoid::new(i32::MAX, |&x, &y| i32::min(x, y));
        let mut tree = SegmentTree::new(5, min);
        tree.update(0, 10);
        tree.update(1, 11);
        tree.update(2, 21);
        tree.update(3, 1);
        tree.update(4, -1);

        assert_eq!(tree.get(2), 21);
        assert_eq!(tree.query(1, 3), 11);
        assert_eq!(tree.query(0, 3), 10);
        assert_eq!(tree.query(0, 4), 1);
        assert_eq!(tree.query(0, 5), -1);
        assert_eq!(tree.query(2, 5), -1);
    }
    #[test]
    fn segtree_6element_max() {
        let max = Monoid::new(i32::MIN, |&x, &y| i32::max(x, y));
        let mut tree = SegmentTree::new(5, max);
        tree.update(0, 10);
        tree.update(1, 11);
        tree.update(2, 21);
        tree.update(3, 1);
        tree.update(4, -1);
        tree.update(5, -32);

        assert_eq!(tree.get(3), 1);
        assert_eq!(tree.query(1, 3), 21);
        assert_eq!(tree.query(0, 3), 21);
        assert_eq!(tree.query(0, 5), 21);
        assert_eq!(tree.query(1, 6), 21);
        assert_eq!(tree.query(2, 6), 21);
        assert_eq!(tree.query(3, 6), 1);
        assert_eq!(tree.query(4, 6), -1);
    }
}
