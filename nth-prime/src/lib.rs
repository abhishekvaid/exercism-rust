pub fn nth(n: u32) -> u32 {
    // todo!("What is the 0-indexed {n}th prime number?")

    let mut primes: Vec<u32> = vec![2];

    for _ in 0..n {
        let start = primes.last().unwrap() + 1 ; 
        if let Some(next_prime) = (start..)
            .filter(|num| 
                !primes.iter().any(|prime| num % prime == 0)
            )
            .next()
        {
            primes.push(next_prime);
        }
    }

    primes.pop().unwrap()
}
