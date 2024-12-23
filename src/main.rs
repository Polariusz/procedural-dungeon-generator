use procedural_dungeon_generator::{
    generate_map_with_empty_tiles, generate_structures_one, print_map,
};

fn main() {
    for i in 0..=255 {
        println!("Map {i}:");
        let mut map = generate_map_with_empty_tiles(160, 40);
        let rnd_index = i;
        generate_structures_one(&mut map, rnd_index);
        print_map(&map);
    }
}
