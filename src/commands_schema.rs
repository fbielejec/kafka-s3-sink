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

// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct CreateValue {
//     pub id: Uuid,
//     pub action: ActionType,
//     pub data: Value
// }

// // TODO
// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct UpdateValue {
//     pub fu: String,
//     pub action: ActionType,
// }


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "action")]
pub enum Command {
    // CREATE_VALUE (CreateValue),
    // UPDATE_VALUE (UpdateValue)

    CREATE_VALUE {id: Uuid, data: Value } ,
    UPDATE_VALUE {id: Uuid, }
}
