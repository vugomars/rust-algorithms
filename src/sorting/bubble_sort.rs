fn bubble_sort<T: Ord>(arr: &mut [T]) { // n^2
    if arr.is_empty() {
        return;
    }
    let n = arr.len();
    for i in 0..n { // O^n
        for j in 0..n - 1 -i { // O^n
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn bubble_sort_sol2<T: Ord>(arr: &mut [T]) { // n^2
    if arr.is_empty() {
        return;
    }
    let mut n = arr.len();
    let mut is_sorted = false;
    while !is_sorted {
        is_sorted = true;
        for i in 0..n - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                is_sorted = false;
            }
        }
        n -= 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_bubble_sort() {
        use super::*;
        let mut arr = [5,3,2,8,1,4,7,6,9];
        bubble_sort_sol2(&mut arr);
        assert_eq!(arr, [1,2,3,4,5,6,7,8,9]);
    }
}
