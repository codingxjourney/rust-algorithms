use std::cmp::Ordering;

pub fn binary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;

        match item.cmp(&arr[mid]) {
            Ordering::Less => right = mid,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => left = mid + 1,
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_strings() {
        let searching_value = "x";
        let arr_list = vec!["r", "s", "t", "u", "v", "w", "x", "y", "z"];
        
        let search = binary_search(&searching_value, &arr_list);
        assert_eq!(search, Some(6));
    }

    #[test]
    fn search_ints() {
        let searching_value = 7;
        let arr_list = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        
        let search = binary_search(&searching_value, &arr_list);
        assert_eq!(search, Some(7));
    }

    #[test]
    fn empty_arr_list() {
        let searching_value = 7;
        let arr_list = vec![];
        
        let search = binary_search(&searching_value, &arr_list);
        assert_eq!(search, None);
    }

    #[test]
    fn searching_value_not_found() {
        let searching_value = 7;
        let arr_list = vec![0, 1, 2, 3, 4, 5];
        
        let search = binary_search(&searching_value, &arr_list);
        assert_eq!(search, None);
    }
    
}
