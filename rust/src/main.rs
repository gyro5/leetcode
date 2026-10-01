#![allow(unused)]

// One mod for each problem
mod nlogn_interval_intersection_count;
mod p1096_brace_grammer_expansion;
mod p1111_max_paren_split_depth;
mod p1190_wormhole;
mod p1235_max_weighted_interval_scheduling;
mod p1520_max_non_overlapping_substr;
mod p1658_max_subarray_equal_sum;
mod p1751_k_interval_scheduling;
mod p1807_string_replacement;
mod p2267_grid_dfs;
mod p3498_reverse_degree_str;
mod p3524_subarray_remainder;
mod p3525_segment_tree;
mod p3550;
mod p3_sliding_window_basic;
mod p67_add_binary_str;

mod temp;

fn main() {
    println!(
        "{:?}",
        temp::Solution::add_binary("11".to_owned(), "1".to_owned())
    );
}
