use core::panic;
use std::str::from_utf8_unchecked;

const RND_TABLE: [u8; 256] = [
    0, 8, 109, 220, 222, 241, 149, 107, 75, 248, 254, 140, 16, 66, 74, 21, 211, 47, 80, 242, 154,
    27, 205, 128, 161, 89, 77, 36, 95, 110, 85, 48, 212, 140, 211, 249, 22, 79, 200, 50, 28, 188,
    52, 140, 202, 120, 68, 145, 62, 70, 184, 190, 91, 197, 152, 224, 149, 104, 25, 178, 252, 182,
    202, 182, 141, 197, 4, 81, 181, 242, 145, 42, 39, 227, 156, 198, 225, 193, 219, 93, 122, 175,
    249, 0, 175, 143, 70, 239, 46, 246, 163, 53, 163, 109, 168, 135, 2, 235, 25, 92, 20, 145, 138,
    77, 69, 166, 78, 176, 173, 212, 166, 113, 94, 161, 41, 50, 239, 49, 111, 164, 70, 60, 2, 37,
    171, 75, 136, 156, 11, 56, 42, 146, 138, 229, 73, 146, 77, 61, 98, 196, 135, 106, 63, 197, 195,
    86, 96, 203, 113, 101, 170, 247, 181, 113, 80, 250, 108, 7, 255, 237, 129, 226, 79, 107, 112,
    166, 103, 241, 24, 223, 239, 120, 198, 58, 60, 82, 128, 3, 184, 66, 143, 224, 145, 224, 81,
    206, 163, 45, 63, 90, 168, 114, 59, 33, 159, 95, 28, 139, 123, 98, 125, 196, 15, 70, 194, 253,
    54, 14, 109, 226, 71, 17, 161, 93, 186, 87, 244, 138, 20, 52, 123, 251, 26, 36, 17, 46, 52,
    231, 232, 76, 31, 221, 84, 37, 216, 165, 212, 106, 197, 242, 98, 43, 39, 175, 254, 145, 190,
    84, 118, 222, 187, 136, 120, 163, 236, 249,
];

pub fn generate_map_with_empty_tiles(x: u8, y: u8) -> Vec<Vec<u8>> {
    let mut map: Vec<Vec<u8>> = Vec::new();
    for _ in 0..y {
        let mut row: Vec<u8> = Vec::new();
        for _ in 0..x {
            row.push(b' ');
        }
        map.push(row);
    }
    map
}

pub fn print_map(map: &Vec<Vec<u8>>) {
    if map.get(0).is_none() || map.get(0).unwrap().get(0).is_none() {
        panic!("Arg: `map` should not be empty.");
    }

    print!("+");
    for _ in 0..map.get(0).unwrap().len() {
        print!("-");
    }
    println!("+");
    for row in map {
        let row2 = unsafe { from_utf8_unchecked(row) };
        println!("|{}|", row2)
    }
    print!("+");
    for _ in 0..map.get(0).unwrap().len() {
        print!("-");
    }
    println!("+");
}

pub fn generate_structures_one(map: &mut Vec<Vec<u8>>, mut rnd_index: u8) {
    if map.get(0).is_none() || map.get(0).unwrap().get(0).is_none() {
        panic!("Arg: `map` should not be empty.")
    }

    let row_cnt = map.len();
    let row_len = map.get(0).unwrap().len();
    let amount_of_structure_generations = get_rnd_table_value(&mut rnd_index);
    for _ in 0..amount_of_structure_generations {
        let x1 = ((get_rnd_table_value(&mut rnd_index) as f32 / 255.0) * row_len as f32) as u8;
        let x2 = ((get_rnd_table_value(&mut rnd_index) as f32 / 255.0) * row_len as f32) as u8;
        let y1 = ((get_rnd_table_value(&mut rnd_index) as f32 / 255.0) * row_cnt as f32) as u8;
        let y2 = ((get_rnd_table_value(&mut rnd_index) as f32 / 255.0) * row_cnt as f32) as u8;
        println!("|x1:{x1}| |x2:{x2}| |y1:{y1}| |y2:{y2}|");

        paint(map, x1, x2, y1, y2);
    }
}

fn paint(map: &mut Vec<Vec<u8>>, x1: u8, x2: u8, y1: u8, y2: u8) {
    for y_index in y1..=y2 {
        for x_index in x1..=x2 {
            if y1 == y_index || y2 == y_index || x_index == x1 || x_index == x2 {
                *map.get_mut(y_index as usize)
                    .unwrap()
                    .get_mut(x_index as usize)
                    .unwrap() = b'#';
            } else {
                *map.get_mut(y_index as usize)
                    .unwrap()
                    .get_mut(x_index as usize)
                    .unwrap() = b'.';
            }
        }
    }
}

fn get_rnd_table_value(rnd_index: &mut u8) -> u8 {
    let val_to_return = RND_TABLE[*rnd_index as usize];
    match rnd_index {
        255 => *rnd_index = 0,
        _ => *rnd_index += 1,
    }
    val_to_return
}
