pub struct Solution;

impl Solution {
    /*
    Idea: For each index r, we try to maintain the sum:
        S = sum(squared unique counts of all subarrays ending at r)

    We also use UQ[i] as the unique count for subarray nums[i..=r]

    When processing a new element, if that element is new for a subarray with unique count C,
    the squared sum increases by:
        (C+1)^2 - C^2 = 2C + 1

    Suppose we store the previous time we see nums[r] in a hashmap last_seen, then when processing
    a new index r:

        p = last_seen[nums[r]] -> index of last time we see nums[r].

    Then all counts in UQ[p+1..=r] increase by 1, because all of the subarray nums[i..=r] for
    i in [p+1..=r] will have 1 more unique item (they haven't seen A[r] before).
    => We perform a range update for the range above by using a Fenwick tree over UQ.

    After such update, the squared sum S increase by: 2C + 1 for each C being the unique count
    of an affected subarray (from the update step above):
        Total increase = 2 * sum(UQ[p+1..=r]) + (r - p)

    */
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        const MOD: i64 = 1_000_000_007;
        let mut last_seen: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        let mut uq_fenwick = RangeFenwick::new(n);

        let mut res = 0;
        let mut curr_s = 0;

        for (r, &elem) in nums.iter().enumerate() {
            let p = *last_seen.get(&elem).unwrap_or(&-1);

            // Update squared sum
            curr_s = (curr_s
                + 2 * uq_fenwick.range_sum((p + 1) as usize, r)
                + (r as i64 - p as i64))
                % MOD;

            res = (res + curr_s) % MOD;

            // Update unique counts for all subarrays [i..=r] for i = [p+1..=r]
            uq_fenwick.update((p + 1) as usize, r, 1);

            // Update last_seen with this occurence
            last_seen.insert(elem, r as i32);
        }

        res as i32
    }
}

/*
(go read original fenwick tree below first)
-----------------------------------------------------------------------------------------
# VARIANT 1: Fenwick for range update + point query (see normal Fenwick below)

-> Trick to use Fenwick for O(log n) range update (update all elements in a range)
and point query (get value of one element).

Given the original array A[0..n], consider the difference array D[0..n]:

    D[0] = A[0]
    D[i] = A[i] - A[i - 1]

Aka aside from D[0], each D[i] stores the difference between 2 consecutive elements of A.
This has a special property:

    A[i] = D[0] + D[1] + ... + D[i]  => Aka prefix sum D[0..=i]

Thus, we can use a Fenwick tree over the difference array D:

* Range update: Update all A[i] from index l to r by delta (add).

    D[l] = A[l] - A[l-1] => Increase by delta because A[l] increases, A[l-1] stays.
    D[i] for i = l+1 -> r-1 => Stay the same because both A[i] and A[i-1] increases.
    D[r+1] = A[r+1] - A[r] => Decrease by delta because A[r+1] stays, A[r] increases.

    => Two updates to the array D, means two updates for Fenwick of D => Still O(log n).

* Point query: A[i] = sum(i) of Fenwick of D, due to property of D above.

-----------------------------------------------------------------------------------------
# VARIANT 2: Fenwick for range update + range query (relevant for this Leetcode)

Consider prefix sum of A in terms of the difference array D:

sum( A[0..=r] )
= A[0] + A[1] + A[2] + ... + A[r]
= (D[0]) + (D[0] + D[1]) + (D[0] + D[1] + D[2]) + ... + (D[0] + D[1] + ... + D[r])
= (r+1)*D[0] + r*D[1] + (r-1)*D[2] + ... + 1*D[r]
= (r+1)*sum( D[0..=r] ) - (0*D[0] + 1*D[1] + 2*D[2] + ... + r*D[r])
= (r+1)*sum( D[0..=r] ) - sum( k*D[k] for k=[0..=r] )

(IMPORTANT: last 2 lines are subtraction, not addition)

Aka the prefix sum of D (times r+1) and the prefix sum of index-weighted D.
=> We can maintain both with 2 Fenwick.
=> Separate into 2 prefix sums as above so that both does not depends on r internally.

* Fenwick for D is already described in VARIANT 1 above.

* Fenwick for WD (index-weighted D):

-> Update A[l..r] by delta like this:

    WD[l] = l * D[l] => Increase by `delta * l` because D[l] increases by delta.
    WD[i] = i * D[i] for i = l+1 -> r-1 => D[i] stays the same, thus WD[i] too.
    WD[r+1] = (r+1) * D[r+1] => Decrease by `delta * (r+1)`` because D[r+1] decreases.

==> THUS, for the original array A:
- Range update: Update both D and WD as above.
- Prefix query: A[0..=r] = (r+1) * D[0..=r] + WD[0..=r]
- Range query: is just 2 prefix query A[0..=r] - A[0..=l-1]
*/
struct RangeFenwick {
    diffs: Fenwick,
    weighted_diffs: Fenwick,
}

impl RangeFenwick {
    fn new(n: usize) -> Self {
        RangeFenwick {
            diffs: Fenwick::new(n),
            weighted_diffs: Fenwick::new(n),
        }
    }

    /// Update A[l..=r] by delta for the original array A.
    fn update(&mut self, l: usize, r: usize, delta: i64) {
        self.diffs.update(l, delta);
        self.diffs.update(r + 1, -delta);
        self.weighted_diffs.update(l, l as i64 * delta);
        self.weighted_diffs.update(r + 1, -(r as i64 + 1) * delta);
        // Update r+1 out of bound should have no effect
    }

    /// Prefix sum for A[0..=r]
    fn sum(&self, r: usize) -> i64 {
        (r as i64 + 1) * self.diffs.sum(r) - self.weighted_diffs.sum(r)
    }

    /// Range sum for A[l..=r]
    fn range_sum(&self, l: usize, r: usize) -> i64 {
        // println!("D: {:?}", self.diffs.t_arr);
        // println!("WD: {:?}", self.weighted_diffs.t_arr);
        if l == 0 {
            self.sum(r)
        }
        else {
            self.sum(r) - self.sum(l - 1)
        }
    }
}

/*
ORIGINAL FENWICK TREE:

Reference: https://cp-algorithms.com/data_structures/fenwick.html
Summary:
    Fenwick tree is a data structure that supports O(log n) range query and
    point updates, ie. finding the sum of a range [i..j], and updating element i.

    The backing array of Fenwick stores special range sums, see below.

Given an array A[0..n] -> Fenwick tree (aka binary index tree) is an array T[0..n]
such that:

    T[i] = sum of A[start_for(i)] -> A[i]

It supports 2 operation:

* sum(r): Calculate sum of A[0..=r]
  -> while r > 0: result += T[r]; r = start_for(r) - 1
  -> Aka sum all ranges from right to left, starting at r
  -> start_for(i) must be <= i

* update(i, delta): Update element A[i] with delta, updating all range sums.
  -> for all j where i is in [start_for(j), j]: T[j] += delta
  -> Aka update all range sums that are affected by change of A[i].
  -> Aka all ranges that contain i.

Note that sum can be any group operation, aka has identity element and inversible.
Inversible so that sum for range [l..=r] can be calculated as `sum(r) - sum(l - 1)`.

Choosing start_for(i) affects the time complexity of the 2 operations:
- start_for(i) = i => T = A => O(1) update, O(n) sum
- start_for(i) = 0 => T is prefix sum array => O(n) update, O(1) sum

# Fenwick tree is a smart start_for(i) such that both ops are O(log n):

    start_for(i) = i but flip all trailing 1's to 0's

-> For example: start_for(0b1011) = 0b1000
                start_for(0b1000) = 0b1000

-> Easy to see that start_for(i) will be smaller than or equal to i.
-> O(1) way to calculate: start_for(i) = i & (i + 1) (easy to prove)

Proof that sum(r) is O(log n):
- Suppose n has k bits. => k = log n
- n has at most k 1's bits, each iteration take care of at least 1 such bit:
    E.x: start_for = 0b11000 -> next iteration: 0b10111 -> start_for = 0b10000
    Notice how the after each iteration, start_for(i) loses one bit 1.
- Thus, maximum k iterations, each iteration is O(1).

# For update(i, delta), we have another smart way to find all j's whose range contains i:

    Continuously flip the rightmost 0's bit of i -> find_j(i) = i | (i + 1)

For example: find_j(0b1010) = 0b1011 => Range: 0b1000-0b1011
             find_j(0b1011) = 0b1111 => Range: 0b0000-0b1111
             ...
             (do this until find_j > n - 1 aka out of range)

Obviously, the ranges like these will contain i. But other j > i, such as 0b1100,
will not contain i, because of how start_for(j) works => No trailing 1's to flip so
start_for(0b1100) = 0b1100 (itself).

=> Only numbers larger than i with trailing 1's have ranges containing i.

It's also easy to see that this is O(log n) for update:
- Each iteration flip one 0's bit of i to 1, and i has at most log n 0's bit.
- Thus, maximum O(log n) iterations, each takes O(1) to update.
*/
struct Fenwick {
    t_arr: Vec<i64>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Fenwick { t_arr: vec![0; n] }
    }
    /// Definition of `g(i)` for Fenwick
    fn start_for(i: isize) -> isize {
        i & (i + 1)
    }

    /// Function to find next `j` whose range contains `i`
    fn find_j(i: usize) -> usize {
        i | (i + 1)
    }

    /// Return the prefix sum for the original array `A[0..=r]`
    fn sum(&self, r: usize) -> i64 {
        // Cap at last index
        let mut r: isize = r.min(self.t_arr.len() - 1) as isize;

        let mut res = 0;
        while r >= 0 {
            res += self.t_arr[r as usize];
            r = Fenwick::start_for(r) - 1;
        }

        res
    }

    /// Update 1 element in the original array `A` (by updating the range sums).
    /// No effect if i > A.len().
    fn update(&mut self, i: usize, delta: i64) {
        // Find all j whose range contains i
        let mut j = i;

        while j < self.t_arr.len() {
            self.t_arr[j] += delta;
            j = Fenwick::find_j(j);
        }
    }
}

mod test {
    use crate::p2916_fenwick_tree::*;

    #[test]
    fn test_range_fenwick() {
        let a = vec![0, 1, 2, 3, 4, 5];

        let mut fen = RangeFenwick::new(6);
        fen.update(1, 1, 1);
        fen.update(2, 2, 2);
        fen.update(3, 3, 3);
        fen.update(4, 4, 4);
        fen.update(5, 5, 5);

        assert_eq!(a[1..=3].iter().sum::<i64>(), fen.range_sum(1, 3));
    }
}