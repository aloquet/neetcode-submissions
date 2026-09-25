impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut set_var = HashSet::new();
     
            // renvoie true si élement existe pas encore, sinon false
            // ici si insertion fausse alors true
        (0..nums.len()).any(|i| !set_var.insert(nums[i]))
        
        
    }
}
