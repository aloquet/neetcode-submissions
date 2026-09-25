impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // faire une hash map avec en cle les différent chiffre et le nombre de répetition en val
        // a terme si val => k alors je return la clé.
        
        let mut nums_map : HashMap<usize, i32> = HashMap::new();
        let n = nums.len();
        let mut buckets : Vec<Vec<i32>> = vec![Vec::new(); n + 1]; 
        for num in nums {
            let count = nums_map.entry(num as usize).or_default();
            *count +=1;
            
        }
        for (val, freq) in nums_map.iter() {
            buckets[*freq as usize].push(*val as i32);
        }
        let res: Vec<i32> = buckets
            .into_iter()
            .rev()
            .flatten()
            .take(k as usize)
            .collect();

        res
    }
}
