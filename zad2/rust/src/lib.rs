use std::{fmt::Debug, mem};

pub fn quicksort<T: Ord + Debug>(arr: &mut [T]) {
    qs_inner(arr)
}

fn qs_inner<T: Ord + Debug>(arr: &mut [T]) {
    match arr.len() {
        0 | 1 => return,
        2 => {
            if arr[0] > arr[1] {
                arr.swap(0, 1);
            }
            return;
        }
        _ => {}
    }
    // `split_first_mut` uses arcane, `unsafe` magic to consume the slice we provide and returns both references to
    // its first element and its subslice, consisting of the rest of elements after the first one. In rustland, we
    // can't have multiple mutable references into a slice, so we need to do this to please the borrow checker.
    let (pivot, rest) = arr.split_first_mut().expect("slice is non-empty");
    let mut left = 0;
    let mut right = (rest.len() - 1) as isize;

    while left <= right {
        if &rest[left as usize] <= pivot {
            // already on the correct side
            left += 1;
        } else if &rest[right as usize] > pivot {
            // right happens to be on the right side, we can just decrement to awoid unnecessary swaps
            println!(
                "pivot = {:?}, left = {:?}, right = {:?}, arr = {:?}",
                pivot, left, right, rest
            );
            right -= 1;
        } else {
            // element on the right is smaller or equal to pivot, on the left is bigger, swap them and advance from both
            // directions
            rest.swap(left as usize, right as usize);
            right -= 1;
            left += 1;
        }
    }

    println!(
        "pivot = {:?}, left = {:?}, right = {:?}, arr = {:?}",
        pivot, left, right, rest
    );

    // l = 7
    // r = 6
    // [6, 4, 3, 0, 1, 5, 2, 7, 9, 8]
    arr.swap(0, left as usize);
    let (left, right) = arr.split_at_mut(left as usize);
    qs_inner(left);
    qs_inner(&mut right[1..]);
}

// Now only works for unsigned integers
pub fn radixsort(arr: &mut [u32]) {
    const BITS: u32 = 4;
    const BASE: usize = 2usize.pow(BITS);
    let mut current_pos = 0;
    let mut output: Vec<u32> = vec![0; arr.len()];

    // get the max element so we can stop before doing maximum rounds for a type
    // for int, it would be 32bits / 4bits = 8 rounds
    let max = *arr.iter().max().unwrap();

    while dbg!(2u32.pow((current_pos) * BITS)) <= max {
        let mut counters: [usize; BASE] = [0; BASE];
        for i in arr.iter() {
            let digit = ((*i as usize) >> (current_pos * BITS)) & (BASE - 1);
            counters[digit] += 1;
        }

        for i in 1..counters.len() {
            counters[i] += counters[i - 1];
        }

        for x in arr.iter().rev() {
            let digit = ((*x as usize) >> (current_pos * BITS)) & (BASE - 1);
            counters[digit] -= 1;
            output[counters[digit]] = *x;
        }

        dbg!(&output);
        arr.swap_with_slice(output.as_mut_slice());
        current_pos += 1;
    }

    if current_pos % 2 != 0 {
        arr.swap_with_slice(output.as_mut_slice());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quicksort_works() {
        let mut arr = [74, 254, 140, 213, 18, 156, 21, 164, 68, 155];
        quicksort(&mut arr);
        assert_eq!(arr, [18, 21, 68, 74, 140, 155, 156, 164, 213, 254]);
    }

    #[test]
    fn radixsort_works_short_sequence() {
        let mut arr = [74, 254, 140, 213, 18, 156, 21, 164, 68, 155];
        radixsort(&mut arr);
        assert_eq!(arr, [18, 21, 68, 74, 140, 155, 156, 164, 213, 254]);
    }

    #[test]
    fn radixsort_works_long_sequence() {
        let mut arr = [
            24214, 38156, 52288, 17970, 26656, 28915, 35949, 6995, 34557, 8092, 3743, 18150, 11916,
            63241, 100, 41863, 10408, 12799, 16333, 10648,
        ];
        radixsort(&mut arr);
        assert_eq!(
            arr,
            [
                100, 3743, 6995, 8092, 10408, 10648, 11916, 12799, 16333, 17970, 18150, 24214,
                26656, 28915, 34557, 35949, 38156, 41863, 52288, 63241
            ]
        );
    }
}
