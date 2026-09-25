impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut set_var = HashSet::new();
        for i in 0..nums.len(){
            // renvoie true si élement existe pas encore, sinon false
            // ici si insertion fausse alors true
            if !set_var.insert(nums[i]) {
                return true
            };
        }
        false
    }
}
