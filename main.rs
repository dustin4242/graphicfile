fn main() {
    to_gf();
}

fn to_gf() {
    let graphic = [
        1, 1, 0x00, 0x07, 0x18, 0x22, 0x44, 0x22, 0x18, 0x06, 0x18, 0x22, 0x44, 0x22, 0x18, 0x7,
        0x00,
    ];
    std::fs::write("./gf.gf", graphic).unwrap();
}
