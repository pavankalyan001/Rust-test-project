// Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number. in rust

fn find_first_occurrence(nums: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut result = None;

    while left <= right {
        let mid = left + (right - left) / 2;

        if nums[mid] == target {
            result = Some(mid);
            right = mid - 1; // continue searching on the left side
        } else if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    result
}

fn main() {
    let nums = vec![1, 2, 3, 4, 4, 5, 6, 7, 8, 9];
    let target = 4;

    if let Some(index) = find_first_occurrence(&nums, target) {
        println!("The first occurrence of {} is at index {}", target, index);
    } else {
        println!("{} not found in the array", target);
    }
}
