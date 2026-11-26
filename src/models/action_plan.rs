use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ActionPlan {
    pub goal: String,
    pub steps: Vec<ActionStep>,
    pub difficulty: Difficulty,
    pub estimited_minutes: u32,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ActionStep {
    pub index: u8,
    pub description: String,
    pub tool_hint: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}
