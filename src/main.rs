mod algoritms;
mod data_structures;

// use crate::data_structures::hash_tables::MoMap;
use crate::data_structures::hash_tables_generic::MoMap;

fn main() {
    let mut my_map: MoMap<u32, u32> = MoMap::new();

    my_map.bla();

    my_map.insert(1, 32);
    my_map.bla();
    my_map.insert(2, 32);
    my_map.bla();
    my_map.insert(3, 42);
    my_map.bla();
    my_map.insert(3, 11);
    my_map.bla();
    my_map.insert(4, 11);
    my_map.bla();
    my_map.insert(5, 11);
    my_map.bla();
    my_map.insert(6, 11);
    my_map.bla();
    my_map.insert(7, 11);
    my_map.bla();
    my_map.insert(8, 11);
    my_map.bla();
    my_map.insert(9, 11);
    my_map.bla();
    my_map.insert(10, 11);
    my_map.bla();
    my_map.insert(11, 11);
    my_map.bla();
    my_map.insert(12, 11);
    my_map.bla();
    let test = my_map.get(&3);

    println!("woow: {:?}", test);
    my_map.remove(&3);
    let test = my_map.get(&3);
    println!("woow2: {:?}", test);
    my_map.insert(3, 123);
    my_map.bla();
}
