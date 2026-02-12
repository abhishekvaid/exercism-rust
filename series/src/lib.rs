pub fn series(digits: &str, len: usize) -> Vec<String> {

    digits
        .as_bytes() // &[u8]
        .windows(len) // Iterator <&[u8]>
        .map(|slice| String::from_utf8(slice.to_vec()).unwrap())
        .collect() 
}
