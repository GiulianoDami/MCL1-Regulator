use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct ProteinInteraction {
    pub source: String,
    pub target: String,
    pub interaction_type: String,
    pub confidence: f64,
}

#[derive(Debug)]
pub struct InteractionNetwork {
    pub nodes: HashSet<String>,
    pub edges: Vec<ProteinInteraction>,
    pub node_attributes: HashMap<String, HashMap<String, String>>,
}

impl InteractionNetwork {
    pub fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            edges: Vec::new(),
            node_attributes: HashMap::new(),
        }
    }

    pub fn add_interaction(&mut self, interaction: ProteinInteraction) {
        self.nodes.insert(interaction.source.clone());
        self.nodes.insert(interaction.target.clone());
        self.edges.push(interaction);
    }

    pub fn add_node_attribute(&mut self, node: &str, key: &str, value: &str) {
        self.node_attributes
            .entry(node.to_string())
            .or_insert_with(HashMap::new)
            .insert(key.to_string(), value.to_string());
    }

    pub fn get_neighbors(&self, node: &str) -> Vec<&String> {
        self.edges
            .iter()
            .filter_map(|edge| {
                if edge.source == *node {
                    Some(&edge.target)
                } else if edge.target == *node {
                    Some(&edge.source)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_degree(&self, node: &str) -> usize {
        self.edges
            .iter()
            .filter(|edge| edge.source == *node || edge.target == *node)
            .count()
    }

    pub fn get_subnetwork(&self, seed_nodes: &[&str]) -> Self {
        let mut subnetwork = InteractionNetwork::new();
        let mut visited = HashSet::new();

        let mut to_visit: Vec<String> = seed_nodes.iter().map(|s| s.to_string()).collect();

        while let Some(node) = to_visit.pop() {
            if !visited.contains(&node) {
                visited.insert(node.clone());

                // Add node attributes if they exist
                if let Some(attributes) = self.node_attributes.get(&node) {
                    for (key, value) in attributes {
                        subnetwork.add_node_attribute(&node, key, value);
                    }
                }

                // Add edges involving this node
                for edge in &self.edges {
                    if edge.source == node || edge.target == node {
                        subnetwork.add_interaction(edge.clone());
                        
                        // Add neighbor to visit list
                        let neighbor = if edge.source == node {
                            &edge.target
                        } else {
                            &edge.source
                        };
                        
                        if !visited.contains(neighbor) {
                            to_visit.push(neighbor.clone());
                        }
                    }
                }
            }
        }

        subnetwork
    }
}