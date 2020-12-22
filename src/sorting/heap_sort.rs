/// 堆排序
pub fn heap_sort(arr: &mut [i32]) {
    if arr.is_empty() {
        return
    }
    let end = arr.len();
    for start in (0..end / 2).rev() {
        sift_down(arr, start, end - 1);
    }

    for end in (1..arr.len()).rev() {
        arr.swap(end, 0);
        sift_down(arr, 0, end - 1);
    }
}

fn sift_down(arr: &mut [i32], start: usize, end: usize) {
    let mut root = start;
    loop {
        let mut child = root * 2 + 1;
        if child > end {
            break;
        }
        if child + 1 <= end && arr[child] < arr[child + 1] {
            child += 1;
        }
        if arr[root] < arr[child] {
            arr.swap(child, root);
            root = child;
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test(){
        println!("{}",4);
    }

    #[test]
    fn empty() {
        let mut arr: Vec<i32> = Vec::new();
        heap_sort(&mut arr);
        assert_eq!(arr, &[]);
    }
}