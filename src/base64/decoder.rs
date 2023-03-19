pub struct Decoder{
    pub logging: bool
}

impl Decoder {
    pub fn decode(&self, input: &str) -> String {
        let mut chars = input.chars();
        let mut count: u8 = 0;
        let mut prev: u8 = 0;
        let mut result: String = String::new();
        loop {
            match chars.next() {
                Some(c) => {
                    let curr: u8 = self.get_bits(&c);
                    let mut out: u8 = 0;
                    if count % 4 == 0 {
                        // We have the 6 lower bits from current to put in prev
                        prev = curr << 2;
                        if self.logging {
                            println!("count 0: {} | {:08b} | {:08b}", c, out, prev);
                        }
                    }
                    else if count % 4 == 1 {
                        // We have 6 bits from prev, use 2 from curr + save 4
                        out = prev | ((curr & 0b00_11_00_00) >> 4);
                        prev = (curr & 0b00_00_11_11) << 4;
                        if self.logging {
                            println!("count 1: {} | {:08b} | {:08b}", c, out, prev);
                        }
                    }
                    else if count % 4 == 2 {
                        // We have 4 bits from prev, use 4 from curr + save 2
                        out = prev | ((curr & 0b00_11_11_00) >> 2);
                        prev = (curr & 0b00_00_00_11) << 6;
                        if self.logging {
                            println!("count 2: {} | {:08b} | {:08b}", c, out, prev);
                        }
                    }
                    else if count % 4 == 3 {
                        // We have 2 bits from prev, use 6 from curr
                        out = prev | curr;
                        prev = 0;
                        if self.logging {
                            println!("count 3: {} | {:08b} | {:08b}", c, out, prev);
                        }
                    }
                    if out != 0 {
                        if self.logging {
                            println!("found char: {:08b} | {}", out, out as char);
                        }
                        result.push(out as char);
                    }
                    count += 1;
                },
                None => {
                    break;
                }
            }
        }
        return result;
    }
    fn get_bits(&self, in_char: &char) -> u8 {
        let result: u8 = *in_char as u8;
        if *in_char >= 'A' && *in_char <= 'Z' {
            // Capital letters (65-90 => 0-25)
            result - 65
        } else if *in_char >= 'a' && *in_char <= 'z' {
            // Lowercase letters (97-122 => 26-51)
            result - 71
        } else if *in_char >= '0' && *in_char <= '9' {
            // Numbers (48-57 => 52-61)
            result + 4
        } else if *in_char == '/' {
            // 62
            62
        } else if *in_char == '+' {
            // 63
            63
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wiki_1() {
        let dec = Decoder{logging: true};
        assert_eq!(dec.decode("TWFu"), "Man");
    }
    #[test]
    fn wiki_2() {
        let dec = Decoder{logging: true};
        assert_eq!(dec.decode("TWE="), "Ma");
    }
    #[test]
    fn wiki_3() {
        let dec = Decoder{logging: true};
        assert_eq!(dec.decode("TQ=="), "M");
    }
    #[test]
    fn long_string() {
        let dec = Decoder{logging: true};
        assert_eq!(dec.decode("YXNkZmxrandlZm9pbnZ2b2ZvaWZqZGZqYXNkbHZr"), "asdflkjwefoinvvofoifjdfjasdlvk");
    }
    #[test]
    fn quick_brown() {
        let dec = Decoder{logging: true};
        assert_eq!(dec.decode("VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXp5IGRvZy4="), "The quick brown fox jumps over the lazy dog.");
    }
}