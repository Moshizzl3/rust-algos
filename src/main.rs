mod algoritms;
mod data_structures;

use crate::data_structures::hash_tables::MoMap;

fn main() {
    let mut my_map: MoMap<u32> = MoMap::new();

    my_map.bla();

    my_map.insert("hello".to_string(), 32);
    my_map.bla();
    my_map.insert("hello1".to_string(), 32);
    my_map.bla();
    my_map.insert("aaaa".to_string(), 42);
    my_map.bla();
    my_map.insert("mohamad".to_string(), 11);
    my_map.bla();
    my_map.insert("mohasbcdqdasdaamad".to_string(), 11);
    my_map.bla();
    my_map.insert("ddddddd".to_string(), 11);
    my_map.bla();
    my_map.insert("ddddaaaaaaddd".to_string(), 11);
    my_map.bla();
    my_map.insert("llllllll".to_string(), 11);
    my_map.bla();
    my_map.insert("kkkkkkkk".to_string(), 11);
    my_map.bla();
    my_map.insert("k".to_string(), 11);
    my_map.bla();
    my_map.insert("a".to_string(), 11);
    my_map.bla();
    my_map.insert("b".to_string(), 11);
    my_map.bla();
    my_map.insert("c".to_string(), 11);
    my_map.bla();
    let test = my_map.get("mohamad");

    println!("woow: {:?}", test);
    my_map.remove("mohamad");
    let test = my_map.get("mohamad");
    println!("woow2: {:?}", test);
    my_map.insert("aaaa".to_string(), 123);
    my_map.bla();
}
