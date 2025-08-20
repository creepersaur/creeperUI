#[derive(PartialEq, Clone, Copy)]
pub enum ActionType {
    Once,
    EachFrame,
}

impl Default for ActionType {
    fn default() -> Self {
        ActionType::EachFrame
    }
}

impl From<()> for ActionType {
    fn from(value: ()) -> Self {
        ActionType::EachFrame
    }
}

// impl Into<ActionType> for ActionType {
// 	fn into(&self) -> Self {
// 		*self
// 	}
// }
