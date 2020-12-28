#![allow(dead_code)]

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams = HashMap::new();

        for s in strs {
            let mut k: Vec<u8> = s.bytes().collect();
            k.sort_unstable();
            anagrams.entry(k).or_insert_with(|| vec![]).push(s);
        }

        anagrams.into_iter().map(|(_, v)| v).collect()
    }
}

// for each character in the vector, we have to sort it to get a unique key
// then we add the value we're iterating over into a hashmap where the key
// is the sorted string and the value is a vector of strings
// then once we're done iterating over the vector, we have to return the values
// of the map as a vec
