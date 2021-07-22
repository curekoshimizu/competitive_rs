#[derive(Debug, PartialEq, PartialOrd)]
pub struct TotalOrd<T>(pub T);
impl<T: PartialEq> Eq for TotalOrd<T> {}
impl<T: PartialOrd> Ord for TotalOrd<T> {
    fn cmp(&self, other: &TotalOrd<T>) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn btree_set() {
        use std::collections::BTreeSet;

        let mut set = BTreeSet::new();

        set.insert(TotalOrd(0.0));
        set.insert(TotalOrd(10.0));
        set.insert(TotalOrd(3.0));
        set.insert(TotalOrd(5.0));
        set.insert(TotalOrd(1.0));

        // get min value
        assert_eq!(set.iter().next(), Some(&TotalOrd(0.0)));
        // get max value
        assert_eq!(set.iter().last(), Some(&TotalOrd(10.0)));

        // TODO: range
        // for &elem in set.range((Included(TotalOrd(4.0), Included(&8))) {
        //     println!("{}", elem);
        // }
    }
    #[test]
    fn btree_map() {
        use std::collections::BTreeMap;

        let mut map = BTreeMap::new();

        map.insert(TotalOrd(0.0), 'a');
        map.insert(TotalOrd(10.0), 'b');
        map.insert(TotalOrd(3.0), 'c');
        map.insert(TotalOrd(5.0), 'd');
        map.insert(TotalOrd(1.0), 'e');

        // get min value
        assert_eq!(map.iter().next(), Some((&TotalOrd(0.0), &'a')));
        // get max value
        assert_eq!(map.iter().last(), Some((&TotalOrd(10.0), &'b')));

        // TODO: range
        // for &elem in set.range((Included(TotalOrd(4.0), Included(&8))) {
        //     println!("{}", elem);
        // }
    }
    #[test]
    fn heap() {
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::new();

        heap.push(TotalOrd(0.0));
        heap.push(TotalOrd(10.0));
        heap.push(TotalOrd(3.0));
        heap.push(TotalOrd(5.0));
        heap.push(TotalOrd(1.0));

        assert_eq!(heap.peek(), Some(&TotalOrd(10.0)));

        assert_eq!(heap.pop(), Some(TotalOrd(10.0)));
        assert_eq!(heap.pop(), Some(TotalOrd(5.0)));
        assert_eq!(heap.pop(), Some(TotalOrd(3.0)));
        assert_eq!(heap.pop(), Some(TotalOrd(1.0)));
        assert_eq!(heap.pop(), Some(TotalOrd(0.0)));
    }
    #[test]
    fn rev_heap() {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::new();

        heap.push(Reverse(TotalOrd(0.0)));
        heap.push(Reverse(TotalOrd(10.0)));
        heap.push(Reverse(TotalOrd(3.0)));
        heap.push(Reverse(TotalOrd(5.0)));
        heap.push(Reverse(TotalOrd(1.0)));

        assert_eq!(heap.peek(), Some(&Reverse(TotalOrd(0.0))));

        assert_eq!(heap.pop(), Some(Reverse(TotalOrd(0.0))));
        assert_eq!(heap.pop(), Some(Reverse(TotalOrd(1.0))));
        assert_eq!(heap.pop(), Some(Reverse(TotalOrd(3.0))));
        assert_eq!(heap.pop(), Some(Reverse(TotalOrd(5.0))));
        assert_eq!(heap.pop(), Some(Reverse(TotalOrd(10.0))));
    }
    #[test]
    fn vector_sort() {
        let mut v = vec![
            TotalOrd(0.0),
            TotalOrd(10.0),
            TotalOrd(3.0),
            TotalOrd(5.0),
            TotalOrd(1.0),
        ];
        v.sort();
        assert_eq!(v[0], TotalOrd(0.0));
        assert_eq!(v[v.len() - 1], TotalOrd(10.0));
    }
}
