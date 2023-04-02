pub fn insertion_sort<T: Ord + Copy>(input: &mut [T]) {
    for j in 2..input.len() {
        let key = input[j];
        let mut i = j - 1;
        while j > 0 && input[i] > key {
            input[i+1] = input[i];
            i = i - 1;
        }
        input[i+1] = key
    }
}

#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::*;

    #[test]
    fn empty() {
        let mut arr: [u8; 0] = [];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }
}
