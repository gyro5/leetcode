struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused)]
impl Solution {
    pub fn count_intersecting_intervals(mut intervals: Vec<Vec<i32>>) -> i64 {
        let mut count: i64 = 0;
        
        // Firstly, sort the intervals by start time
        intervals.sort_by_key(|interval| interval[0]);
        
        // Keep a min heap of finish time
        // Rust BinaryHeap is max-heap by default, so Reverse is needed
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

        /*     
        The main idea of the loop: For each start time, count the number
        of intervals that overlap with it and with no earlier start time.

        The min heap acts like the outer loop of the O(n^2) solution,
        while the for loop acts like the inner loop. For each item in the
        min heap, its presence will be counted once for each interval
        that is after it in the sequence and intersects with it.

        The min heap allows us to not have to check an interval after we
        "move" past its start time (by sorting by start time and pop heap
        at the start of each iteration).
        */
        for i in intervals.iter() {
            // After this loop, the heap will contain all end times >= i[0]
            // --> All of these intervals will overlap with i
            while let Some(Reverse(f)) = heap.peek() && *f < i[0] {
                heap.pop();
            }

            // All intervals that start after i[0] and haven't ended before that
            count += heap.len() as i64;
            heap.push(Reverse(i[1]));
        }

        count
    }
}
