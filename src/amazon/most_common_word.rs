#![allow(dead_code)]

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let mut words = Vec::new();
        for word in paragraph.split(&['!', '?', '\'', ';', '.', ' ', ','][..]) {
            if !word.is_empty() {
                words.push(word.to_lowercase().to_string());
            }
        }

        let mut counter: HashMap<&String, i32> = HashMap::new();

        words.iter().for_each(|word| {
            if let Some(value) = counter.get_mut(&word) {
                *value += 1;
            } else {
                counter.insert(word, 1);
            }
        });

        let mut counter_vec: Vec<_> = counter.iter().collect();

        counter_vec.sort_by(|a, b| b.1.cmp(&a.1));

        for (word, _) in counter_vec.iter() {
            if !banned.contains(&word) {
                return word.to_string();
            }
        }

        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn should_pass_basic() {
        let s = String::from("Bob hit a ball, the hit BALL flew far after it was hit.");
        let banned = vec![String::from("hit")];

        assert_eq!(Solution::most_common_word(s, banned), String::from("ball"));
    }
}
