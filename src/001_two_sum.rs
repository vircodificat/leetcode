impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
       for (i, x) in nums.iter().copied().enumerate() {
           for (j, y) in nums.iter().copied().enumerate() {
               if x + y == target && i != j {
                   return vec![i, j];
               }
           }
       }
       panic!("No such solution")
    }
}
