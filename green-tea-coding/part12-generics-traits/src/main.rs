fn main() {
    let nums = [1, 2, 3, 4, 5];
    println!("{}", sum_of_elemets(&nums));
}

pub fn sum_of_elemets<T>(nums: &[T]) -> T {
    let mut sum = nums[0];
    for i in 1..nums.len() {
        sum += nums[i];
    }
    sum
}
