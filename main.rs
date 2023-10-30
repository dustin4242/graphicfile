use std::{env, fs};

fn main() {
    //cat ears
    to_gf(str_to_bytes(
        &fs::read_to_string(env::args().nth(1).unwrap()).unwrap(),
        1,
        1,
    ));
}

fn str_to_bytes(string: &str, width: u8, height: u8) -> Vec<u8> {
    let no_whitespace = string.split_whitespace().collect::<String>();
    let mut bytes = vec![width, height];
    for y in 0..(height * 16) as usize {
        for x in 0..width as usize {
            let index = y * width as usize * 8 + x * 8;
            let mut sub = no_whitespace
                .get(index..index + 8)
                .unwrap_or("00000000")
                .chars();
            let mut byte: u8 = 0;
            for i in 0..8 {
                if sub.next().unwrap() == '1' {
                    byte += 0x80 >> i;
                };
            }
            bytes.push(byte);
        }
    }
    print!("{bytes:?}");
    bytes
}

fn to_gf(graphic_bytes: Vec<u8>) {
    std::fs::write("./gf.gf", graphic_bytes).unwrap();
}
