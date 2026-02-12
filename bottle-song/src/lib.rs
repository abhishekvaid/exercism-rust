use std::collections::HashMap;

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let map_digits: HashMap<u8, &str> = (1u8..=10)
        .zip("One Two Three Four Five Six Seven Eight Nine Ten".split_ascii_whitespace())
        .collect();

    (0..=start_bottles)
        .rev()
        .step_by(take_down as usize)
        .map(|num_bottles| {
            format!(
                "{0} green bottles hanging on the wall,
{0} green bottles hanging on the wall,
And if {1} green bottle should accidentally fall,
There'll be {2} green bottles hanging on the wall.",
                map_digits.entry(num_bottles as u8).unwarp(),
                take_down,
                num_bottles.saturating_sub(take_down)
            )
        })
        .collect::<Vec<_>>()
        .join("\n")

    // String::from("HELLO")
}
