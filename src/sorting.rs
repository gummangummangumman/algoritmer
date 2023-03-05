use std::collections::VecDeque;

//Just the in-built rust sorting algorithm
pub fn default_sort(v: Vec<i32>) -> Vec<i32> {
    let mut vec = v;
    vec.sort();
    vec
}

//Just the in-built rust unstable sorting algorithm
pub fn default_unstable_sort(v: Vec<i32>) -> Vec<i32> {
    let mut vec: Vec<i32> = v;
    vec.sort_unstable();
    vec
}

pub fn bubble_sort(v: Vec<i32>) -> Vec<i32> {
    let mut vec: Vec<i32> = v;
    if vec.len() <= 1 {
        return vec;
    }
    let mut number_to_fix = vec.len() - 1;
    let mut current_biggest = (0, 0); //position, value
    while number_to_fix > 0 {
        for i in 0..(number_to_fix + 1) {
            if vec[i] > current_biggest.1 {
                current_biggest = (i, vec[i]);
            }

            if i == number_to_fix {
                vec[current_biggest.0] = vec[number_to_fix];
                vec[number_to_fix] = current_biggest.1;
                current_biggest = (0, 0);
                number_to_fix -= 1;
            }
        }
    }

    vec
}

pub fn merge_sort(v: Vec<i32>) -> Vec<i32> {
    let vec: Vec<i32> = v;
    if vec.len() <= 1 {
        return vec;
    }

    let (left, right) = vec.split_at(vec.len() / 2);

    let left_sorted = merge_sort(left.to_vec());
    let right_sorted = merge_sort(right.to_vec());

    return merge(left_sorted.to_vec(), right_sorted.to_vec());
}

fn merge(l: Vec<i32>, r: Vec<i32>) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::with_capacity(l.len() + r.len());
    //converting left and right to VecDeques to use pop_front
    let (mut left, mut right) = (VecDeque::new(), VecDeque::new());
    left.extend(l);
    right.extend(r);
    while !left.is_empty() && !right.is_empty() {
        if left[0] < right[0] {
            vec.push(left.pop_front().unwrap())
        } else {
            vec.push(right.pop_front().unwrap())
        }
    }
    vec.extend(left);
    vec.extend(right);
    vec
}

pub fn quicksort(v: Vec<i32>) -> Vec<i32> {
    let vec: Vec<i32> = v;
    if vec.len() <= 1 {
        return vec;
    }
    let pivot = vec[0];
    let mut left:Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for i in 1..vec.len() {
        if vec[i] < pivot {
            left.push(vec[i]);
        } else {
            right.push(vec[i]);
        }
    }
    left = quicksort(left);
    right = quicksort(right);
    left.into_iter().chain(vec![pivot]).chain(right).collect()
}

// optimised version - passing by reference and using indices instead of creating new vectors
pub fn quicksort_v2(v: Vec<i32>) -> Vec<i32> {
    let mut vec: Vec<i32> = v;
    quicksort_recursive(&mut vec);
    vec
}

fn quicksort_recursive(v: &mut [i32]) {
    if v.len() <= 1 {
        return;
    }
    let pivot: i32 = v[0];
    let mut i = 0;
    for j in 1..v.len() {
        if v[j] < pivot {
            i += 1;
            v.swap(i, j);
        }
    }
    v.swap(0, i);
    quicksort_recursive(&mut v[..i]);
    quicksort_recursive(&mut v[(i + 1)..]);
}

