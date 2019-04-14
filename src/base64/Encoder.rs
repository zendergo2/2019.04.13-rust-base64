pub struct Encoder {
}

impl Encoder {
    pub fn encode(&self, input: &str) -> String {
        // create output string to hold adjusted characters
        let mut buf = String::with_capacity(input.len() * (4/3));
        let mut chars = input.chars();
        let mut ch: char = 'a';
        let mut bits_to_consume: u8 = 6;
        let mut bits_to_mask: u8 = 2;
        let mut mask: u16 = 0;
        let mut swap: u8 = 0;
        let mut rust: u8 = 0;
        loop {
            match chars.next() {
                Some(c) => ch = c,
                None => {
                    buf.push(self.get_b64_char(rust)); // Whatever's left in rust gets output
                    match bits_to_consume {
                        2 => buf.push('='), 
                        4 => {
                            buf.push('=');
                            buf.push('=');
                        },
                        _ => break,
                    }
                    break;
                },
            }
            if bits_to_consume == 0 {
                swap = 0;
            } else {
                swap = (ch as u8) >> (((ch.len_utf8() as u8) * 8) - bits_to_consume);
            }
            buf.push(self.get_b64_char(rust | swap));

            if bits_to_consume == 0 { bits_to_consume = 6; }
            else { bits_to_consume -= 2; }
            mask = (2 as u16).pow(bits_to_mask as u32) - 1;
            rust = ((ch as u8) & mask as u8) << bits_to_consume; // bring back our dropped bits 
            bits_to_mask = 8 - bits_to_consume;
            println!("{:?}", (mask, swap, rust, bits_to_consume, bits_to_mask));
        }
        buf
    }

    fn get_b64_char(&self, in_byte: u8) -> char {
        // println!("{:b} {}", in_byte, in_byte);
        if in_byte < 26 {
            // Capital letters
            char::from(in_byte + 65)
        } else if in_byte < 52 {
            // Lowercase letters
            char::from(in_byte + 71)
        } else if in_byte < 62 {
            // Numbers
            char::from(in_byte - 4)
        } else if in_byte == 63 {
            '/'
        } else {
            '+'
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wiki_1() {
        let enc = Encoder{};
        assert_eq!(enc.encode("Man"), "TWFu");
    }
    #[test]
    fn wiki_2() {
        let enc = Encoder{};
        assert_eq!(enc.encode("Ma"), "TWE=");
    }
    #[test]
    fn wiki_3() {
        let enc = Encoder{};
        assert_eq!(enc.encode("M"), "TQ==");
    }
    #[test]
    fn long_string() {
        let enc = Encoder{};
        assert_eq!(enc.encode("asdflkjwefoinvvofoifjdfjasdlvk"), "YXNkZmxrandlZm9pbnZ2b2ZvaWZqZGZqYXNkbHZr");
    }

}
