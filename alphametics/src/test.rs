fn single_value() -> Box<dyn Iterator<Item = u32>> {
    // TODO: return an iterator that yields just the number 42
}

fn main() {
    let mut iter = single_value();
    println!("{:?}", iter.next()); // Should print: Some(42)
    println!("{:?}", iter.next()); // Should print: None
}