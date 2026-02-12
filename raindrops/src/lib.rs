pub fn raindrops(n: u32) -> String {

    let mut tokens: String = String::new();

    if n % 3 == 0 { tokens.push_str("Pling")};
    if n % 5 == 0 { tokens.push_str("Plang")};
    if n % 7 == 0 { tokens.push_str("Plong")};

    if tokens.is_empty() {
        return n.to_string();
    }

    tokens

    
}
