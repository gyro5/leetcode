#![allow(unused)]

// One mod for each problem
mod p1235_max_weighted_interval_scheduling;
mod p1520_max_non_overlapping_substr;
mod p1658_max_subarray_equal_sum;
mod p1751_k_interval_scheduling;
mod p3498_reverse_degree_str;
mod p3524_subarray_remainder;
mod p3525_segment_tree;
mod p3_sliding_window_basic;

mod nlogn_interval_intersection_count;

fn main() {
    println!(
        "{:?}",
        p1658_max_subarray_equal_sum::Solution::min_operations(vec![3, 2, 20, 1, 1, 3], 10)
    );
}
