pub fn bubble_sort(array: &mut Vec<u32>) {
    let mut n = array.len();
    while n > 1 {
        let mut new_n = 0;
        for i in 1..=(n - 1) {
            if array[i - 1] > array[i] {
                array.swap(i - 1, i);
                new_n = i;
            }
        }
        n = new_n;
    }
}