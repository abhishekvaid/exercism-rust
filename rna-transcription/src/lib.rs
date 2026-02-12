#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    seq: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    seq: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        match dna
            .as_bytes()
            .iter()
            .position(|ch| !matches!(ch, b'G' | b'C' | b'T' | b'A'))
        {
            Some(idx) => Err(idx),
            None => Ok(Dna {
                seq: dna.to_owned(),
            }),
        }
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            seq: String::from_utf8(
                self.seq
                    .bytes()
                    .map(|ch| match ch {
                        b'G' => b'C',
                        b'C' => b'G',
                        b'T' => b'A',
                        b'A' => b'U',
                        _ => unreachable!(),
                    })
                    .collect(),
            )
            .unwrap(),
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        match rna
            .as_bytes()
            .iter()
            .position(|ch| !matches!(ch, b'C' | b'G' | b'A' | b'U'))
        {
            Some(idx) => Err(idx),
            None => Ok(Rna {
                seq: rna.to_owned(),
            }),
        }
    }
}
