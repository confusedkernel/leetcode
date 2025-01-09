fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn gcd_of_strings(str1: String, str2: String) -> String {
    if str1.clone() + &str2 != str2.clone() + &str1 {
        return String::new();
    }

    let gcd_length = gcd(str1.len(), str2.len());
    str1[..gcd_length].to_string()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_of_strings() {
        assert_eq!(
            gcd_of_strings("ABCABC".to_string(), "ABC".to_string()),
            "ABC".to_string()
        );
        assert_eq!(
            gcd_of_strings("ABABAB".to_string(), "ABAB".to_string()),
            "AB".to_string()
        );
        assert_eq!(
            gcd_of_strings("LEET".to_string(), "CODE".to_string()),
            "".to_string()
        );
    }
}
