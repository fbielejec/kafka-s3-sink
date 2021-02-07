use crate::commands_schema::{Value, UpdateOperation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// TODO : timestamp

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "action")]
pub enum Event {
    ValueCreated {id: Uuid, parent: Uuid, data: Value},
    ValueUpdated {id: Uuid, parent: Uuid, data: UpdateOperation}
}
