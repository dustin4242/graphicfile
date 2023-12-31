fn main() {
    color_cat()
}

fn color_cat() {
    let cat = std::fs::read_to_string("./color_cat.txt").unwrap();
    export_gf(_txt_to_graphicfile(&cat, 1, 1));
}

fn nyan() {
    let ppm = std::fs::read_to_string("nyan.ppm").unwrap();
    let mut no_comment = ppm.split("\n").collect::<Vec<&str>>();
    if no_comment.get(1).unwrap().starts_with("#") {
        no_comment.remove(1);
    };
    let graphicfile = ppm_to_graphicfile(no_comment.join("\n"));
    export_gf(graphicfile.unwrap());
}

fn export_gf(graphic_bytes: Vec<u8>) {
    std::fs::write("./gf.gf", graphic_bytes).unwrap();
}

fn _txt_to_graphicfile(string: &str, width: u8, height: u8) -> Vec<u8> {
    let no_whitespace = string.split_whitespace().collect::<String>();
    // header section
    let mut bytes = vec![width, height];
    // data section
    let mut index;
    for y in 0..(height * 16) as usize {
        for x in 0..width as usize {
            let index = y * width as usize * 8 + x * 8;
            bytes.push(_get_byte(&no_whitespace, x, index));
        }
    }
    // color section
    index = width as usize * 8 * height as usize * 16;
    if no_whitespace.get(index..index + 8 * 8 * 3).is_some() {
        for x in 0..8 * 3 {
            bytes.push(_get_byte(&no_whitespace, x, index));
        }
        index = index + 8 * 8 * 3;
        while no_whitespace.get(index..index + 8).is_some() {
            bytes.push(_get_byte(&no_whitespace, 0, index));
            index += 8;
        }
    }
    print!("{bytes:?}");
    bytes
}

fn _get_byte(string: &String, offset: usize, index: usize) -> u8 {
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

fn ppm_to_graphicfile(ppm: String) -> Option<Vec<u8>> {
    let mut gf = Vec::new();
    let mut info = ppm.split_whitespace();
    info.next();
    let width = info.next()?.parse::<usize>().unwrap();
    let height = info.next()?.parse::<usize>().unwrap();
    let division: u8 = ((info.next()?.parse::<usize>().unwrap() + 1) / 64) as u8;
    let mut image_data = info;
    let mut converted = Vec::new();
    let mut bitpixels = Vec::new();
    for _y in 0..height {
        for _x in 0..width / 8 {
            let mut byte: u8 = 0;
            for x in 0..8 {
                let rgb = (
                    image_data.next()?.parse::<u8>().unwrap() / division,
                    image_data.next()?.parse::<u8>().unwrap() / division,
                    image_data.next()?.parse::<u8>().unwrap() / division,
                );
                if (rgb.0 + rgb.1 + rgb.2) > 0 {
                    byte += 0x80 >> x;
                };
                converted.push(rgb);
            }
            bitpixels.push(byte);
        }
    }
    let mut pallete: Vec<(u8, u8, u8)> = Vec::with_capacity(8);
    for x in converted.to_owned() {
        if !pallete.contains(&x) {
            pallete.push(x);
        };
    }
    let mut color_information: Vec<u8> = Vec::new();
    let mut color_index = 0;
    let mut byte: u8 = 0;
    for x in 0..converted.len() {
        if converted.get(x).unwrap() != &(0, 0, 0) {
            byte += (pallete
                .iter()
                .position(|p| p == converted.get(x).unwrap())
                .unwrap() as u8)
                << (color_index % 2) * 4;
            color_index += 1;
            if color_index % 2 == 0 {
                color_information.push(byte);
                byte = 0;
            }
        }
    }
    print!("{color_information:?}");
    gf.append(vec![width as u8 / 8].as_mut());
    gf.append(vec![height as u8 / 16].as_mut());
    gf.append(bitpixels.as_mut());
    for x in pallete {
        gf.push(x.0);
        gf.push(x.1);
        gf.push(x.2);
    }
    gf.append(color_information.as_mut());
    Some(gf)
}
