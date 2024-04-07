struct Solution;

impl Solution {

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut k = 1;
        for i in 1..nums.len() {
            if k == 1 || nums[i] != nums[k - 2] {
                nums[k] = nums[i];
                k += 1;
            }
        }
        k as i32
    }
}

fn main() {
    let mut nums1 = vec![1,1,1,2,2,3];
    println!();
    println!("Solution");
    println!("{} {:?}", Solution::remove_duplicates(&mut nums1), nums1);
    println!("Answer");
    println!("5 [1, 1, 2, 2, 3, _]");
    println!("======================");
}


