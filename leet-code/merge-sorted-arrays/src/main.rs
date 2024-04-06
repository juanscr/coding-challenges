struct Solution;

impl Solution {
    pub fn merge<'a>(nums1: &'a mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) -> &'a mut Vec<i32> {
        let mut i = m - 1;
        let mut j = n - 1;
        let mut k = m + n - 1;

        while i >= 0 && j >= 0 {
            if nums1[i as usize] > nums2[j as usize] {
                nums1[k as usize] = nums1[i as usize];
                k -= 1;
                i -= 1;
                continue;
            }
            nums1[k as usize] = nums2[j as usize];
            k -= 1;
            j -= 1;
        }
        while j >= 0 {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
            k -= 1;
        }
        nums1
    }
}

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let m = 3;
    let mut nums2 = vec![2, 5, 6];
    let n = 3;
    // Should be
    println!("Real solution: {:?}", vec![1, 2, 2, 3, 5, 6]);
    println!("{:?}", Solution::merge(&mut nums1, m, &mut nums2, n));
    println!("======================");

    let mut nums1 = vec![0];
    let m = 0;
    let mut nums2 = vec![1];
    let n = 1;
    //// Should be [1]
    println!("Real solution: {:?}", vec![1]);
    println!("{:?}", Solution::merge(&mut nums1, m, &mut nums2, n));
    println!("======================");

    let mut nums1 = vec![2, 0];
    let m = 1;
    let mut nums2 = vec![1];
    let n = 1;
    println!("Real solution: {:?}", vec![1, 2]);
    println!("{:?}", Solution::merge(&mut nums1, m, &mut nums2, n));
    println!("======================");

    let mut nums1 = vec![0,1,2,3,0,0,0,0,0];
    let m = 4;
    let mut nums2 = vec![-1,1,2,3,7];
    let n = 5;
    println!("Real solution: {:?}", vec![-1,0,1,1,2,2,3,3,7]);
    println!("{:?}", Solution::merge(&mut nums1, m, &mut nums2, n));
    println!("======================");

    let mut nums1 = vec![-10,-9,-8,-8,0,1,2,3,0,0,0,0,0,0,0,0];
    let m = 8;
    let mut nums2 = vec![-10,-9,-9,-8,-8,0,1,1];
    let n = 8;
    println!("Real solution: {:?}", vec![-10, -10, -9, -9, -9, -8, -8, -8, -8, 0, 0, 1, 1, 1, 2, 3]);
    println!("{:?}", Solution::merge(&mut nums1, m, &mut nums2, n));
    println!("======================");

}


