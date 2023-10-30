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
    // header section
    let mut bytes = vec![width, height];
    // data section
    let mut index;
    for y in 0..(height * 16) as usize {
        for x in 0..width as usize {
            let index = y * width as usize + x;
            bytes.push(get_byte(&no_whitespace, x, index * 8));
        }
    }
    // color section
    index = width as usize * 8 * height as usize * 16;
    if no_whitespace.get(index..index + 8 * (1 + 4)).is_some() {
        for x in 0..4 {
            bytes.push(get_byte(&no_whitespace, x, index));
        }
        index = index + 8 * 4;
        while no_whitespace.get(index..index + 8).is_some() {
            bytes.push(get_byte(&no_whitespace, 0, index));
            index += 8;
        }
    }
    print!("{bytes:?}");
    bytes
}

fn get_byte(string: &String, offset: usize, index: usize) -> u8 {
    let mut sub = string
        .get(index + 8 * offset..index + 8 * (offset + 1))
        .unwrap_or("00000000")
        .chars();
    let mut byte: u8 = 0;
    for i in 0..8 {
        if sub.next().unwrap() == '1' {
            byte += 0x80 >> i;
        };
    }
    byte
}

fn to_gf(graphic_bytes: Vec<u8>) {
    std::fs::write("./gf.gf", graphic_bytes).unwrap();
}
