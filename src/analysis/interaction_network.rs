rust
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ProteinInteraction {
    pub protein_a: String,
    pub protein_b: String,
    pub interaction_type: String,
    pub confidence_score: f64,
}

#[derive(Debug)]
pub struct InteractionNetwork {
    pub interactions: Vec<ProteinInteraction>,
    pub protein_set: HashSet<String>,
    pub adjacency_map: HashMap<String, Vec<String>>,
}

impl InteractionNetwork {
    pub fn new() -> Self {
        Self {
            interactions: Vec::new(),
            protein_set: HashSet::new(),
            adjacency_map: HashMap::new(),
        }
    }

    pub fn load_from_csv<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut network = Self::new();
        
        for line in reader.lines().skip(1) { // Skip header
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();
            
            if parts.len() < 4 {
                continue;
            }
            
            let interaction = ProteinInteraction {
                protein_a: parts[0].trim().to_string(),
                protein_b: parts[1].trim().to_string(),
                interaction_type: parts[2].trim().to_string(),
                confidence_score: parts[3].trim().parse::<f64>().unwrap_or(0.0),
            };
            
            network.add_interaction(interaction);
        }
        
        Ok(network)
    }

    pub fn add_interaction(&mut self, interaction: ProteinInteraction) {
        self.interactions.push(interaction.clone());
        self.protein_set.insert(interaction.protein_a.clone());
        self.protein_set.insert(interaction.protein_b.clone());
        
        self.adjacency_map
            .entry(interaction.protein_a.clone())
            .or_insert_with(Vec::new)
            .push(interaction.protein_b.clone());
            
        self.adjacency_map
            .entry(interaction.protein_b.clone())
            .or_insert_with(Vec::new)
            .push(interaction.protein_a.clone());
    }

    pub fn get_neighbors(&self, protein: &str) -> Option<&Vec<String>> {
        self.adjacency_map.get(protein)
    }

    pub fn get_protein_count(&self) -> usize {
        self.protein_set.len()
    }

    pub fn get_interaction_count(&self) -> usize {
        self.interactions.len()
    }

    pub fn filter_by_confidence(&self, min_score: f64) -> Vec<ProteinInteraction> {
        self.interactions
            .iter()
            .filter(|interaction| interaction.confidence_score >= min_score)
            .cloned()
            .collect()
    }

    pub fn find_shortest_path(&self, start: &str, end: &str) -> Option<Vec<String>> {
        use std::collections::VecDeque;
        
        if start == end {
            return Some(vec![start.to_string()]);
        }
        
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut predecessors: HashMap<String, String> = HashMap::new();
        
        visited.insert(start.to_string());
        queue.push_back(start.to_string());
        
        while let Some(current) = queue.pop_front() {
            if let Some(neighbors) = self.get_neighbors(&current) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        predecessors.insert(neighbor.clone(), current.clone());
                        queue.push_back(neighbor.clone());
                        
                        if neighbor == end {
                            let mut path = vec![end.to_string()];
                            let mut current_node = end.to_string();
                            
                            while let Some(prev) = predecessors.get(&current_node) {
                                path.push(prev.clone());
                                current_node = prev.clone();
                            }
                            
                            path.reverse();
                            return Some(path);
                        }
                    }
                }
            }
        }
        
        None
    }
}