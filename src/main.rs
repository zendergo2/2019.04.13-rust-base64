use std::env;
mod base64;
use base64::encoder::Encoder;
use base64::decoder::Decoder;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <encode/decode> <string/base64>", args[0]);
        return;
    }
    let command = &args[1];
    let string_or_b64 = &args[2];

    if command == "encode" {
        let b64_encoder = Encoder{logging: false};
        println!("{}", b64_encoder.encode(string_or_b64));
    } else if command == "decode" {
        let b64_decoder = Decoder{logging: false};
        println!("{}", b64_decoder.decode(string_or_b64));
    } else {
        println!("Usage: {} <encode/decode> <string/base64>", args[0]);
    }
}

