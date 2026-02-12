/// Check a Luhn checksum.

pub fn is_valid(code: &str) -> bool {
    let x = code
        .chars()
        .rev()
        .filter(|ch| !ch.is_ascii_whitespace())
        .try_fold((0, 0), |(count, sum), elem| {
            elem.to_digit(10)
                .map(|d| if count % 2 == 1 { d * 2 } else { d })
                .map(|d| if d > 9 { d - 9 } else { d })
                .map(|d| (count + 1, sum + d))
        });


    matches!(x, Some((count, sum)) if count > 1 && sum % 10 == 0 )
    
    }    
