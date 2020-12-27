#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut scores: Vec<i32> = Vec::new();

        ops.iter().for_each(|op| match op.as_str() {
            "+" => {
                let n = scores.len();
                scores.push(scores[n - 1] + scores[n - 2]);
            }
            "C" => {
                scores.pop();
            }
            "D" => {
                let n = scores.len();
                scores.push(scores[n - 1] * 2);
            }
            _ => {
                let digit = op.parse::<i32>().unwrap();
                scores.push(digit);
            }
        });

        scores.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn should_pass_basic_test() {
        let ops = vec![
            String::from("5"),
            String::from("2"),
            String::from("C"),
            String::from("D"),
            String::from("+"),
        ];
        assert_eq!(Solution::cal_points(ops), 30);
    }
}

/*
 * Solution:
 * We can create a vector that holds the scores
 * and then pattern match the rest.
 * if we have something that resembles a digit,
 * we add it to the stack of digits.
 * If we have a +, then we add the previous two scores
 * we are also guaranteed that there exists some score
 * If we have a C, then we remove the value from the top
 * of the list
 * If we have a D, then we double the value at the top of the lsit
 * and add it as a new value
 * We then return the sum of all the scores
 */
