/// leetcode: 27. Remove Element [https://leetcode.com/problems/remove-element/]
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut result = Vec::new();

    for &num in nums.iter() {
        if num != val {
            result.push(num);
        }
    }

    *nums = result;
    nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;

        assert_eq!(remove_element(&mut nums, val), 2);
        assert_eq!(nums, vec![2, 2]);
    }
}
