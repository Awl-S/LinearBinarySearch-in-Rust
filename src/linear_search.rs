
pub fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    for i in 0..arr.len() {
        if arr[i] == target {
            return Some(i);
        }
    }

    #[cfg(debug_assertions)]
    println!("Debug linear_search: элемент '{}' не найден в списке", target);

    None
}

#[test]
fn test_linear_search_found() {
    let array = [1, 3, 5, 7, 9];
    let target = 5;
    let result = linear_search(&array, target);
    assert_eq!(result, Some(2));
}

#[test]
fn test_linear_search_not_found() {
    let array = [1, 3, 5, 7, 9];
    let target = 6;
    let result = linear_search(&array, target);
    assert_eq!(result, None);
}
