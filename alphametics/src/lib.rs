use std::collections::{HashMap, HashSet};

pub fn make_number(token: &str, map: &HashMap<u8, u8>) -> u32 {
    debug_assert!(token.is_ascii(), "expected ASCII input");
    token
        .as_bytes()
        .iter()
        .map(|ch| map.get(ch).copied().unwrap())
        .fold(0_u32, |acc, digit| (acc * 10) + (digit as u32))
}

pub fn parse_puzzle(puzzle: &str) -> Result<(Vec<&str>, &str, HashSet<u8>, HashSet<u8>), String> {
    let tokens: Vec<&str> = puzzle.split(" == ").collect();

    debug_assert!(
        tokens.len() == 2,
        r#"puzzle format should be exactly "... == ..." "#
    );

    let rhs = tokens[1];
    let lhs = tokens[0];

    let lhs_tokens: Vec<&str> = lhs.split(" + ").collect();

    debug_assert!(lhs_tokens.len() > 1);

    let unique_bytes: HashSet<u8> = lhs_tokens
        .iter()
        .chain(std::iter::once(&rhs))
        .flat_map(|&token| token.as_bytes())
        .copied()
        .collect();

    let leading_bytes = lhs_tokens
        .iter()
        .chain(std::iter::once(&rhs))
        .map(|&token| token.as_bytes()[0])
        .collect();

    Ok((lhs_tokens, rhs, unique_bytes, leading_bytes))
}

pub fn is_valid_leading(leading_bytes: &HashSet<u8>, bytes_digit_map: &HashMap<u8, u8>) -> bool {
    leading_bytes.iter().all(|byte| bytes_digit_map[byte] != 0)
}

pub fn build_mapping(bytes: &[u8], digits: &[u8]) -> HashMap<u8, u8> {
    bytes.iter().copied().zip(digits.iter().copied()).collect()
}

pub fn check_solution(lhs_tokens: &[&str], rhs: &str, bytes_digits_map: &HashMap<u8, u8>) -> bool {
    let lhs_eval: u32 = lhs_tokens
        .iter()
        .map(|&s| make_number(s, bytes_digits_map))
        .sum();

    let rhs_eval: u32 = make_number(rhs, bytes_digits_map);

    lhs_eval == rhs_eval
}

struct Permutations {
    items: Vec<u32>,
    slots: Vec<usize>,
    k: usize,
    used: Vec<bool>,
    depth: usize,
    highest_seen: Vec<usize>,
}

impl Permutations {
    fn new(items: Vec<u32>, k: usize) -> Self {
        let n = items.len();
        let slots = vec![];
        let highest_seen = vec![0; k];
        let used = vec![false; n];
        Permutations {
            items,
            slots,
            k,
            used,
            depth: 0,
            highest_seen,
        }
    }

    fn next(&mut self) -> Vec<u32> {
        
        if self.slots.len() == self.k {
            let pop_idx = self.slots.pop().unwrap();
            self.highest_seen[self.k - 1] = pop_idx;
            self.used[pop_idx] = false;
        }

        while self.depth <= self.k {
            let highest_seen = self.highest_seen[self.depth];
            if let Some(candidate_idx) = (highest_seen..n).find(|&idx| !self.used[idx]) {
                self.used[candidate_idx] = true;
                self.slots[self.depth] = self.items[candidate_idx];
            }
        }

        let result = self
            .slots
            .iter()
            .map(|&idx| self.items[idx])
            .collect();

        result
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    match parse_puzzle(input) {
        Ok((lhs_tokens, rhs, unique_bytes, leading_bytes)) => {
            // let mut permutation = vec![0_u8; unique_bytes.len()];
            // let mut bytes_digits_map: HashMap<u8, u8> = unique_bytes.iter().zip(permutation.iter()).collect();

            None
        }

        Err(msg) => None,
    }
}
