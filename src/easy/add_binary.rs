pub fn add_binary(a: String, b: String) -> String {
    let mut result = String::new();
    let mut carry = 0;
    let mut a = a.chars().rev().peekable();
    let mut b = b.chars().rev().peekable();

    while a.peek().is_some() || b.peek().is_some() {
        let mut sum = carry;
        if let Some('1') = a.next() {
            sum += 1;
        }
        if let Some('1') = b.next() {
            sum += 1;
        }

        carry = sum / 2;
        result.push_str(&(sum % 2).to_string());
    }

    if carry > 0 {
        result.push('1');
    }

    result.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let a = "11".to_string();
        let b = "1".to_string();

        assert_eq!(add_binary(a, b), "100".to_string());
    }
    
    #[test]
    fn test_2() {
        let a = "1010".to_string();
        let b = "1011".to_string();

        assert_eq!(add_binary(a, b), "10101".to_string());
    }
}