use std::cmp::Ordering;

pub enum ReturnIndexPolicy {
    AllowBoundary,     // same as lower_bound
    GreaterThanTarget, // same as upper_bound
}

pub trait BinarySearch<T> {
    fn binary_search_position(&self, x: &T, policy: ReturnIndexPolicy) -> Option<usize> {
        match policy {
            ReturnIndexPolicy::AllowBoundary => self.lower_bound(x),
            ReturnIndexPolicy::GreaterThanTarget => self.upper_bound(x),
        }
    }
    ///   returned index of T is is equal to or greater than search_target.
    ///   if search_target is too big, it will return None.
    ///
    ///   let index = vec.lower_bound(&search_target)
    ///   [0..index].iter().all(|&x| x < search_target);
    ///   [index..].iter().all(|&x| x >= search_target); index returns [search_target, ...)
    fn lower_bound(&self, x: &T) -> Option<usize>;
    ///   returned index of T is greater than search_target.
    ///   if search_target is too big, it will return None.
    ///
    ///   let index = vec.lower_bound(&search_target)
    ///   let index = vec.upper_bound(&search_target)
    ///   [0..index].iter().all(|&x| x <= search_target);
    ///   [index..].iter().all(|&x| x > search_target); index returns (search_target, ...)
    fn upper_bound(&self, x: &T) -> Option<usize>;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> Option<usize> {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        if low < self.len() {
            Some(low)
        } else {
            None
        }
    }

    fn upper_bound(&self, x: &T) -> Option<usize> {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        if low < self.len() {
            Some(low)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_lower() {
        let target = vec![1, 3, 5];
        let index = target.lower_bound(&3).unwrap();
        assert_eq!(index, 1);
        target[..index].iter().all(|&x| x < 3);
        target[index..].iter().all(|&x| x >= 3);

        let target = vec![1, 3, 5];
        let index = target.lower_bound(&2).unwrap();
        assert_eq!(index, 1);
        target[..index].iter().all(|&x| x < 2);
        target[index..].iter().all(|&x| x >= 2);
    }
    #[test]
    fn example_upper() {
        let target = vec![1, 3, 5];
        let index = target.upper_bound(&3).unwrap();
        assert_eq!(index, 2);
        target[..index].iter().all(|&x| x <= 3);
        target[index..].iter().all(|&x| x > 3);

        let target = vec![1, 3, 5];
        let index = target.lower_bound(&2).unwrap();
        assert_eq!(index, 1);
        target[..index].iter().all(|&x| x < 2);
        target[index..].iter().all(|&x| x >= 2);
    }
    #[test]
    fn binary_search() {
        let vec = vec![1, 2, 4, 6, 7, 12, 54, 60, 100];

        for target in 2..20 {
            let pos = vec.lower_bound(&target).unwrap();
            if let Some(index) = vec.iter().position(|&x| x == target) {
                assert!(vec[pos] == target);
                assert_eq!(pos, index);
            } else {
                assert!(vec[pos] > target);
            }
            assert!(vec[pos - 1] < target);

            let pos = vec.upper_bound(&target).unwrap();
            assert!(vec[pos] > target);
            assert!(vec[pos - 1] <= target);
        }

        // check boundary (small value)
        let pos = vec.lower_bound(&1);
        assert_eq!(pos, Some(0));
        let pos = vec.lower_bound(&-1000);
        assert_eq!(pos, Some(0));

        let pos = vec.upper_bound(&1);
        assert_eq!(pos, Some(1));
        let pos = vec.upper_bound(&-1000);
        assert_eq!(pos, Some(0));

        // check boundary (big value)
        let pos = vec.lower_bound(&100);
        assert_eq!(pos, Some(vec.len() - 1));
        let pos = vec.lower_bound(&100000);
        assert_eq!(pos, None);

        let pos = vec.upper_bound(&100);
        assert_eq!(pos, None);
        let pos = vec.upper_bound(&100000);
        assert_eq!(pos, None)
    }

    #[test]
    fn binary_search_2() {
        let vec = vec![1, 1, 2, 2, 4, 4, 4, 100, 100];

        let pos = vec.lower_bound(&4);
        assert_eq!(pos, Some(4));
        let pos = vec.lower_bound(&3);
        assert_eq!(pos, Some(4));

        let pos = vec.upper_bound(&4);
        assert_eq!(pos, Some(7));
        let pos = vec.upper_bound(&3);
        assert_eq!(pos, Some(4));

        // check boundary (small value)
        let pos = vec.lower_bound(&1);
        assert_eq!(pos, Some(0));
        let pos = vec.lower_bound(&-1000);
        assert_eq!(pos, Some(0));

        let pos = vec.upper_bound(&1);
        assert_eq!(pos, Some(2));
        let pos = vec.upper_bound(&-1000);
        assert_eq!(pos, Some(0));

        // check boundary (big value)
        let pos = vec.lower_bound(&100);
        assert_eq!(pos, Some(vec.len() - 2));
        let pos = vec.lower_bound(&100000);
        assert_eq!(pos, None);

        let pos = vec.upper_bound(&100);
        assert_eq!(pos, None);
        let pos = vec.upper_bound(&100000);
        assert_eq!(pos, None)
    }
}
