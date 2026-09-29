impl Solution {

    pub fn encode(strs: Vec<String>) -> String {
        let mut encoded : String = String::new();
        for  s in strs {
            let len = s.len();
            encoded.push_str(&len.to_string());
            encoded.push('#');
            encoded.push_str(&s);
        } 
        return encoded
    }

   pub fn decode(s: String) -> Vec<String> {
        let mut decoded : Vec<String> = Vec::new();
        let mut i = 0;
        while i < s.len(){
            let idx = s[i..].find("#").unwrap() + i ;
            let length : usize = s[i..idx].parse::<usize>().unwrap();
            let start = idx + 1;
            let end = start + length;
            decoded.push(s[start..end].to_string());
            i = end;
        }
        decoded
    }
}
