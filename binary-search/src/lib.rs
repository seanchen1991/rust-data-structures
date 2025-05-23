use std::cmp::*;

/// Defines the ability for a container type containing sorted elements
/// to search for an element in logarithmic time.
pub trait BinarySearch<'a, T: Ord> {
    /// Performs the binary search, returning a reference to the target
    /// element in the container if it is found.
    fn binary_search(&'a self, target: T) -> Option<&'a T>;

    /// Checks that the container type is indeed sorted.
    /// Note that this devolves the binary search to linear time.
    fn is_sorted(&self) -> bool;
}

impl<'a, T: Ord> BinarySearch<'a, T> for Vec<T> {
    fn is_sorted(&self) -> bool {
        self.iter().zip(self.iter().skip(1)).all(|(a, b)| a <= b)
    }

    fn binary_search(&'a self, target: T) -> Option<&'a T> {
        if !self.is_sorted() {
            return None;
        }

        let mut left = 0;
        let mut right = self.len() - 1;

        while left <= right {
            let mid = left + (right - left) / 2;

            if let Some(val) = self.get(mid) {
                match (*val).cmp(&target) {
                    Ordering::Equal => return Some(val),
                    Ordering::Less => left = mid + 1,
                    Ordering::Greater => right = mid - 1,
                }
            } else {
                break;
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsorted_vector_should_return_none() {
        let input = vec![18, 23, 9, 1, 10, 4];

        assert_eq!(input.binary_search(4), None);
    }

    #[test]
    fn should_correctly_find_element() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        assert_eq!(input.binary_search(4), Some(&4));
    }

    #[test]
    fn should_return_none_when_target_does_not_exist() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        assert_eq!(input.binary_search(11), None);
    }
}
