use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::inputs_schema::{ OperationType };

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ActionType {
    CREATE_VALUE,
    UPDATE_VALUE,
}

// TODO : timestamp

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Value {
    pub value_id: Uuid,
    pub value: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateOperation {
    pub value_id: Uuid,
    pub operation: OperationType,
    pub value: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "action")]
pub enum Command {
    CREATE_VALUE { id: Uuid, data: Value },
    UPDATE_VALUE { id: Uuid, data: UpdateOperation }
}
