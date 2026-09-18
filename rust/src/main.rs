// One mod for each problem
mod p1235_max_weighted_interval_scheduling;
mod p1751_k_interval_scheduling;
mod p1520_max_non_overlapping_substr;

fn main() {
    println!("{:?}", p1520_max_non_overlapping_substr::Solution::max_num_of_substrings("eaaeeaebcb".to_string()));
}
