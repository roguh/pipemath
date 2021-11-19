// TODO Vec should be any iterator
// TODO: f64 should just a value that supports Sum and to string
pub fn sum(nums: &Vec<f64>) -> f64 {
    let mut result: f64 = 0.0; // should be first
    for num in nums {
        result = result + num;
    }
    result
}

pub fn mean(nums: &Vec<f64>) -> f64 {
    let nums_sum = sum(&nums);
    let len = nums.len() as f64;
    nums_sum / len
}
