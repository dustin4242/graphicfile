fn main() {
    to_gf(str_to_bytes(
        "
        00000000
        00000111
        00011000
        00100010
        01000100
        00100010
        00011000
        00000110
        00011000
        00100010
        01000100
        00100010
        00011000
        00000111
        00000000
        00000000",
        1,
        1,
    ));
}

fn to_cgf() {
    let mut file = Vec::new();
    let mut header = vec![1, 1];
    let mut graphic = vec![
        0x00, 0x07, 0x18, 0x22, 0x44, 0x22, 0x18, 0x06, 0x18, 0x22, 0x44, 0x22, 0x18, 0x7, 0x00,
    ];
    let mut color_table = vec![0x00, 0x0F, 0x2A, 0x00];
    let mut color_data = vec![];
    file.append(header.as_mut());
    file.append(graphic.as_mut());
    file.append(color_table.as_mut());
    file.append(color_data.as_mut());
    std::fs::write("./gf.cgf", graphic).unwrap();
}

fn str_to_bytes(string: &str, width: u8, height: u8) -> Vec<u8> {
    let no_whitespace = string.split_whitespace().collect::<String>();
    let mut bytes = vec![width, height];
    for y in 0..(height * 16) as usize {
        for x in 0..width as usize {
            let index = y * width as usize * 8 + x * 8;
            let mut sub = no_whitespace.get(index..index + 8).unwrap().chars();
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
