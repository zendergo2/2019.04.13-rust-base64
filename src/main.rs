mod base64;
use base64::Encoder::Encoder;
fn main() {
    let mut to_encode = "Man";
    let mut expected_result = "TWFu";
    let b64_encoder = Encoder{};
    println!("{}, {} = {}", to_encode, b64_encoder.encode(to_encode), expected_result);
    
    to_encode = "Ma";
    expected_result = "TWE=";
    println!("{}, {} = {}", to_encode, b64_encoder.encode(to_encode), expected_result);

    to_encode = "M";
    expected_result = "TQ==";
    println!("{}, {} = {}", to_encode, b64_encoder.encode(to_encode), expected_result);
}

