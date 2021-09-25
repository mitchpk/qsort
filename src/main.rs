fn sort_internal<T: PartialOrd>(array: &mut [T], low: usize, high: usize) {
    if low < high {
        let pivot_location = partition(array, low, high);
        if pivot_location > 0 {
            sort_internal(array, low, pivot_location - 1);
        }
        sort_internal(array, pivot_location + 1, high);
    }
}

fn partition<T: Eq + PartialEq>(array: &mut [T], low: usize, high: usize) -> usize {
    let pivot = high;
    let mut index = low;

    for i in low..high {
        if array[i] < array[pivot] {
            array.swap(i, index);
            index += 1;
        }
    }

    array.swap(pivot, index);
    index
}

fn sort<T: PartialOrd>(array: &mut [T]) {
    sort_internal(array, 0, array.len() - 1)
}

const ITERATIONS: u32 = 100;
const TEST_MAX: u32 = 16;

fn main() {
    for i in 1..TEST_MAX + 1 {
        let n = (2 as usize).pow(i);
        let mut arr: Vec<i64> = Vec::new();
        for _ in 0..n {
            arr.push(rand::random());
        }
        let mut total_time_qsort = std::time::Duration::new(0, 0);
        let mut total_time_rust = std::time::Duration::new(0, 0);
        for _ in 0..ITERATIONS {
            let mut arr1 = arr.clone();
            let mut arr2 = arr.clone();
            let t1 = std::time::Instant::now();
            sort(&mut arr1);
            total_time_qsort += t1.elapsed();
            let t2 = std::time::Instant::now();
            arr2.sort();
            total_time_rust += t2.elapsed();
            assert!(arr1 == arr2);
        }
        print!(
            "Sorted {} elements in {} ns using qsort, ",
            n,
            (total_time_qsort / ITERATIONS).as_nanos()
        );
        println!(
            "{} ns using default rust sort",
            (total_time_rust / ITERATIONS).as_nanos()
        );
    }
}
