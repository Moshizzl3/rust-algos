mod algoritms;
mod data_structures;

use std::{collections::VecDeque, fs::read_dir};

// use crate::data_structures::hash_tables::MoMap;
use crate::{
    algoritms::search,
    data_structures::graphs::{MoGraph, Node},
};

fn main() {
    let mut graph = MoGraph::new();

    graph.add_node(Node {
        name: "you".to_string(),
        is_seller: false,
    });
    graph.add_node(Node {
        name: "a".to_string(),
        is_seller: false,
    });
    graph.add_node(Node {
        name: "b".to_string(),
        is_seller: true,
    });
    graph.add_node(Node {
        name: "c".to_string(),
        is_seller: false,
    });
    graph.add_node(Node {
        name: "d".to_string(),
        is_seller: true,
    });

    graph.add_edge("you".to_string(), "a".to_string());
    graph.add_edge("you".to_string(), "b".to_string());
    graph.add_edge("a".to_string(), "c".to_string());
    graph.add_edge("c".to_string(), "d".to_string());

    println!("BFS: {:?}", graph.bfs("you", &|x| x.is_seller));
    println!("DFS: {:?}", graph.dfs("you", &|x| x.is_seller))
}
