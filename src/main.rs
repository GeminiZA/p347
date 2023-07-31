
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            map.entry(num).and_modify(|value| *value += 1).or_insert(1);
        }
        let mut v: Vec<_> = map.iter().collect();
        v.sort_by(|a, b| b.1.cmp(a.1));
        let res: Vec<i32> = v.iter().take(k as usize).map(|(k, _)| **k).collect();
        return res;
    }

}
fn main() {
    let nums = vec![1,1,1,2,2,2,2,3];
    let k: i32 = 2;
    let result: Vec<i32> = Solution::top_k_frequent(nums, k);
    println!("{:?}", result);
}

