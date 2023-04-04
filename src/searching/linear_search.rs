pub fn linear_search<T: Ord>(input: &[T], value: T) -> Option<usize> {
    for i in 0..input.len() {
        if input[i] == value {
            return Some(i)
        }
    }
    return None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let arr: [u8; 0] = [];
        let res = linear_search(&arr, 0);
        assert!(res.is_none())
    }

    #[test]
    fn element_exist() {
        let arr: [u8; 3]= [1, 2, 3];
        let res = linear_search(&arr, 3);
        assert!(res.is_some());
        assert!(res.unwrap() == 2);
    }

    #[test]
    fn element_not_exist() {
        let arr: [&str; 3] = ["a", "b", "c"];
        let value = "d";
        let res = linear_search(&arr, value);
        assert!(res.is_none());
    }

    #[test]
    fn repeated_elements() {
        let arr: [&str; 3] = ["a", "a", "a"];
        let value = "a";
        let res = linear_search(&arr, value);
        assert!(res.unwrap() == 0);
    }
}
