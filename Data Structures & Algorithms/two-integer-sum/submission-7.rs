impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut t : HashMap<i32, usize> = HashMap::new();
        for (idx,&num) in nums.iter().enumerate(){
            let diff = target - num;
            if let Some(&index_trouve) = t.get(&diff){
                return vec![index_trouve as i32 , idx as i32]
            } else {
                t.insert(num , idx);
            }
        
        }
        return vec![]
    }
}
