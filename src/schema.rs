use serde::{Deserialize, Serialize};

/// Input schema

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Value {
    pub value: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum OperationType {
    ADD,
    MULTIPLY,
}

impl Default for OperationType {
    fn default() -> Self { OperationType::ADD }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Operation {
    pub operation: OperationType,
    pub value: f64,
}

// TODO commands schema
