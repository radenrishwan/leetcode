/// leetcode: 58. Length of Last Word [leetcode.com/problems/length-of-last-word/]
pub fn length_of_last_word(s: String) -> i32 {
    let word: Vec<&str> = s.trim().split(" ").collect();

     word.last().unwrap().len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "Hello World".to_string();

        assert_eq!(length_of_last_word(s), 5);
    }
}
