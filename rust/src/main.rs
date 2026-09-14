// One mod for each problem
mod p1235_max_weighted_interval_scheduling;
mod p1751_k_interval_scheduling;

fn main() {
    let x = vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 10]];
    println!("{:?}", p1751_k_interval_scheduling::Solution::max_value(x, 2));
}
