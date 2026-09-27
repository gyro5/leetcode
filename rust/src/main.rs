#![allow(unused)]

// One mod for each problem
mod nlogn_interval_intersection_count;
mod p1096_brace_grammer_expansion;
mod p1190_wormhole;
mod p1235_max_weighted_interval_scheduling;
mod p1520_max_non_overlapping_substr;
mod p1658_max_subarray_equal_sum;
mod p1751_k_interval_scheduling;
mod p1807_string_replacement;
mod p3498_reverse_degree_str;
mod p3524_subarray_remainder;
mod p3525_segment_tree;
mod p3550;
mod p3_sliding_window_basic;

fn main() {
    println!(
        "{:?}",
        p1190_wormhole::Solution::reverse_parentheses("(abcd)".to_owned())
    );
}
