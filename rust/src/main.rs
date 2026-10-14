#![allow(unused)]

// One mod for each problem
mod nlogn_interval_intersection_count;
mod p1096_brace_grammer_expansion;
mod p1111_max_paren_split_depth;
mod p1114_thread_ordering;
mod p1190_wormhole;
mod p1235_max_weighted_interval_scheduling;
mod p1520_max_non_overlapping_substr;
mod p1541_double_closing_bracket;
mod p1658_max_subarray_equal_sum;
mod p1751_k_interval_scheduling;
mod p1807_string_replacement;
mod p2267_grid_dfs;
mod p22_generate_parenthesis;
mod p2333_min_sum_sq_diff;
mod p32_longest_parentheses_substr;
mod p3498_reverse_degree_str;
mod p3524_subarray_remainder;
mod p3525_segment_tree;
mod p3550;
mod p3723_maximize_sq_sum_digit;
mod p3727_max_alternating_sum;
mod p3_sliding_window_basic;
mod p678_valid_parenthesis_with_wildcard;
mod p67_add_binary_str;
mod p856_parenthesis_score;

mod temp;

fn main() {
    println!(
        "{:?}",
        p2333_min_sum_sq_diff::Solution::min_sum_square_diff(
            vec![1, 4, 10, 12],
            vec![5, 8, 6, 9],
            1,
            1
        )
    );
}
