#![allow(dead_code)]

fn find_complements_for_num(nums: &Vec<usize>, target: usize) -> Option<(usize, usize)> {
    nums.iter().find_map(|&x| {
        nums.iter()
            .find(|&&y| target > x && y == target - x)
            .map(|&y| (y, x))
    })
}

fn product_of_pairs(nums: Vec<usize>) -> Option<usize> {
    nums.iter().find_map(|x| {
        find_complements_for_num(&nums, 2020 - x)
            .map(|(y, z)| x * y * z)
    })

}

#[cfg(test)]
mod tests {
    use crate::input::{read_nums};
    use super::*;

    #[test]
    fn test_product() {
        let nums = read_nums("input/day1.txt");

        assert_eq!(product_of_pairs(nums), Some(8446464));
    }
}
