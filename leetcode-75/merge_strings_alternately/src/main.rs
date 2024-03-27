impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::new();
        let max_len = word1.len().max(word2.len());

        for i in 0..max_len {
            if i < word1.len() {
                result.push(word1.chars().nth(i).unwrap());
            }
            if i < word2.len() {
                result.push(word2.chars().nth(i).unwrap());
            }
        }
        result
    }
}

fn main() {}

