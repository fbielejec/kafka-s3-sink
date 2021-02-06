use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ActionType {
    CREATE_VALUE,
    UPDATE_VALUE,
}

// TODO : timestamp

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Value {
    pub id: Uuid,
    pub value: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateValueCommand {
    pub id: Uuid,
    pub action: ActionType,
    pub data: Value //HashMap<String, String>
}
