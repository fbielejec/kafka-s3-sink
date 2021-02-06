use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Inputs schema

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

/// Commands schema

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ActionType {
    CREATE_VALUE,
    UPDATE_VALUE,
}

// TODO : timestamp

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Command {
    pub id: Uuid,
    pub action: ActionType,
    pub data: HashMap<String, String>
}
