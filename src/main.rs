use std::time::{Duration, Instant};
use rand::SeedableRng;

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

fn main() {
    let (sorted, t) = timed(|| quicksort(gen_vec(1_000_000)));
    println!("quick_sort: sorted = {}, t = {}s", is_sorted(sorted), t.as_secs_f64());

}