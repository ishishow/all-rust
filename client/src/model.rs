#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TodoItem {
    pub id: i32,
    pub text: String,
}
