const RADIX: u32 = 10;

fn encrypt(text: &str, shift: u32) -> String {
    use std::char::from_u32;

    text.chars().fold(String::new(), |mut acc, c| {
        let x = c as u32;

        if x == 32 {
            acc.push(' ');
            acc
        } else {
            // Uppercase
            if (65..=90).contains(&x) {
                let ch = (x - 65 + shift) % 26 + 65;
                acc.push(from_u32(ch).unwrap());
                // Lowercase
            } else if (97..=122).contains(&x) {
                let ch = (x - 97 + shift) % 26 + 97;
                acc.push(from_u32(ch).unwrap());
            }
            acc
        }
    })
}

fn decrypt(text: &str, shift: u32) -> String {
    println!("decrypt {text}");
    encrypt(text, (26 - shift) % 26)
}

fn main() {
    let text = "Hello World!";
    let caesar = encrypt(text, RADIX);
    let reversed = decrypt(&caesar, RADIX);
    println!("Caesar {caesar} - decrypted {reversed}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let encrypted = encrypt("hello", RADIX);

        assert_eq!(encrypted, "rovvy");
        assert_eq!(encrypt("Hello World!", RADIX), "Rovvy Gybvn");
    }

    #[test]
    fn test_decrypt() {
        assert_eq!(decrypt("rovvy", RADIX), "hello");
        assert_eq!(decrypt("Rovvy Gybvn", RADIX), "Hello World");
    }

    #[test]
    fn test_encrypt_empty_str() {
        assert_eq!(encrypt("", RADIX), "");
    }

    #[test]
    fn test_encrypt_non_a_z() {
        assert_eq!(encrypt("---", RADIX), "");
    }

    #[test]
    fn test_decrypt_empty_str() {
        assert_eq!(decrypt("", RADIX), "");
    }

    #[test]
    fn test_decrypt_non_a_z() {
        assert_eq!(decrypt("---", RADIX), "");
    }

    #[test]
    fn test_encrypt_numbers() {
        assert_eq!(encrypt("27389303", RADIX), "");
    }
}
