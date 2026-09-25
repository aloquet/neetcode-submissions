impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // prermiere idée en brute force 
        // double boucle for une pour i et lautre pour faire la somme avec le meme tab comme j
        // et verif si egal à target.
        // bien penser à continue si indice égaux
        // for i in 0..nums.len(){
        //     for j in 0..nums.len(){
        //         if i == j { continue };
        //         if nums[i] +nums[j] == target {
        //             return vec![i as i32,j as i32]
        //         };
        //     };
        // };
        // return vec![0, 0] 
        //time O(n²) space:0(1)

        // for i in 0..nums.len(){
        //     let diff = target - nums[i];
        //     let ind : Vec<usize> = nums.iter().enumerate().filter_map(|(idx, &x)| if i!=idx && x == diff { Some(idx)} else {None} ).collect();
        //     println!("{:?}",ind);
        //     if ind.is_empty() {continue}
        //     return vec![i as i32, ind[0] as i32]
        // }
        // return vec![0,0]
        //time O(n²) space:0(1) echec

        // let mut t = HashMap :: new();
        // let v = nums
        //     .iter()
        //     .enumerate()
        //     .map( |(idx, &num)| if t.contains_key(&(target - &num)){ 
        //                             vec![*(t.get(&(target - &num))).unwrap(),idx as i32];
        //                         }else {
        //                             t.insert(num, idx as i32);
        //                         }
        //     );
        // println!("{:?}", v);
        // return vec![0,0]

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
