use rand::Rng;
use std::time::Instant;
mod sorting;

const LENGTH: usize = 20_000;
const IS_PRINTING_VECTORS: bool = LENGTH < 50;

fn main() {
    let vector = get_vector_of_random_numbers(LENGTH);
    print_pretty("unsorted", &vector);

    let sort_timer = Instant::now();
    let sorted = sorting::default_sort(vector.clone());
    println!("default sort spent {}ms", sort_timer.elapsed().as_millis());
    print_pretty("default sort", &sorted);

    let sort_timer = Instant::now();
    let sorted = sorting::default_unstable_sort(vector.clone());
    println!("default unstable sort spent {}ms", sort_timer.elapsed().as_millis());
    print_pretty("default unstable sort", &sorted);

    let sort_timer = Instant::now();
    let sorted = sorting::merge_sort(vector.clone());
    println!("merge sort spent {}ms", sort_timer.elapsed().as_millis());
    print_pretty("merge sort", &sorted);

    let sort_timer = Instant::now();
    let sorted = sorting::quicksort(vector.clone());
    println!("quicksort spent {}ms", sort_timer.elapsed().as_millis());
    print_pretty("quicksort", &sorted);

    let sort_timer = Instant::now();
    let sorted = sorting::quicksort_v2(vector.clone());
    println!("quicksort v2 spent {}ms", sort_timer.elapsed().as_millis());
    print_pretty("quicksort v2", &sorted);

    let sort_timer = Instant::now();
    let sorted = sorting::bubble_sort(vector.clone());
    println!("bubble sort spent {}ms", sort_timer.elapsed().as_millis());
    print_pretty("bubble sort", &sorted);
}

fn get_vector_of_random_numbers(length: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::with_capacity(length);

    for _ in 0..length {
        vec.push(rng.gen_range(0..=100));
    }

    vec
}

fn print_pretty(name: &str, vec: &Vec<i32>) {
    if !IS_PRINTING_VECTORS {
        return
    }
    println!("{name}:");
    for (i, num) in vec.iter().enumerate() {
        if i == 0 {
            print!("{}", num);
        } else {
            print!(", {}", num);
        }
    }
    println!();
}