use std::u8;

pub fn stob(s: &str) -> Vec<u8> {
    let midi = s.replace(" ", "");

    let mut bytes: Vec<u8> = Vec::new();
    let mut i: bool = false;
    let mut bs: String = String::from("");

    for c in midi.chars() {
        if i {
            // Calculate the byte's value
            bs.push(c);
            bytes.push(
                u8::from_str_radix(bs.as_str(), 16).expect("Error parsing the HEX code in stob"),
            );
            bs.clear();
            i = false;
        } else {
            // Store and wait for next byte
            bs.push(c);
            i = true;
        }
    }
    return bytes;
}
