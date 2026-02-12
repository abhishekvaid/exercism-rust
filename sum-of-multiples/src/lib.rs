use std::collections::HashSet; 

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {

    if limit == 0 {
        return 0; 
    }
    
    factors
    .iter()
    .filter(|&n| *n != 0)
    .flat_map( |&n|
        (n..).step_by(n as usize).take_while( |&m| m < limit) 
    )
    .collect::<HashSet<u32>>()
    .iter()
    .sum()
    
}
