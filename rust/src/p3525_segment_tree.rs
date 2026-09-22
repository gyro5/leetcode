pub struct Solution;

// https://leetcode.com/problems/find-x-value-of-array-ii/
#[allow(unused)]
impl Solution {
    pub fn result_array(mut nums: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // Preprocess to prevent the big values overflowing (bc we only care about modulo)
        // NOTE: This prevented all sorts of problem with overflowing value in Rust lmao
        for num in nums.iter_mut() {
            *num %= k;
        }

        // Construct the segment tree
        let mut seg_tree = SegmentTree::new(&nums, k);

        queries
            .iter()
            .map(|query| {
                let idx = query[0];
                let new_value = query[1];
                let start = query[2];
                let x = query[3];

                seg_tree.update(idx as usize, new_value);
                // Need the number of prefix of nums[start:] for this x
                if k == 1 {
                    nums.len() as i32 - start
                }
                else {
                    seg_tree.get_x_value(start as usize, nums.len() - 1, x as usize)
                }
            })
            .collect()
    }
}

// NOTE: Compared to p3524 which has only 1 query so dp is the best choice, this problem has the
// same idea of maintaining an array of remainder count, but here it's in each segment tree node
// instead of in the dp memo.

// This problem requires the use of Segment Tree, which is a special data structure that
// allows array range queries (such as sum of all elements in [3:8]) to be done in
// O(logn). Reference: https://cp-algorithms.com/data_structures/segment_tree.html
//
// ALSO: this is not the best implementation of Segment Tree. A better one would use a backing
// array, as segment trees are complete (similar to heaps). But this version makes it really clear
// about the logic of the algorithm.
struct SegmentTree {
    head: SegmentTreeNode,
}

impl SegmentTree {
    fn new(nums: &[i32], k: i32) -> Self {
        SegmentTree {
            head: SegmentTreeNode::new(nums, 0, nums.len() - 1, k as usize),
        }
    }

    fn update(&mut self, index: usize, new_value: i32) {
        self.head.update(index, new_value);
    }

    fn get_x_value(&self, ql: usize, qr: usize, x: usize) -> i32 {
        let res = self.head.range_query(ql, qr);
        res.0[x]
    }
}

// Each node store the value for the range [l..=r], in this case the product of all
// elements in the range.
#[derive(Debug)]
struct SegmentTreeNode {
    l: usize,
    r: usize,
    value: i32,
    left: Option<Box<SegmentTreeNode>>,
    right: Option<Box<SegmentTreeNode>>,
    k_count: Vec<i32>, /* problem-specific augmentation, should be the count of
                        * prefix of the range for each remainder of k */
}

impl SegmentTreeNode {
    fn new(nums: &[i32], l: usize, r: usize, k: usize) -> Self {
        if l > r {
            panic!("Got l={l} > r={r}, this should not happen.")
        }
        else if l == r {
            let mut k_count = vec![0; k];
            k_count[nums[l] as usize % k] = 1;

            SegmentTreeNode {
                l,
                r,
                value: nums[l],
                left: None,
                right: None,
                k_count: k_count,
            }
        }
        else {
            let mid = (l + r) / 2;
            let left = Self::new(nums, l, mid, k);
            let right = Self::new(nums, mid + 1, r, k);
            let mut k_count = left.k_count.clone();
            for i in 0..k {
                k_count[left.value as usize * i % k] += right.k_count[i];
            }

            SegmentTreeNode {
                l,
                r,
                value: left.value * right.value % k as i32,
                left: Some(Box::new(left)),
                right: Some(Box::new(right)),
                k_count: k_count,
            }
        }
    }

    // Should return the k_count[x] count for all prefixes of [ql..=qr]
    fn range_query(&self, ql: usize, qr: usize) -> (Vec<i32>, i32) {
        let k = self.k_count.len();

        // Case 1: self.[l..=r] completely in [ql..=qr].
        if ql <= self.l && self.r <= qr {
            (self.k_count.clone(), self.value)
        }
        // Case 2: No overlap -> This segment contributes the identity element (aka 1)
        else if ql > self.r || qr < self.l {
            (vec![0; k], 1)
        }
        // Case 3: self.[l..=r] partially covers [ql..=qr]
        else {
            // Prefix of left sub-range is also prefix of current range
            let (res_left, pleft) = self.left.as_ref().unwrap().range_query(ql, qr);
            let (res_right, pright) = self.right.as_ref().unwrap().range_query(ql, qr);

            let mut res = res_left;
            for i in 0..k {
                // Take the part of the left sub-range that is within the query range with
                // prefix of right sub-range to get the remaining prefixes of this range
                res[pleft as usize * i % k] += res_right[i];
            }

            (res, pleft * pright)
        }
    }

    fn update(&mut self, index: usize, new_value: i32) {
        let k = self.k_count.len();

        // Base case: The leaf node of the updated index
        if self.l == index && self.r == index {
            self.k_count[self.value as usize % k] = 0;
            self.value = new_value % k as i32;
            self.k_count[self.value as usize % k] = 1;
        }
        // Recursive case: Delegate to the child that contains the index
        else {
            if index <= (self.l + self.r) / 2 {
                self.left.as_mut().unwrap().update(index, new_value);
            }
            else {
                self.right.as_mut().unwrap().update(index, new_value);
            };

            let left = self.left.as_ref().unwrap();
            let right = self.right.as_ref().unwrap();

            // Recalculate the product of this range
            self.value = left.value * right.value % k as i32;

            // Update k_count for this range
            self.k_count.copy_from_slice(&left.k_count);

            for r in 0..k {
                // Take full left sub-range with prefix of right sub-range to get the
                // remaining prefixes of current range
                self.k_count[left.value as usize * r % k] += right.k_count[r];
            }
        }
    }
}
