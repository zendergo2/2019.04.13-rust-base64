pub struct Encoder {
    pub logging: bool
}

impl Encoder {
    pub fn encode(&self, input: &str) -> String {
        // need to process a multiple of 4 bytes, pad extra with =
        let mut chars = input.chars();
        let mut count: u8 = 0;
        let mut next: u8 = 0;
        let mut result: String = String::new();
        loop {
            match chars.next() {
                Some(c) => {
                    let hold: u8 = c as u8;
                    let mut out: u8 = 0;
                    if count % 3 == 0 {
                        // get largest 6 bits as output
                        out = hold >> 2;
                        next = (hold & 0b00_00_00_11) << 4;
                        if self.logging {
                            println!("count 0: {:08b} | {:08b} | {:08b} | {} | {}", hold, out, next, self.get_b64_char(&out), self.get_b64_char(&next));
                        }
                    }
                    else if count % 3 == 1 {
                        // Get 2 bits from previous, 4 bits from current
                        out = (hold >> 4) | next;
                        next = (hold & 0b00_00_11_11) << 2;
                        if self.logging {
                            println!("count 1: {:08b} | {:08b} | {:08b} | {} | {}", hold, out, next, self.get_b64_char(&out), self.get_b64_char(&next));
                        }
                    }
                    else if count % 3 == 2 {
                        // Get 4 bits from previous, 2 bits from current
                        out = (hold >> 6) | next;
                        next = hold & 0b00_11_11_11;
                        if self.logging {
                            println!("count 2: {:08b} | {:08b} | {:08b} | {} | {}", hold, out, next, self.get_b64_char(&out), self.get_b64_char(&next));
                        }
                    }
                    result.push(self.get_b64_char(&out));
                    if count % 3 == 2 {
                        // Get last 6 bits from current
                        if self.logging {
                            println!("last 6: {:08b} | {}", next, self.get_b64_char(&next));
                        }
                        result.push(self.get_b64_char(&next));
                    }
                    count += 1;
                },
                None => {
                    if count % 3 == 0 {
                        break;
                    }
                    if count % 3 == 1 || count % 3 == 2 {
                        result.push(self.get_b64_char(&next));
                        if self.logging {
                            println!("done next: {:08b} | {}", next, self.get_b64_char(&next));
                        }
                        count += 1;
                    }
                    while count % 3 != 1 {
                        result.push('=');
                        if self.logging {
                            println!("done =");
                        }
                        count += 1;
                    }
                    break;
                }
            }
        }
        //let ch: char = input.chars().next().unwrap();
        //println!("{:08b} | {:08b} | {} | {}", next, out, self.get_b64_char(out), self.get_b64_char(next));
        return result.to_string();
    }

    // 6 bits -> equivalent 64bit character
    fn get_b64_char(&self, in_byte: &u8) -> char {
        // println!("{:b} {}", in_byte, in_byte);
        if *in_byte < 26 {
            // Capital letters (0-25 => 65-90)
            char::from(in_byte + 65)
        } else if *in_byte < 52 {
            // Lowercase letters (26-51 => 97-122)
            char::from(in_byte + 71)
        } else if *in_byte < 62 {
            // Numbers (52-61 => 48-57)
            char::from(in_byte - 4)
        } else if *in_byte == 62 {
            // 62
            '/'
        } else if *in_byte == 63 {
            // 63
            '+'
        } else {
            '#'
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wiki_1() {
        let enc = Encoder{logging: true};
        assert_eq!(enc.encode("Man"), "TWFu");
    }
    #[test]
    fn wiki_2() {
        let enc = Encoder{logging: true};
        assert_eq!(enc.encode("Ma"), "TWE=");
    }
    #[test]
    fn wiki_3() {
        let enc = Encoder{logging: true};
        assert_eq!(enc.encode("M"), "TQ==");
    }
    #[test]
    fn long_string() {
        let enc = Encoder{logging: true};
        assert_eq!(enc.encode("asdflkjwefoinvvofoifjdfjasdlvk"), "YXNkZmxrandlZm9pbnZ2b2ZvaWZqZGZqYXNkbHZr");
    }
    #[test]
    fn quick_brown() {
        let enc = Encoder{logging: true};
        assert_eq!(enc.encode("The quick brown fox jumps over the lazy dog."), "VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXp5IGRvZy4=");
    }

}
