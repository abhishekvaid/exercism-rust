use std::result;

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    
    let mut lines = diagram.lines();

    let STUDENTS: [&str; 12] = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];

    let student_idx = STUDENTS.iter().position(|s| *s == student).unwrap();

    let start = student_idx * 2;
    let end = start + 2;

    let top_line = lines.next().unwrap(); 
    let bottom_line= lines.next().unwrap(); 

    top_line[start..end]
    .chars()
    .chain(bottom_line[start..end].chars())
    .map(|ch| match ch {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => panic!("Flower not found")
    }).collect()
}
