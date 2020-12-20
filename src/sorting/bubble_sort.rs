/// 冒泡排序
pub fn bubble_sort(arr: &mut [i32]) {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..arr.len() {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swapped = true;
            }
        }
    }
}