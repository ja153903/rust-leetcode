#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut points = points.clone();

        points.sort_by(|a, b| (a[0].pow(2) + a[1].pow(2)).cmp(&(b[0].pow(2) + b[1].pow(2))));

        points
            .iter()
            .take(k as usize)
            .map(|v| v.clone())
            .collect::<Vec<Vec<i32>>>()
    }
}
