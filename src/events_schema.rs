use crate::commands_schema::{Value, UpdateOperation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// TODO : timestamp

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "action")]
pub enum Event {
    VALUE_CREATED {id: Uuid, parent: Uuid, data: Value},
    VALUE_UPDATED {id: Uuid, parent: Uuid, data: UpdateOperation}
}
