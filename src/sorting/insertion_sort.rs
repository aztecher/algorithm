pub fn insertion_sort<T: Ord + Copy>(input: &mut [T]) {
    for j in 1..input.len() {
        let key = input[j];
        // 'i' will be decremented in while loop and 'i' can be '-1' value in this loop.
        // but while condition requires
        //   1. 'i' is 0,1,2....n
        //   2. 'i' is 'usize' type (-1 is not the element of 'usize')
        // so, we have to...
        //   1. define i as 'i8' and cast it as usize when it will be used as index
        //   2. add while condition i >= 0 because the condition i = -1 is no more evalting
        let mut i = (j - 1) as i8;
        while j > 0 && i >= 0 && input[(i as usize)] > key {
            input[((i+1) as usize)] = input[(i as usize)];
            i = i - 1;
        }
        input[((i+1) as usize)] = key
    }
}

pub fn insertion_sort_asc<T: Ord + Copy>(input: &mut [T]) {
    for j in 1..input.len() {
        let key = input[j];
        let mut i = (j - 1) as i8;
        while j > 0 && i >= 0 && input[(i as usize)] < key {
            input[((i+1) as usize)] = input[(i as usize)];
            i = i - 1;
        }
        input[((i+1) as usize)] = key
    }
}

#[cfg(test)]
mod tests {
    use super::super::{is_sorted,is_sorted_asc};
    use super::*;

    #[test]
    fn empty() {
        let mut arr: [u8; 0] = [];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn empty_asc() {
        let mut arr: [u8; 0] = [];
        insertion_sort_asc(&mut arr);
        assert!(is_sorted_asc(&arr));
    }

    #[test]
    fn one_element() {
        let mut arr: [char; 1] = ['a'];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn one_element_asc() {
        let mut arr: [char; 1] = ['a'];
        insertion_sort_asc(&mut arr);
        assert!(is_sorted_asc(&arr));
    }

    #[test]
    fn already_sorted() {
        let mut arr: [&str; 3] = ["a", "b", "c"];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn already_sorted_asc() {
        let mut arr: [&str; 3] = ["c", "b", "a"];
        insertion_sort_asc(&mut arr);
        assert!(is_sorted_asc(&arr));
    }

    #[test]
    fn basic() {
        let mut arr: [&str; 4] = ["d", "a", "c", "b"];
        println!("before = {:?}", arr);
        insertion_sort(&mut arr);
        println!("after = {:?}", arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn basic_asc() {
        let mut arr: [&str; 4] = ["a", "d", "c", "b"];
        insertion_sort_asc(&mut arr);
        assert!(is_sorted_asc(&arr));
    }

    #[test]
    fn odd_number_of_elements() {
        let mut arr: Vec<&str> = vec!["d", "a", "c", "e", "b"];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn odd_number_of_elements_asc() {
        let mut arr: Vec<&str> = vec!["d", "a", "c", "e", "b"];
        insertion_sort_asc(&mut arr);
        assert!(is_sorted_asc(&arr));
    }

    #[test]
    fn repeated_elements() {
        let mut arr: Vec<usize> = vec![542, 542, 542, 542];
        insertion_sort(&mut arr);
        assert!(is_sorted(&mut arr));
    }

    #[test]
    fn repeated_elements_asc() {
        let mut arr: Vec<usize> = vec![542, 542, 542, 542];
        insertion_sort_asc(&mut arr);
        assert!(is_sorted_asc(&mut arr));
    }
}
