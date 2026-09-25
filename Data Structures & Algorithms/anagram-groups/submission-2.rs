impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // brute force
        // boucle
        // convertir string en asci sort 
        // creer un hasmap avec en val le mot et cle le code ascii sort
        // faire un get avec if let pour insert nouveau tab dans res final si false
        // si true faire insert dans le tab existant  correspondant.
    //     let mut hm : HashMap< Vec<u8>, Vec<String>> = HashMap::new();
    //     for i in strs.iter(){
    //         let mut oct = i.clone().into_bytes();
    //         oct.sort();
    //         hm.entry(oct).or_insert(Vec::new()).push(i.clone());

    //     }
    // let reos: Vec<Vec<String>> = hm.into_values().collect();
    // reos
        let mut hm : HashMap<[u8; 26], Vec<String>> = HashMap::new();
        for s in strs {
            let mut count = [0u8; 26];
            for b in s.bytes() {
                count[(b - b'a') as usize] += 1;
            }
            hm.entry(count).or_default().push(s);
        }
        hm.into_values().collect()

    }
}
