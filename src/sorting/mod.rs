mod bubble_sort;

pub fn is_sorted<T: Ord>(arr: &[T]) -> bool {
    if arr.is_empty(){
        return true;
    }
    let mut previous = &arr[0];

    for current in arr.iter().skip(1) {
        if previous > current {
            return false;
        }
        previous = current;
    }
    true
}
