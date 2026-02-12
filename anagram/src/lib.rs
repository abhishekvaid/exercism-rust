use std::collections::{HashMap, HashSet};


fn char_freq(s: &str) -> HashMap<char, usize> {
    let mut result = HashMap::new(); 
    for ch in s.chars() {
        *result.entry(ch).or_insert(0) += 1
    }
    result 

}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[& 'a str]) -> HashSet<&'a str> {

    let word_lower = word.to_lowercase();
    let word_freq = char_freq(word_lower.as_str()); 

    let  result: HashSet<&str> = possible_anagrams
        .iter()
        .filter(|candidate| {
            let candidate_lower = candidate.to_lowercase();
            candidate_lower != word_lower && char_freq(&candidate_lower) == word_freq 
        })
        .copied()
        .collect();  
    result 
    
}