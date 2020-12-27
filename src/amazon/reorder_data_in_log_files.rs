#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        let mut digit_logs = logs
            .iter()
            .filter(|log| Solution::is_digit(log))
            .collect::<Vec<&String>>();

        let mut letter_logs = logs
            .iter()
            .filter(|log| !Solution::is_digit(log))
            .collect::<Vec<&String>>();

        letter_logs.sort_by(|a, b| {
            let a_index = a.find(" ").unwrap();
            let b_index = b.find(" ").unwrap();

            if a[a_index + 1..].eq(&b[b_index + 1..]) {
                a[..a_index].cmp(&b[..b_index])
            } else {
                a[a_index + 1..].cmp(&b[b_index + 1..])
            }
        });

        letter_logs.append(&mut digit_logs);

        letter_logs.iter().map(|log| log.to_string()).collect()
    }

    fn is_digit(log: &String) -> bool {
        log.split_whitespace()
            .nth(1)
            .unwrap()
            .chars()
            .nth(0)
            .unwrap()
            .is_digit(10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn should_pass_digit_test() {
        let s = String::from("jaime 1 2 3");

        assert!(Solution::is_digit(&s));
    }
}
