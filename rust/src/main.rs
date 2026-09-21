mod temp;

// One mod for each problem
mod p1235_max_weighted_interval_scheduling;
mod p1520_max_non_overlapping_substr;
mod p1751_k_interval_scheduling;
mod p3498_reverse_degree_str;
mod p3524_subarray_remainder;
mod p3_sliding_window_basic;

mod nlogn_interval_intersection_count;

fn main() {
    println!(
        "{:?}",
        p3524_subarray_remainder::Solution::result_array(vec![1, 2, 3, 4, 5], 3)
    );
}
