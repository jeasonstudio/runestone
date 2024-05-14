use super::*;
use std::fmt::Write;

pub fn decode_hex_to_bytes(s: String) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

#[wasm_bindgen(js_name = "decodeHex")]
pub fn decode_hex(s: String) -> js_sys::Uint8Array {
    js_sys::Uint8Array::from(decode_hex_to_bytes(s).as_slice())
}

pub fn encode_bytes_to_hex(bytes: Vec<u8>) -> String {
    let buf = bytes.as_slice();
    let mut result = String::with_capacity(buf.len() * 2);
    for &b in buf {
        write!(&mut result, "{:02x}", b).unwrap();
    }
    result
}

#[wasm_bindgen(js_name = "encodeHex")]
pub fn encode_hex(bytes: js_sys::Uint8Array) -> String {
    let bytes = bytes.to_vec();
    encode_bytes_to_hex(bytes)
}
