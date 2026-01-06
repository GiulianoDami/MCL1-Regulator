use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugTarget {
    pub id: String,
    pub name: String,
    pub target_protein: String,
    pub mechanism_of_action: String,
    pub cardiotoxicity_risk: f32,
    pub binding_affinity: f32,
    pub selectivity_score: f32,
}

impl DrugTarget {
    pub fn new(
        id: String,
        name: String,
        target_protein: String,
        mechanism_of_action: String,
        cardiotoxicity_risk: f32,
        binding_affinity: f32,
        selectivity_score: f32,
    ) -> Self {
        DrugTarget {
            id,
            name,
            target_protein,
            mechanism_of_action,
            cardiotoxicity_risk,
            binding_affinity,
            selectivity_score,
        }
    }

    pub fn is_safe_for_cardiac_use(&self) -> bool {
        self.cardiotoxicity_risk < 0.3
    }

    pub fn is_highly_selective(&self) -> bool {
        self.selectivity_score > 0.8
    }
}