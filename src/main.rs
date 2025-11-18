mod algoritms;
mod data_structures;

// use crate::data_structures::hash_tables::MoMap;
use crate::data_structures::graphs::{MoGraph, Profile};

fn main() {
    let mut mo_graph = MoGraph::new();
    mo_graph.add_edge(
        "Mo".to_string(),
        Profile {
            name: "bob".to_string(),
            is_seller: false,
        },
    );
    mo_graph.add_edge(
        "Mo".to_string(),
        Profile {
            name: "Moh".to_string(),
            is_seller: true,
        },
    );
    mo_graph.add_edge(
        "Moh".to_string(),
        Profile {
            name: "Susan".to_string(),
            is_seller: false,
        },
    );
    mo_graph.add_edge(
        "Susan".to_string(),
        Profile {
            name: "henry".to_string(),
            is_seller: true,
        },
    );

    println!("{:?}", mo_graph);

    let seller = mo_graph.bfs("Mo", |x| x.is_seller);

    println!("The first seller is: {:?}", seller)
}
