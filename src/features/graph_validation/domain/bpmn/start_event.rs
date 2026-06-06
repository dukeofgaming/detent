#[derive(Debug, Clone, PartialEq)]
pub struct StartEvent {
    pub id: String,
    pub name: Option<String>,
    pub outgoing: Vec<String>,
    pub documentation: Option<String>,
}
