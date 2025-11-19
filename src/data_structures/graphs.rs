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
        self.nodes.insert(node.name.clone(), node);
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
    /// Returns `Some(Node)` if a node matching the condition is found, `None` otherwise.
    ///
    pub fn bfs<F>(&self, start: &str, condition: &F) -> Option<&Node>
    where
        F: Fn(&Node) -> bool,
    {
        if !self.nodes.contains_key(start) {
            return None;
        }
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

    /// Depth-first search
    ///
    /// # Arguments
    ///
    /// * `start` - The name of the starting node
    /// * `condition` - A predicate function that returns `true` when the desired node is found
    ///
    /// # Returns
    ///
    /// Returns `Some(&Node)` if a node matching the condition is found, `None` otherwise.
    pub fn dfs<F>(&self, start: &str, condition: &F) -> Option<&Node>
    where
        F: Fn(&Node) -> bool,
    {
        let mut visited: HashSet<String> = HashSet::new();
        self.dfs_helper(start, &condition, &mut visited)
    }

    fn dfs_helper<F>(
        &self,
        start: &str,
        condition: &F,
        visited: &mut HashSet<String>,
    ) -> Option<&Node>
    where
        F: Fn(&Node) -> bool,
    {
        if visited.contains(start) {
            return None;
        }
        visited.insert(start.to_string());

        if let Some(node) = self.nodes.get(start)
            && condition(node)
        {
            return Some(node);
        }
        if let Some(neighbors) = self.neighbors(start) {
            for neighbor in neighbors {
                println!("yo neighbor: {:?}", neighbor);
                if let Some(result) = self.dfs_helper(neighbor, condition, visited) {
                    return Some(result);
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

        // Should only have one node (but is replaces)
        assert_eq!(graph.nodes.len(), 1);
        // Original value should be replaces
        assert!(graph.get_node("alice").unwrap().is_seller);
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
        assert!(node.is_seller);
    }

    #[test]
    fn test_get_nonexistent_node() {
        let graph = MoGraph::new();
        assert!(graph.get_node("nobody").is_none());
    }
    // BFS tests
    #[test]
    fn test_bfs_find_seller() {
        let graph = create_test_graph();

        let result = graph.bfs("you", &|node| node.is_seller);

        assert!(result.is_some());
        let seller = result.unwrap();
        assert!(seller.is_seller);
        // Should find thom or jonny (both are sellers)
        assert!(seller.name == "thom" || seller.name == "jonny");
    }

    #[test]
    fn test_bfs_finds_closest_match() {
        let mut graph = MoGraph::new();

        // Create a path: you -> a -> b -> seller1
        //                you -> seller2
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
            is_seller: false,
        });
        graph.add_node(Node {
            name: "seller1".to_string(),
            is_seller: true,
        });
        graph.add_node(Node {
            name: "seller2".to_string(),
            is_seller: true,
        });

        graph.add_edge("you".to_string(), "a".to_string());
        graph.add_edge("a".to_string(), "b".to_string());
        graph.add_edge("b".to_string(), "seller1".to_string());
        graph.add_edge("you".to_string(), "seller2".to_string());

        let result = graph.bfs("you", &|node| node.is_seller);

        // BFS should find seller2 (1 hop) before seller1 (3 hops)
        assert_eq!(result.unwrap().name, "seller2");
    }

    #[test]
    fn test_bfs_start_node_matches() {
        let mut graph = MoGraph::new();
        graph.add_node(Node {
            name: "you".to_string(),
            is_seller: true,
        });

        let result = graph.bfs("you", &|node| node.is_seller);

        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "you");
    }

    #[test]
    fn test_bfs_no_match_found() {
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

        let result = graph.bfs("you", &|node| node.is_seller);

        assert!(result.is_none());
    }

    #[test]
    fn test_bfs_with_custom_condition() {
        let graph = create_test_graph();

        // Find someone named "peggy"
        let result = graph.bfs("you", &|node| node.name == "peggy");

        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "peggy");
    }

    #[test]
    fn test_bfs_with_name_condition() {
        let graph = create_test_graph();

        // Find someone whose name starts with 't'
        let result = graph.bfs("you", &|node| node.name.starts_with('t'));

        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "thom");
    }

    #[test]
    fn test_bfs_isolated_node() {
        let mut graph = MoGraph::new();
        graph.add_node(Node {
            name: "you".to_string(),
            is_seller: false,
        });
        graph.add_node(Node {
            name: "isolated".to_string(),
            is_seller: true,
        });

        // No edges, so "isolated" can't be reached from "you"
        let result = graph.bfs("you", &|node| node.is_seller);

        assert!(result.is_none());
    }

    #[test]
    fn test_bfs_handles_cycles() {
        let mut graph = MoGraph::new();

        // Create a cycle: a -> b -> c -> a
        graph.add_node(Node {
            name: "a".to_string(),
            is_seller: false,
        });
        graph.add_node(Node {
            name: "b".to_string(),
            is_seller: false,
        });
        graph.add_node(Node {
            name: "c".to_string(),
            is_seller: true,
        });

        graph.add_edge("a".to_string(), "b".to_string());
        graph.add_edge("b".to_string(), "c".to_string());
        graph.add_edge("c".to_string(), "a".to_string());

        let result = graph.bfs("a", &|node| node.is_seller);

        // Should find c without infinite loop
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "c");
    }

    #[test]
    fn test_bfs_large_graph() {
        let mut graph = MoGraph::new();

        // Create a larger graph
        for i in 0..100 {
            graph.add_node(Node {
                name: format!("node{}", i),
                is_seller: i == 99, // Only last one is seller
            });
        }

        // Create a chain: node0 -> node1 -> node2 -> ... -> node99
        for i in 0..99 {
            graph.add_edge(format!("node{}", i), format!("node{}", i + 1));
        }

        let result = graph.bfs("node0", &|node| node.is_seller);

        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "node99");
    }

    #[test]
    fn test_bfs_from_nonexistent_start() {
        let graph = create_test_graph();

        let result = graph.bfs("nobody", &|node| node.is_seller);

        // Start node doesn't exist, should return None
        assert!(result.is_none());
    }

    #[test]
    fn test_bfs_multiple_paths_to_same_node() {
        let mut graph = MoGraph::new();

        // Diamond pattern:
        //     you
        //    /   \
        //   a     b
        //    \   /
        //   target
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
            is_seller: false,
        });
        graph.add_node(Node {
            name: "target".to_string(),
            is_seller: true,
        });

        graph.add_edge("you".to_string(), "a".to_string());
        graph.add_edge("you".to_string(), "b".to_string());
        graph.add_edge("a".to_string(), "target".to_string());
        graph.add_edge("b".to_string(), "target".to_string());

        let result = graph.bfs("you", &|node| node.is_seller);

        // Should find target via shortest path (2 hops)
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "target");
    }

    #[test]
    fn test_dfs_find_seller() {
        let graph = create_test_graph();

        let result = graph.dfs("you", &|node| node.is_seller);

        assert!(result.is_some());
        let seller = result.unwrap();
        assert!(seller.is_seller);
    }

    #[test]
    fn test_dfs_finds_first_path_not_shortest() {
        let mut graph = MoGraph::new();

        // Create: you → a → b → c → seller1 (deep path)
        //         you → seller2 (shallow path)
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
            is_seller: false,
        });
        graph.add_node(Node {
            name: "c".to_string(),
            is_seller: false,
        });
        graph.add_node(Node {
            name: "seller1".to_string(),
            is_seller: true,
        });
        graph.add_node(Node {
            name: "seller2".to_string(),
            is_seller: true,
        });

        graph.add_edge("you".to_string(), "a".to_string());
        graph.add_edge("a".to_string(), "b".to_string());
        graph.add_edge("b".to_string(), "c".to_string());
        graph.add_edge("c".to_string(), "seller1".to_string());
        graph.add_edge("you".to_string(), "seller2".to_string());

        let result = graph.dfs("you", &|node| node.is_seller);

        // DFS explores first path deeply, finds seller1 first (not shortest!)
        assert_eq!(result.unwrap().name, "seller1");
    }

    #[test]
    fn test_dfs_start_node_matches() {
        let mut graph = MoGraph::new();
        graph.add_node(Node {
            name: "you".to_string(),
            is_seller: true,
        });

        let result = graph.dfs("you", &|node| node.is_seller);

        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "you");
    }

    #[test]
    fn test_dfs_no_match_found() {
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

        let result = graph.dfs("you", &|node| node.is_seller);

        assert!(result.is_none());
    }

    #[test]
    fn test_dfs_with_custom_condition() {
        let graph = create_test_graph();

        let result = graph.dfs("you", &|node| node.name == "peggy");

        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "peggy");
    }

    #[test]
    fn test_dfs_handles_cycles() {
        let mut graph = MoGraph::new();

        // Create cycle: a → b → c → a
        graph.add_node(Node {
            name: "a".to_string(),
            is_seller: false,
        });
        graph.add_node(Node {
            name: "b".to_string(),
            is_seller: false,
        });
        graph.add_node(Node {
            name: "c".to_string(),
            is_seller: true,
        });

        graph.add_edge("a".to_string(), "b".to_string());
        graph.add_edge("b".to_string(), "c".to_string());
        graph.add_edge("c".to_string(), "a".to_string());

        let result = graph.dfs("a", &|node| node.is_seller);

        // Should find c without infinite loop
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "c");
    }

    #[test]
    fn test_dfs_deep_path() {
        let mut graph = MoGraph::new();

        // Create long chain: a → b → c → d → e → target
        for i in 0..5 {
            graph.add_node(Node {
                name: format!("node{}", i),
                is_seller: false,
            });
        }
        graph.add_node(Node {
            name: "target".to_string(),
            is_seller: true,
        });

        for i in 0..4 {
            graph.add_edge(format!("node{}", i), format!("node{}", i + 1));
        }
        graph.add_edge("node4".to_string(), "target".to_string());

        let result = graph.dfs("node0", &|node| node.is_seller);

        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "target");
    }

    #[test]
    fn test_dfs_from_nonexistent_start() {
        let graph = create_test_graph();

        let result = graph.dfs("nobody", &|node| node.is_seller);

        assert!(result.is_none());
    }

    #[test]
    fn test_dfs_isolated_node() {
        let mut graph = MoGraph::new();
        graph.add_node(Node {
            name: "you".to_string(),
            is_seller: false,
        });
        graph.add_node(Node {
            name: "isolated".to_string(),
            is_seller: true,
        });

        // No edges, isolated can't be reached
        let result = graph.dfs("you", &|node| node.is_seller);

        assert!(result.is_none());
    }

    #[test]
    fn test_dfs_diamond_pattern() {
        let mut graph = MoGraph::new();

        //     you
        //    /   \
        //   a     b
        //    \   /
        //   target
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
            is_seller: false,
        });
        graph.add_node(Node {
            name: "target".to_string(),
            is_seller: true,
        });

        graph.add_edge("you".to_string(), "a".to_string());
        graph.add_edge("you".to_string(), "b".to_string());
        graph.add_edge("a".to_string(), "target".to_string());
        graph.add_edge("b".to_string(), "target".to_string());

        let result = graph.dfs("you", &|node| node.is_seller);

        // DFS explores first path (you → a → target)
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "target");
    }

    #[test]
    fn test_dfs_vs_bfs_difference() {
        let mut graph = MoGraph::new();

        // you → a → b (seller at depth 2)
        // you → c (seller at depth 1)
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
            is_seller: true,
        });

        graph.add_edge("you".to_string(), "a".to_string());
        graph.add_edge("a".to_string(), "b".to_string());
        graph.add_edge("you".to_string(), "c".to_string());

        let dfs_result = graph.dfs("you", &|node| node.is_seller);
        let bfs_result = graph.bfs("you", &|node| node.is_seller);

        // DFS goes deep first → finds b
        assert_eq!(dfs_result.unwrap().name, "b");

        // BFS goes level by level → finds c (closer)
        assert_eq!(bfs_result.unwrap().name, "c");
    }

    #[test]
    fn test_dfs_large_graph() {
        let mut graph = MoGraph::new();

        // Create chain of 100 nodes
        for i in 0..100 {
            graph.add_node(Node {
                name: format!("node{}", i),
                is_seller: i == 99,
            });
        }

        for i in 0..99 {
            graph.add_edge(format!("node{}", i), format!("node{}", i + 1));
        }

        let result = graph.dfs("node0", &|node| node.is_seller);

        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "node99");
    }
}
