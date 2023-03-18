use std::env;
mod base64;
use base64::encoder::Encoder;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let to_encode = &args[1];
    let expected_result = &args[2];
    let b64_encoder = Encoder{};
    println!("{}, {} = {}", to_encode, b64_encoder.encode(to_encode), expected_result);
}

