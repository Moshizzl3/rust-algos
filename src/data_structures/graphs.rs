// simple concept of graph

use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub is_seller: bool,
}

#[derive(Debug)]
pub struct MoGraph {
    adjacency_list: HashMap<String, Vec<Profile>>,
}

impl MoGraph {
    pub fn new() -> Self {
        Self {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: String, to: Profile) {
        self.adjacency_list.entry(from).or_default().push(to);
    }

    pub fn neighbors(&self, node: &str) -> Option<&Vec<Profile>> {
        self.adjacency_list.get(node)
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
    pub fn bfs<F>(&self, start: &str, condition: F) -> Option<Profile>
    where
        F: Fn(&Profile) -> bool,
    {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(start.to_string());

        while let Some(person) = queue.pop_front() {
            if visited.contains(&person) {
                continue;
            }
            visited.insert(person.clone());
            if let Some(neighbors) = self.neighbors(&person) {
                for neighbor in neighbors {
                    if condition(neighbor) {
                        return Some(neighbor.clone());
                    }
                    queue.push_back(neighbor.name.clone());
                }
            }
        }
        None
    }
}
