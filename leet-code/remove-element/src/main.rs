struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        let mut j = nums.len();
        while i < j {
            if nums[i] == val {
                j -= 1;
                let temp = nums[i];
                nums[i] = nums[j];
                nums[j] = temp;
                continue;
            }
            i += 1;
        }
        i as i32
    }
}

fn main() {
    let mut nums1 = vec![3,2,2,3];
    let val = 3;
    println!("{} {:?}", Solution::remove_element(&mut nums1, val), nums1);
    println!("======================");
}


