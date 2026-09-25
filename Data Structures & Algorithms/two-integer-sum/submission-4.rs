impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        //time O(n²) space:0(1) echec
        for i in 0..nums.len(){
            let diff = target - nums[i];
            let ind : Vec<usize> = nums.iter().enumerate().filter_map(|(idx, &x)| if i!=idx && x == diff { Some(idx)} else {None} ).collect();
            println!("{:?}",ind);
            if ind.is_empty() {continue}
            return vec![i as i32, ind[0] as i32]
        }
        return vec![0,0]

    }
}
