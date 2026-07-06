use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = input.trim().as_bytes();

    let n = s.len();
    let mut pi = vec![0usize; n];

    for i in 1..n {
        let mut j = pi[i - 1];

        while j > 0 && s[i] != s[j] {
            j = pi[j - 1];
        }

        if s[i] == s[j] {
            j += 1;
        }

        pi[i] = j;
    }

    let mut borders = Vec::new();

    let mut k = pi[n - 1];

    while k > 0 {
        borders.push(k);
        k = pi[k - 1];
    }

    borders.reverse();

    for b in borders {
        print!("{} ", b);
    }

    println!();
}