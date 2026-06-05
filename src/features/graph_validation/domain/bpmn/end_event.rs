#[derive(Debug, Clone, PartialEq)]
pub struct EndEvent {
    pub id: String,
    pub name: Option<String>,
    pub incoming: Vec<String>,
    pub documentation: Option<String>,
}
