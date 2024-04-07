struct Solution;

impl Solution {
    pub fn swap_to_end(nums: &mut Vec<i32>, i: i32, k: i32) -> i32 {
        let mut j = i + 1;
        let mut z = i;
        while j < k {
            let nums_j = nums[j as usize];
            nums[j as usize] = nums[z as usize];
            nums[z as usize] = nums_j;
            z = j;
            j += 1;
        }
        k - 1
    }

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut last_find_value: Option<(i32, i32)> = None;
        let mut found_twice = false;
        let mut i = nums.len() as i32 - 1;
        let mut k = nums.len() as i32;
        while i >= 0 {
            println!("{:?}", nums);
            let n_i = nums[i as usize];
            if last_find_value.is_none() {
                last_find_value = Some((n_i, i));
                i -= 1;
                continue;
            }

            let prev_value = last_find_value.unwrap();
            if n_i != prev_value.0 {
                last_find_value = Some((n_i, i));
                found_twice = false;
                i -= 1;
                continue;
            }

            if !found_twice {
                found_twice = true;
                i -= 1;
                continue;
            }
            k = Solution::swap_to_end(nums, prev_value.1, k);
            last_find_value = Some((prev_value.0, prev_value.1 - 1));
            i -= 1;

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


