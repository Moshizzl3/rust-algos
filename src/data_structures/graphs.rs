// simple concept of graph

use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub is_seller: bool,
}

#[derive(Debug)]
pub struct MoGraph {
    nodes: HashMap<String, Node>,
    adjacency_list: HashMap<String, Vec<String>>,
}

impl MoGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.entry(node.name.clone()).or_insert(node);
    }

    pub fn add_edge(&mut self, from: String, to: String) {
        self.adjacency_list.entry(from).or_default().push(to);
    }

    pub fn neighbors(&self, node: &str) -> Option<&Vec<String>> {
        self.adjacency_list.get(node)
    }

    pub fn get_node(&self, key: &str) -> Option<&Node> {
        self.nodes.get(key)
    }

    /// Breadth-first search
    ///  # Arguments
    ///
    /// * `start` - The name of the starting node
    /// * `condition` - A predicate function that returns `true` when the desired node is found
    ///
    /// # Returns
    ///
    /// Returns `Some(Profile)` if a node matching the condition is found, `None` otherwise.
    ///
    pub fn bfs<F>(&self, start: &str, condition: F) -> Option<&Node>
    where
        F: Fn(&Node) -> bool,
    {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(start.to_string());

        while let Some(key) = queue.pop_front() {
            if let Some(node) = self.nodes.get(&key)
                && condition(node)
            {
                return Some(node);
            }
            if let Some(neighbors) = self.neighbors(&key) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a simple graph for testing
    fn create_test_graph() -> MoGraph {
        let mut graph = MoGraph::new();

        // nodes
        graph.add_node(Node {
            name: "you".to_string(),
            is_seller: false,
        });
        graph.add_node(Node {
            name: "alice".to_string(),
            is_seller: false,
        });
        graph.add_node(Node {
            name: "bob".to_string(),
            is_seller: false,
        });
        graph.add_node(Node {
            name: "claire".to_string(),
            is_seller: false,
        });
        graph.add_node(Node {
            name: "anuj".to_string(),
            is_seller: false,
        });
        graph.add_node(Node {
            name: "peggy".to_string(),
            is_seller: false,
        });
        graph.add_node(Node {
            name: "thom".to_string(),
            is_seller: true,
        });
        graph.add_node(Node {
            name: "jonny".to_string(),
            is_seller: true,
        });

        // edges (from the book example)
        graph.add_edge("you".to_string(), "alice".to_string());
        graph.add_edge("you".to_string(), "bob".to_string());
        graph.add_edge("you".to_string(), "claire".to_string());
        graph.add_edge("bob".to_string(), "anuj".to_string());
        graph.add_edge("bob".to_string(), "peggy".to_string());
        graph.add_edge("alice".to_string(), "peggy".to_string());
        graph.add_edge("claire".to_string(), "thom".to_string());
        graph.add_edge("claire".to_string(), "jonny".to_string());

        graph
    }

    // graph structure tests
    #[test]
    fn test_new_graph_is_empty() {
        let graph = MoGraph::new();
        assert_eq!(graph.nodes.len(), 0);
        assert_eq!(graph.adjacency_list.len(), 0);
    }

    #[test]
    fn test_add_node() {
        let mut graph = MoGraph::new();
        graph.add_node(Node {
            name: "alice".to_string(),
            is_seller: false,
        });

        assert_eq!(graph.nodes.len(), 1);
        assert!(graph.get_node("alice").is_some());
    }

    #[test]
    fn test_add_duplicate_node() {
        let mut graph = MoGraph::new();
        graph.add_node(Node {
            name: "alice".to_string(),
            is_seller: false,
        });
        graph.add_node(Node {
            name: "alice".to_string(),
            is_seller: true,
        });

        // Should only have one node (or_insert doesn't replace)
        assert_eq!(graph.nodes.len(), 1);
        // Original value should remain
        assert_eq!(graph.get_node("alice").unwrap().is_seller, false);
    }

    #[test]
    fn test_add_edge() {
        let mut graph = MoGraph::new();
        graph.add_node(Node {
            name: "alice".to_string(),
            is_seller: false,
        });
        graph.add_node(Node {
            name: "bob".to_string(),
            is_seller: false,
        });

        graph.add_edge("alice".to_string(), "bob".to_string());

        let neighbors = graph.neighbors("alice").unwrap();
        assert_eq!(neighbors.len(), 1);
        assert_eq!(neighbors[0], "bob");
    }

    #[test]
    fn test_add_multiple_edges_from_same_node() {
        let mut graph = MoGraph::new();
        graph.add_node(Node {
            name: "you".to_string(),
            is_seller: false,
        });
        graph.add_node(Node {
            name: "alice".to_string(),
            is_seller: false,
        });
        graph.add_node(Node {
            name: "bob".to_string(),
            is_seller: false,
        });

        graph.add_edge("you".to_string(), "alice".to_string());
        graph.add_edge("you".to_string(), "bob".to_string());

        let neighbors = graph.neighbors("you").unwrap();
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&"alice".to_string()));
        assert!(neighbors.contains(&"bob".to_string()));
    }

    #[test]
    fn test_neighbors_nonexistent_node() {
        let graph = MoGraph::new();
        assert!(graph.neighbors("nobody").is_none());
    }

    #[test]
    fn test_get_node() {
        let mut graph = MoGraph::new();
        graph.add_node(Node {
            name: "alice".to_string(),
            is_seller: true,
        });

        let node = graph.get_node("alice").unwrap();
        assert_eq!(node.name, "alice");
        assert_eq!(node.is_seller, true);
    }

    #[test]
    fn test_get_nonexistent_node() {
        let graph = MoGraph::new();
        assert!(graph.get_node("nobody").is_none());
    }
}
