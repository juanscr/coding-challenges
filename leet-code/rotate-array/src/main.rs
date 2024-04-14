struct Solution;

impl Solution {
    pub fn reverse(nums: &mut Vec<i32>, to: usize, from: usize) {
            let mut tail = from - 1;
            for i in to..from {
                if 2 * i >= from + to {
                    break;
                }
                let temp = nums[i];
                nums[i] = nums[tail];
                nums[tail] = temp;
                tail -= 1;
            }
    }

    pub fn rotate(nums: &mut Vec<i32>, k: i32) -> &mut Vec<i32>{
        let k = k as usize % nums.len();
        if k == 0 {
            return nums;
        }
        Solution::reverse(nums, 0, nums.len());
        Solution::reverse(nums, 0, k);
        Solution::reverse(nums, k, nums.len());
        nums
    }
}

fn main() {
    let mut nums1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let k = 3;
    println!("Real solution: {:?}", vec![7, 8, 9, 1, 2, 3, 4, 5, ]);
    println!("{:?}", Solution::rotate(&mut nums1, k));
    println!("======================");

    let mut nums1 = vec![1, 2, 3, 4, 5, 6, 7];
    let k = 3;
    println!("Real solution: {:?}", vec![5,6,7,1,2,3,4]);
    println!("{:?}", Solution::rotate(&mut nums1, k));
    println!("======================");

    let mut nums1 = vec![-1, -100, 3, 99];
    let k = 2;
    println!("Real solution: {:?}", vec![3, 99, -1, -100]);
    println!("{:?}", Solution::rotate(&mut nums1, k));
    println!("======================");

    let mut nums1 = vec![1, 2, 3];
    let k = 2;
    println!("Real solution: {:?}", vec![2, 3, 1]);
    println!("{:?}", Solution::rotate(&mut nums1, k));
    println!("======================");

    let mut nums1 = vec![1];
    let k = 1;
    println!("Real solution: {:?}", vec![1]);
    println!("{:?}", Solution::rotate(&mut nums1, k));
    println!("======================");

    let mut nums1 = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27];
    let k = 38;
    println!("Real solution: {:?}", vec![17,18,19,20,21,22,23,24,25,26,27,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]);
    println!("{:?}", Solution::rotate(&mut nums1, k));
    println!("======================");
}


