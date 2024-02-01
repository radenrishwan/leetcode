/// leetcode: 35. Search Insert Position [https://leetcode.com/problems/search-insert-position/]
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut i = 0;
    for &num in nums.iter() {
        if num < target {
            i += 1;
        }
    }

    i as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;

        assert_eq!(search_insert(nums, target), 2);
    }
}
