use serde_bencode::de;
use serde_json::{self, Number};
use std::env;

// Available if you need it!
// use serde_bencode

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    match de::from_str(encoded_value) {
        Ok(v) => serde_json::Value::String(v),
        Err(e) => panic!("{}", e),
    }
    // // If encoded_value starts with a digit, it's a number
    // let mut chars = encoded_value.chars().into_iter().peekable();
    // let next_token = chars.peek().unwrap();
    // if next_token.is_digit(10) {
    //     // Example: "5:hello" -> "hello"
    //     let colon_index = encoded_value.find(':').unwrap();
    //     let number_string = &encoded_value[..colon_index];
    //     let number = number_string.parse::<i64>().unwrap();
    //     let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
    //     return ;
    // } else if next_token == &'i' {
    //     let e_index = encoded_value.find('e').unwrap();
    //     let number_string = &encoded_value[1..e_index];
    //     let number = number_string.parse::<i64>().unwrap();
    //     return serde_json::Value::Number(Number::from(number));
    // } else {
    //     panic!("Unhandled encoded value: {}", encoded_value)
    // }
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        // You can use print statements as follows for debugging, they'll be visible when running tests.
        eprintln!("Logs from your program will appear here!");

        // Uncomment this block to pass the first stage
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}
