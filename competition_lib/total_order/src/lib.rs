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
}
