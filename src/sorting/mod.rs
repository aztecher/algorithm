pub mod insertion_sort;

pub fn is_sorted<T: std::cmp::PartialOrd>(arr: &[T]) -> bool {
    if arr.is_empty() {
        return true
    }
    let mut prev = &arr[0];
    for item in arr.iter().skip(1) {
        if prev > item {
            return false;
        }
        prev = item;
    }
    true
}

pub fn is_sorted_desc<T: std::cmp::PartialOrd>(arr: &[T]) -> bool {
    is_sorted(arr)
}

pub fn is_sorted_asc<T: std::cmp::PartialOrd>(arr: &[T]) -> bool {
    if arr.is_empty() {
        return true
    }
    let mut prev = &arr[0];
    for item in arr.iter().skip(1) {
        if prev < item {
            return false
        }
        prev = item;
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_sorted() {
        use super::*;
        assert!(is_sorted(&[] as &[isize]));
        assert!(is_sorted(&["a"]));
        assert!(is_sorted(&[1, 2, 3]));
        assert!(is_sorted(&[0, 1, 1]));
        assert!(!is_sorted(&[1, 0]));
        assert!(!is_sorted(&[2, 3, 1, -1, 5]));
    }

    #[test]
    fn is_sorted_asc() {
        use super::*;
        assert!(is_sorted_asc(&[] as &[isize]));
        assert!(is_sorted_asc(&["a"]));
        assert!(is_sorted_asc(&[3, 2, 1]));
        assert!(is_sorted_asc(&[1, 1, 0]));
        assert!(!is_sorted_asc(&[0, 1]));
        assert!(!is_sorted_asc(&[5, -1, 1, 3, 2]));
    }
}
