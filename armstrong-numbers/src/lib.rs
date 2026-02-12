pub fn is_armstrong_number(num: u32) -> bool {

    let num_str = num.to_string(); 
    let num_digits = num_str.len(); 

    let sum_of_digits: u32 = num_str
    .bytes()
    .map(|ch| ((ch - b'0') as u32).pow(num_digits as u32))
    .sum();

    sum_of_digits == num 
}
