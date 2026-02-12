#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list.len() == second_list.len() {
        if first_list == second_list {
           return Comparison::Equal; 
        }     
    } else if first_list.len() < second_list.len() {
        let first_list_len = first_list.len(); 
        if first_list_len == 0 {
            return Comparison::Sublist; 
        }
        //  2 5 -> [a a a a a]
        for j in 0..=(second_list.len() - first_list_len) {
            if first_list[0] == second_list[j] && first_list[0..] == second_list[j..(j+first_list_len)]{
                return Comparison::Sublist; 
            }    
        }
    } else {
        if sublist(second_list, first_list) == Comparison::Sublist {
            return Comparison::Superlist; 
        }
       }
    Comparison::Unequal
}
