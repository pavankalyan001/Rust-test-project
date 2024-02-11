// Given a sorted array of integers, implement a function that returns the median of the array.
in rust

fn find_median(nums: &[i32]) -> f64 {
    let len = nums.len();
    if len % 2 == 0 {
        
        let mid_right = len / 2;
        let mid_left = mid_right - 1;
        (nums[mid_left] + nums[mid_right]) as f64 / 2.0
    } else {
        
    }
}

fn main() {
    let nums_even = vec![1, 2, 3, 4, 5, 6];
    let nums_odd = vec![1, 2, 3, 4, 5];

    println!("Median of {:?}: {}", nums_even, find_median(&nums_even));
    println!("Median of {:?}: {}", nums_odd, find_median(&nums_odd));
}
