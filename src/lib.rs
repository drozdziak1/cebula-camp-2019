/// Concat `owned` with `referenced`; demonstrate data ownership
pub fn my_concat(owned: String, referenced: &str) -> String {
    println!("`owned` before move: {}", owned);
    let mut owned_but_mutable = owned;

    // A no-no, `owned` has moved to `owned_but_mutable`
    //println!("Owned after move: {}", owned);

    owned_but_mutable.push_str(referenced);

    owned_but_mutable
}

/// Hexdumps the supplied slice and returns the stringified result; good for hashes
pub fn bytes2string(bytes: &[u8]) -> String {
    let mut out = String::new();
    for byte in bytes {
        out.push_str(&format!("{:x}", byte));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes2string_test() {
        let cases = vec![(vec![], ""), (vec![97], "61"), (vec![0, 0], "0000")];

        for (input, expected) in cases {
            assert_eq!(bytes2string(input.as_slice()).as_str(), expected);
        }
    }
}
