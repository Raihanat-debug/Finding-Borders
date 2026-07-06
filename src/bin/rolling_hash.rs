use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;
const BASE: u64 = 911382323;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let s = input.trim().as_bytes();

    let n = s.len();

    let mut power = vec![1u64; n + 1];
    let mut prefix = vec![0u64; n + 1];

    for i in 0..n {
        power[i + 1] = (power[i] * BASE) % MOD;
        prefix[i + 1] = (prefix[i] * BASE + s[i] as u64) % MOD;
    }

    let mut ans = Vec::new();

    for len in 1..n {
        let left = prefix[len];

        let right =
            (prefix[n] + MOD
                - (prefix[n - len] * power[len]) % MOD)
                % MOD;

        if left == right {
            ans.push(len);
        }
    }

    for x in ans {
        print!("{} ", x);
    }

    println!();
}
