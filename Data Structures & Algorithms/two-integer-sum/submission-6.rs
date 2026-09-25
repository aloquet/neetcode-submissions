impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
         let mut t : HashMap<i32, usize> = HashMap::new();
        for i in 0..nums.len(){
            let diff = target - &nums[i];
            if let Some(&index_trouve) = t.get(&diff){
                return vec![index_trouve as i32 , i as i32]
            } else {
                t.insert(nums[i] as i32, i);
            }
        
        }
        return vec![]
    }
}
