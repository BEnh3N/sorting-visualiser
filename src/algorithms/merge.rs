pub fn merge_sort(array: &mut Vec<u32>) {
    if array.len() < 2 {
        return
    }
    let mid = array.len() / 2;
    let mut left = array[0..mid].to_vec();
    let mut right = array[mid..].to_vec();
    merge_sort(&mut left);
    merge_sort(&mut right);
    merge(&left, &right, array);
}

fn merge(left: &Vec<u32>, right: &Vec<u32>, merged: &mut Vec<u32>) {
    let mut i = 0;
    let mut j = 0;
    merged.clear();

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i = i + 1;
        } else {
            merged.push(right[j]);
            j = j + 1;
        }
    }

    if i < left.len() {
        while i < left.len() {
            merged.push(left[i]);
            i = i + 1;
        }
    }

    if j < right.len() {
        while j < right.len() {
            merged.push(right[j]);
            j = j + 1;
        }
    }
}