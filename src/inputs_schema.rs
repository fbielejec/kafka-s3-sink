use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValueInput {
    pub value: f64,
}

// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub enum OperationTypeInput {
//     ADD,
//     MULTIPLY,
// }

// impl Default for OperationTypeInput {
//     fn default() -> Self { OperationTypeInput::ADD }
// }

// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct OperationInput {
//     pub operation: OperationTypeInput,
//     pub value: f64,
// }
