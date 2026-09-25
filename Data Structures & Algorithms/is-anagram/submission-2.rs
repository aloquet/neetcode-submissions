impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.bytes().len() != t.bytes().len() { return false };
        let mut  b = s.into_bytes();
        b.sort();
        let mut a = t.into_bytes();
        a.sort();
        a==b
    
    }
}
