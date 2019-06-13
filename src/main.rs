use rand::Rng;

fn swap(arr: &mut [i32], i: i32, j: i32) {
    let i = i as usize;
    let j = j as usize;
    let tmp = arr[j];
    arr[j] = arr[i];
    arr[i] = tmp;
}

fn partition(arr: &mut [i32], low: i32, high: i32) -> i32 {
    let pivot = arr[high as usize];
    let mut i:i32 = low as i32 - 1;
    for j in low..high {
        if arr[j as usize] <= pivot {
            i += 1;
            swap(arr, i, j);

        }
    }
    swap(arr, i + 1, high);
    (i + 1)
}

fn quick_sort(arr: &mut [i32], low: i32, high: i32) {

    if low < high {
        let pi = partition(arr, low, high);
        quick_sort(arr, low, pi - 1);
        quick_sort(arr, pi + 1, high);
    }
}


fn main() {
    let mut rng = rand::thread_rng();
    let mut data = Vec::<i32>::new();
    for _ in 0..100 {
        data.push(rng.gen_range(0, 1000));
    }
    let high = data.len() as i32 - 1;
    println!("before sort: {:?}", data);
    quick_sort(&mut data, 0, high);
    println!("after sort: {:?}", data);
}
