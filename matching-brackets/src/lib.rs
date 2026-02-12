pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack : Vec<char> = vec![]; 
    for ch in string.chars() {
        match ch {
            '(' | '{' | '['  => stack.push(ch), 
            ')' | '}' | ']' => {
                let opening_bracket = match ch {
                 ')' => '(',
                 '}' => '{',
                 ']' => '[',
                 _ => unreachable!() 
                };
                if stack.pop() != Some(opening_bracket) {
                    return false ;
                }
            }
            _ => {} // ignore non-bracket character 
        }
    }
    stack.is_empty()
}