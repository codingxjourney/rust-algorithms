
pub fn linear_search<T: PartialEq>(item: &T, arr: &[T]) -> Option<usize> {
    for (i, data) in arr.iter().enumerate() {
        if item == data {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_strings() {
        let index = linear_search(&"x", &vec!["a", "b", "c", "x", "y", "z"]);
        assert_eq!(index, Some(0));
    }

    #[test] 
    fn search_ints() {
        let index = linear_search(&7, &vec![1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(index, Some(6));

        let index = linear_search(&3, &vec![1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(index, Some(2));

        let index = linear_search(&2, &vec![1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(index, Some(1));

        let index = linear_search(&1, &vec![1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn not_found() {
        let index = linear_search(&8, &vec![1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(index, None);
    }

    #[test]
    fn empty() {
        let index = linear_search(&7, &vec![]);
        assert_eq!(index, None);
    }
}