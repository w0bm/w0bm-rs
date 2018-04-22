#[derive(Debug, Clone, Serialize, Deserialize, Identifiable, Queryable)]
#[primary_key(normalized)]
pub struct Tag {
    normalized: String,
    name: String,
}
