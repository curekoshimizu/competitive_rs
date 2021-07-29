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
        let size = 2 * n;
        let data = vec![monoid.e; size];
        SegmentTree { data, monoid, n }
    }

    pub fn update(&mut self, k: usize, a: T) {
        let mut k = k + self.n;
        self.data[k] = a;

        while k > 0 {
            k >>= 1;
            self.data[k] = (self.monoid.op)(&self.data[k << 1], &self.data[(k << 1) | 1]);
        }
    }

    pub fn get(&self, index: usize) -> T {
        self.query(index, index + 1)
    }

    /// [l, r)
    pub fn query(&self, l: usize, mut r: usize) -> T {
        assert!(l < r);
        if r > self.n {
            r = self.n;
        }

        let mut vl = self.monoid.e;
        let mut vr = self.monoid.e;

        let mut l = l + self.n;
        let mut r = r + self.n;

        while l < r {
            if l & 1 == 1 {
                vl = (self.monoid.op)(&vl, &self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                vr = (self.monoid.op)(&self.data[r], &vr);
            }

            l >>= 1;
            r >>= 1;
        }
        (self.monoid.op)(&vl, &vr)
    }
    //
    // pub fn update(&mut self, i: usize, x: T) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn segtree_4element() {
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
    fn segtree_5element() {
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
}
