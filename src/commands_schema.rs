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
#[serde(tag = "action")]
pub enum Command {
    CREATE_VALUE {id: Uuid,  data: Value } ,
    UPDATE_VALUE {id: Uuid, }
}
