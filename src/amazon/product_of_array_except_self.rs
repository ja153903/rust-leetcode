#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![1; nums.len()];

        for i in 1..nums.len() {
            result[i] = result[i - 1] * nums[i - 1];
        }

        let mut right = 1;
        let mut i: isize = (nums.len() - 1) as isize;

        while i >= 0 {
            result[i as usize] *= right;
            right *= nums[i as usize];
            i -= 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn should_pass_test() {
        let v: Vec<i32> = vec![1, 2, 3, 4];
        let expected: Vec<i32> = vec![24, 12, 8, 6];

        assert_eq!(Solution::product_except_self(v), expected);
    }
}

// Create an output array of the same size as the input
// Given that we're constructing products, we can initialize
// the values of the array to be 1.
// We can do an inital pass from index 0 to n-1 where n
// is the size of the array.
// In that initial pass, we update each index of the result array which
// we can call result with the following: result[i] = result[i-1] * nums[i-1]
// So for example, given our input array [1, 2, 3, 4], when we do the initial pass
// we would then get the following values in the result array: [1, 1, 2, 6]
// Now to get the rest of the values, we have to iterate backwards from indices n-1 to 0
// But also keep track of the growing product using a variable we'll call "right"
// So for each backward step, we will multiply result[i] by right and update right by multiplying it
// by nums[i]. Notice that we update after multiplying result[i] because we do not allow
// multiplication by self. So at the end of this pass, we will get the following array:
// [24, 12, 8, 6]
// To further explain it, we can do the following trace of the values
// Before iteration, let i = nums.len() - 1; let right = 1
// i == 3; result[i] = 6; right = 4;
// i == 2; result[i] = 8; right = 12;
// i == 1; result[i] = 12; right = 24;
// i == 0; result[i] = 24; right = 24;
