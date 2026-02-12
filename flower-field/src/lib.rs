pub fn annotate(garden: &[&str]) -> Vec<String> {
    let row = garden.len();
    if row == 0 {
        return Vec::new();
    }
    let col = garden[0].len();
    let mut result = vec![vec![b' '; col]; row];

    for i in 0..row {
        for j in 0..col {
            if garden[i].as_bytes()[j] == b'*' {
                for di in -1i8..=1 {
                    for dj in -1i8..=1 {
                        if !(di == 0 && dj == 0) {
                            let ni = (i as i32) + (di as i32);
                            let nj = (j as i32) + (dj as i32);
                            if ni >= 0 && ni < (row as i32) && nj >= 0 && nj < (col as i32) {
                                let ni_u = ni as usize;
                                let nj_u = nj as usize;
                                if garden[ni_u].as_bytes()[nj_u] != b'*' {
                                    if result[ni_u][nj_u] == b' ' {
                                        result[ni_u][nj_u] = b'1';
                                    } else {
                                        result[ni_u][nj_u] += 1;
                                    }
                                }
                            }
                        }
                    }
                }
                result[i as usize][j as usize] = b'*';
            }
        }
    }

    result
        .into_iter()
        .map(|vec| String::from_utf8(vec).unwrap())
        .collect()
}
