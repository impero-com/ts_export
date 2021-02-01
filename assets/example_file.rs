use std::collections::HashSet;

#[derive(Debug, Serialze, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportSerde<'a> {
    pub field_one: u32,
    pub field_two: String,
    pub field_three: Vec<String>,
    pub field_four: [u8; 4],
    pub field_five: HashSet<i32>,
    pub field_six: (u32, String),
    pub field_seven: Option<String>,
    pub field_eight: &'a [u8],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserId(i32);

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPair(i32, i32);

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum ControlResultAssigneeKind {
    #[serde(rename_all = "camelCase")]
    User { user_id: u32 },
    #[serde(rename_all = "camelCase")]
    Pool { user_pool_id: u32 },
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum ControlResultAssigneeOther {
    #[serde(rename_all = "camelCase")]
    User(u32),
    #[serde(rename_all = "camelCase")]
    Pool(u32),
}

#[derive(PartialEq, Eq, Serialize, Debug)]
#[serde(tag = "type")]
pub enum WorkflowStatus {
    #[serde(rename_all = "camelCase")]
    PendingUserCompletion { external_id: u32 },
    #[serde(rename_all = "camelCase")]
    PendingReview { external_id: u32 },
    #[serde(rename_all = "camelCase")]
    Completed,
}

#[derive(PartialEq, Eq, Serialize, Debug)]
#[serde(untagged)]
pub enum Protected<T> {
    Confidential,
    Visible(T),
}

#[derive(PartialEq, Eq, Serialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum AdjacentEnum {
    FirstVariant { id: u32, name: String },
    SecondVariant { id: u32, age: u32 },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExternalEnum {
    A { id: u32 },
    B { id: u32, name: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WithGeneric<T> {
    id: i32,
    name: String,
    value: T,
}

type ArrayOfNumbers = Vec<u32>;
type Array<T> = Vec<T>;
