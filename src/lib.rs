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
