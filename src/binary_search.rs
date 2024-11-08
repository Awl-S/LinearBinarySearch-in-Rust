
pub fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left < right {
        let mid = (left + right) / 2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else if arr[mid] > target {
            right = mid - 1;
        }
    }
    #[cfg(debug_assertions)]
    println!("Debug binary_search: элемент '{}' не найден в списке", target);
    None
}

#[test]
fn test_binary_search_found() {
    let sorted_array = [1, 3, 5, 7, 9];
    let target = 7;
    let result = binary_search(&sorted_array, target);
    assert_eq!(result, Some(3));
}

#[test]
fn test_binary_search_not_found() {
    let sorted_array = [1, 3, 5, 7, 9];
    let target = 10;
    let result = binary_search(&sorted_array, target);
    assert_eq!(result, None);
}
