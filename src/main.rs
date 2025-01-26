use clap::builder::Str;
use serde_json::{self};
use std::{collections::HashMap, env};

// Available if you need it!
// use serde_bencode

fn decode_bencoded_value(encoded_value: &str) -> anyhow::Result<serde_json::Value> {
    let value: serde_bencode::value::Value = serde_bencode::from_str(encoded_value)?;
    convert(value)
}
fn convert(value: serde_bencode::value::Value) -> anyhow::Result<serde_json::Value> {
    match value {
        serde_bencode::value::Value::Bytes(b) => {
            let string = String::from_utf8(b)?;
            Ok(serde_json::Value::String(string))
        }
        serde_bencode::value::Value::Int(i) => {
            Ok(serde_json::Value::Number(serde_json::Number::from(i)))
        }
        serde_bencode::value::Value::List(l) => {
            let array = l
                .into_iter()
                .map(|item| convert(item))
                .collect::<anyhow::Result<Vec<serde_json::Value>>>()?;
            Ok(serde_json::Value::Array(array))
        }
        serde_bencode::value::Value::Dict(d) => {
            let mut dict: serde_json::Map<std::string::String, serde_json::Value> =
                serde_json::Map::new();

            d.iter().for_each(|(k, v)| {
                dict.insert(
                    String::from_utf8(k.to_vec()).unwrap(),
                    convert(v.clone()).unwrap(),
                );
            });

            Ok(serde_json::Value::Object(dict))
        } // _ => {
          //     panic!("Unhandled encoded value: {:?}", value)
          // }
    }
}
// Usage: your_bittorrent.sh decode "<encoded_value>"

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    if command == "decode" {
        // Uncomment this block to pass the first stage
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value)?;
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
    Ok(())
}
