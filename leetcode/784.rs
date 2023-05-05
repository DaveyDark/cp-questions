/*
Question:
Given a string s, you can transform every letter individually to be lowercase or uppercase to create another string.
Return a list of all possible strings we could create. Return the output in any order.
*/

// Approach:
// The backtrack function works as follows
// 1) The base case is the idx(size of permutation) exceeding the size of s
//    If it does then we push the current permutation to ans
// 2) Otherwise we get the current char of s and check if it's an alphabet
// 3) If it's an alphabet, we backtrack with both the lowercase and uppercase version of that char
//    Otherwise we only do the backtracking once with the character.

impl Solution {
    pub fn backtrack(s: &mut String, st: &mut String, idx: i32, ans: &mut Vec<String>) {
        if idx >= s.len() as i32 {
            ans.push(st.clone());
            return;
        }
        let ch = s.chars().nth(idx as usize).unwrap_or('_');
        if ch.is_ascii_alphabetic() {
            st.push(ch.to_ascii_lowercase());
            Self::backtrack(s,st,idx+1,ans);   
            st.pop();
            st.push(ch.to_ascii_uppercase());
            Self::backtrack(s,st,idx+1,ans);   
            st.pop();
        } else {
            st.push(ch);
            Self::backtrack(s,st,idx+1,ans);
            st.pop();
        }
    }
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut permuts = Vec::new();
        let mut s = s.clone();
        let mut st = String::new();
        Self::backtrack(&mut s,&mut st,0, &mut permuts); 
        permuts       
    }
}
