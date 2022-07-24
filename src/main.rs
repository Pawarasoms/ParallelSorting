use std::time::{Duration, Instant};
use rand::SeedableRng;
use rayon::prelude::*;
use crossbeam;

// Sequential Quick sort
fn quicksort (mut arr: Vec<i32>) -> Vec<i32>{

    if arr.len() <= 1 {
        return arr.to_vec();
    }

    let pivot = arr.pop().unwrap();

    let mut left = Vec::new();
    let mut right = Vec::new();

    arr.iter().for_each(|element| {
        if *element > pivot {
            right.push(*element);
        } else {
            left.push(*element);

        }
    });

    let mut sorted = quicksort(left);
    let mut right_sorted = quicksort(right);
    let mut pivot_vec = vec![pivot];

    sorted.append(&mut pivot_vec);
    sorted.append(&mut right_sorted);

    return sorted;
}

// Parallel Quick sort
fn par_quicksort(mut arr: Vec<i32>) -> Vec<i32>{

    if arr.len() <= 1 {
        return arr.to_vec();
    }

    let pivot = arr.pop().unwrap();

    let mut left = Vec::new();
    let mut right = Vec::new();

    arr.iter().for_each(|element| {
        if *element > pivot {
            right.push(*element);
        } else {
            left.push(*element);
        }
    });

    let (mut sorted, mut right_sorted) = rayon::join(|| par_quicksort(left),
                                                     || par_quicksort(right));

    let mut pivot_vec = vec![pivot];

    sorted.append(&mut pivot_vec);
    sorted.append(&mut right_sorted);

    return sorted;

}



fn timed<R, F>(f: F) -> (R, Duration) where F: Fn() -> R {
    let starting_point = Instant::now();
    let res = f();
    (res, starting_point.elapsed())
}

fn gen_vec(n: usize) -> Vec<i32> {
    use rand::Rng;
    use rand::distributions::Standard;
    let rng = rand::rngs::StdRng::seed_from_u64(0x12345);
    rng.sample_iter(&Standard).take(n).collect()
}

fn is_sorted(xs: Vec<i32>) -> bool {
    if xs.len() <= 1 { return true; }
    let mut prev = &xs[0];
    for it in xs.iter().skip(1) {
        if prev > it { return false; }
        prev = it;
    }
    return true;
}

// Sequential sample sort
fn sample_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let mut temp = 0;
    for i in 0..arr.len() {
        for j in (1..arr.len()).rev() {
            if arr[j] < arr[j - 1] {
                temp = arr[j];
                arr[j] = arr[j - 1];
                arr[j - 1] = temp;
            }
        }
    }
    arr
}

// Parallel sample sort
fn par_sample_sort(mut arr: Vec<i32>, thr: usize) -> Vec<i32>{
    let len = arr.len();
    let chunks = std::cmp::min(len, thr);
    let _ = crossbeam::scope(|scope| {
        for bucket in arr.chunks_mut(len / chunks) {
            scope.spawn(move |_| quicksort(bucket.to_vec()));
        }
    });
    quicksort(arr)
}

// Sequential radix sort
fn radix_sort(mut arr: Vec<i32>) -> Vec<i32> {
    for bit in 0..31 {
        let (small, big): (Vec<_>, Vec<_>) = arr.iter().partition(|&&x| (x >> bit) & 1 == 0);
        let (left, right) = arr.split_at_mut(*&small.len());
        left.clone_from_slice(&small);
        right.clone_from_slice(&big);
    }
    let (negative, positive): (Vec<_>, Vec<_>) = arr.iter().partition(|&&x| x < 0);
    let (left, right) = arr.split_at_mut(*&negative.len());
    left.clone_from_slice(&negative);
    right.clone_from_slice(&positive);
    arr
}



fn main() {
    println!("===== Quick sort =====");
    let (sorted, t) = timed(|| quicksort(gen_vec(1_000_000)));
    println!("Sequential quick sort: sorted = {}, t = {}s", is_sorted(sorted), t.as_secs_f64());
    let (sorted, t) = timed(|| par_quicksort(gen_vec(1_000_000)));
    println!("Parallel quick sort: sorted = {}, t = {}s", is_sorted(sorted), t.as_secs_f64());
    println!("===== Sample sort =====");
    let (sorted, t) = timed(|| sample_sort(gen_vec(10_000)));
    println!("Sequential sample sort: sorted = {}, t = {}s", is_sorted(sorted), t.as_secs_f64());
    let (sorted, t) = timed(|| par_sample_sort(gen_vec(10_000), 10_000));
    println!("Parallel sample sort: sorted = {}, t = {}s", is_sorted(sorted), t.as_secs_f64());
    println!("===== Radix sort =====");
    let (sorted, t) = timed(|| radix_sort(gen_vec(1_000_000)));
    println!("Sequential radix sort: sorted = {}, t = {}s", is_sorted(sorted), t.as_secs_f64());

}








